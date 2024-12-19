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
        if quality_config.enable_clang_format {
            tools.push("clang-format");
        }
        if quality_config.enable_clang_tidy {
            tools.push("clang-tidy");
        }
        if quality_config.enable_cppcheck {
            tools.push("cppcheck");
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
