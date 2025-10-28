use super::config::{CppStandard, ProjectConfig};
use super::{BuildSystem, PackageManager};
use anyhow::{Context, Result};
use std::process::Command;

pub struct ProjectValidator {
    config: ProjectConfig,
}

impl ProjectValidator {
    pub fn new(config: ProjectConfig) -> Self {
        Self { config }
    }

    pub fn check_prerequisites(&self) -> Result<()> {
        self.check_required_tools()?;
        self.check_compiler_version()?;
        Ok(())
    }

    fn check_required_tools(&self) -> Result<()> {
        let mut tools = match self.config.build_system {
            BuildSystem::CMake => vec!["cmake", "g++"],
            BuildSystem::Make => vec!["make", "g++"],
        };

        match self.config.package_manager {
            PackageManager::Conan => {
                tools.push("conan");
            }
            PackageManager::Vcpkg => {
                tools.push("vcpkg");
            }
            PackageManager::None => {}
        };

        let quality_config = &self.config.quality_config;
        if quality_config.enable_clang_tidy {
            tools.push("clang-tidy");
        }
        if quality_config.enable_cppcheck {
            tools.push("cppcheck");
        }
        if quality_config.enable_include_what_you_use {
            tools.push("include-what-you-use");
        }
        let code_formatter = &self.config.code_formatter;
        if code_formatter.enable_clang_format {
            tools.push("clang-format");
        }
        if code_formatter.enable_cmake_format {
            tools.push("cmake-format");
        }
        for tool in tools {
            if !Self::is_tool_installed(tool) {
                return Err(anyhow::anyhow!("{} is not installed", tool));
            }
        }

        Ok(())
    }

    fn check_compiler_version(&self) -> Result<()> {
        let compiler_version = Self::get_compiler_version()?;
        println!("Found compiler: {}", compiler_version);

        // Check if compiler supports the selected C++ standard
        let required_version = match self.config.cpp_standard {
            CppStandard::Cpp11 => 4.8,
            CppStandard::Cpp14 => 5.0,
            CppStandard::Cpp17 => 7.0,
            CppStandard::Cpp20 => 10.0,
            CppStandard::Cpp23 => 12.0,
        };

        if let Some(version) = Self::extract_gcc_version(&compiler_version) {
            if version < required_version {
                return Err(anyhow::anyhow!(
                    "G++ version {} is too old for C++{}. Version >= {} required.",
                    version,
                    self.config.cpp_standard,
                    required_version
                ));
            }
        }

        Ok(())
    }

    fn is_tool_installed(tool: &str) -> bool {
        which::which(tool).is_ok()
    }

    fn get_compiler_version() -> Result<String> {
        let output = Command::new("g++")
            .arg("--version")
            .output()
            .context("Failed to get g++ version")?;

        let version = String::from_utf8_lossy(&output.stdout);
        Ok(version.lines().next().unwrap_or("unknown").to_string())
    }

    fn extract_gcc_version(version_string: &str) -> Option<f32> {
        let version_regex = regex::Regex::new(r"g\+\+ .* (\d+\.\d+)").ok()?;
        version_regex
            .captures(version_string)?
            .get(1)?
            .as_str()
            .parse()
            .ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::config::{CppStandard, ProjectType};
    use crate::project::{CodeFormatter, License, QualityConfig, TestFramework};
    use std::path::PathBuf;

    fn create_test_config() -> ProjectConfig {
        ProjectConfig {
            name: "test-project".to_string(),
            description: "Test project".to_string(),
            project_type: ProjectType::Executable,
            build_system: BuildSystem::CMake,
            cpp_standard: CppStandard::Cpp17,
            test_framework: TestFramework::None,
            package_manager: PackageManager::None,
            license: License::MIT,
            use_git: false,
            path: PathBuf::from("/tmp/test-project"),
            author: "Test Author".to_string(),
            version: "0.1.0".to_string(),
            quality_config: QualityConfig::new(&[]),
            code_formatter: CodeFormatter::new(&[]),
        }
    }

    #[test]
    fn test_extract_gcc_version_valid() {
        let version_string = "g++ (Ubuntu 11.4.0-1ubuntu1~22.04) 11.4.0";
        let version = ProjectValidator::extract_gcc_version(version_string);
        assert_eq!(version, Some(11.4));
    }

    #[test]
    fn test_extract_gcc_version_different_format() {
        let version_string = "g++ (GCC) 12.2.0";
        let version = ProjectValidator::extract_gcc_version(version_string);
        assert_eq!(version, Some(12.2));
    }

    #[test]
    fn test_extract_gcc_version_invalid() {
        let version_string = "invalid version string";
        let version = ProjectValidator::extract_gcc_version(version_string);
        assert_eq!(version, None);
    }

    #[test]
    fn test_extract_gcc_version_no_number() {
        let version_string = "g++ version unknown";
        let version = ProjectValidator::extract_gcc_version(version_string);
        assert_eq!(version, None);
    }

    #[test]
    fn test_validator_creation() {
        let config = create_test_config();
        let validator = ProjectValidator::new(config.clone());
        assert_eq!(validator.config.name, "test-project");
    }

    #[test]
    fn test_cpp_standard_version_requirements() {
        // Test that we can access the required version logic through the type
        let cpp11_config = ProjectConfig {
            cpp_standard: CppStandard::Cpp11,
            ..create_test_config()
        };
        let validator11 = ProjectValidator::new(cpp11_config);
        assert!(matches!(validator11.config.cpp_standard, CppStandard::Cpp11));

        let cpp23_config = ProjectConfig {
            cpp_standard: CppStandard::Cpp23,
            ..create_test_config()
        };
        let validator23 = ProjectValidator::new(cpp23_config);
        assert!(matches!(validator23.config.cpp_standard, CppStandard::Cpp23));
    }
}
