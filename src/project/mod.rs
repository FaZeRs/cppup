mod builder;
mod config;
mod validator;

pub use builder::ProjectBuilder;
pub use config::ProjectConfig;
pub use validator::ProjectValidator;

// Keep enums and other public types here
#[derive(Debug, Clone, PartialEq)]
pub enum BuildSystem {
    CMake,
    Make,
}

impl std::fmt::Display for BuildSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BuildSystem::CMake => write!(f, "cmake"),
            BuildSystem::Make => write!(f, "make"),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub enum License {
    MIT,
    Apache2,
    GPL3,
    BSD3,
}

impl std::fmt::Display for License {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            License::MIT => write!(f, "MIT"),
            License::Apache2 => write!(f, "Apache-2.0"),
            License::GPL3 => write!(f, "GPL-3.0"),
            License::BSD3 => write!(f, "BSD-3-Clause"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PackageManager {
    Conan,
    Vcpkg,
    None,
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PackageManager::Conan => write!(f, "conan"),
            PackageManager::Vcpkg => write!(f, "vcpkg"),
            PackageManager::None => write!(f, "none"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct QualityConfig {
    pub enable_clang_tidy: bool,
    pub enable_cppcheck: bool,
    pub enable_include_what_you_use: bool,
}

impl QualityConfig {
    pub fn new(tools: &[&str]) -> Self {
        Self {
            enable_clang_tidy: tools.contains(&"clang-tidy"),
            enable_cppcheck: tools.contains(&"cppcheck"),
            enable_include_what_you_use: tools.contains(&"include-what-you-use"),
        }
    }
}

impl std::fmt::Display for QualityConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut tools = Vec::new();

        if self.enable_clang_tidy {
            tools.push("clang-tidy");
        }
        if self.enable_cppcheck {
            tools.push("cppcheck");
        }
        if self.enable_include_what_you_use {
            tools.push("include-what-you-use");
        }

        write!(f, "{}", tools.join(", "))
    }
}

#[derive(Debug, Clone)]
pub struct CodeFormatter {
    pub enable_clang_format: bool,
    pub enable_cmake_format: bool,
}

impl CodeFormatter {
    pub fn new(tools: &[&str]) -> Self {
        Self {
            enable_clang_format: tools.contains(&"clang-format"),
            enable_cmake_format: tools.contains(&"cmake-format"),
        }
    }
}

impl std::fmt::Display for CodeFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut tools = Vec::new();

        if self.enable_clang_format {
            tools.push("clang-format");
        }
        if self.enable_cmake_format {
            tools.push("cmake-format");
        }

        write!(f, "{}", tools.join(", "))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestFramework {
    Doctest,
    GTest,
    Catch2,
    BoostTest,
    None,
}

impl std::fmt::Display for TestFramework {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TestFramework::Doctest => write!(f, "doctest"),
            TestFramework::GTest => write!(f, "gtest"),
            TestFramework::Catch2 => write!(f, "catch2"),
            TestFramework::BoostTest => write!(f, "boost"),
            TestFramework::None => write!(f, "none"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_system_display() {
        assert_eq!(BuildSystem::CMake.to_string(), "cmake");
        assert_eq!(BuildSystem::Make.to_string(), "make");
    }

    #[test]
    fn test_license_display() {
        assert_eq!(License::MIT.to_string(), "MIT");
        assert_eq!(License::Apache2.to_string(), "Apache-2.0");
        assert_eq!(License::GPL3.to_string(), "GPL-3.0");
        assert_eq!(License::BSD3.to_string(), "BSD-3-Clause");
    }

    #[test]
    fn test_package_manager_display() {
        assert_eq!(PackageManager::Conan.to_string(), "conan");
        assert_eq!(PackageManager::Vcpkg.to_string(), "vcpkg");
        assert_eq!(PackageManager::None.to_string(), "none");
    }

    #[test]
    fn test_quality_config_new() {
        let config = QualityConfig::new(&["clang-tidy", "cppcheck"]);
        assert!(config.enable_clang_tidy);
        assert!(config.enable_cppcheck);
        assert!(!config.enable_include_what_you_use);

        let empty_config = QualityConfig::new(&[]);
        assert!(!empty_config.enable_clang_tidy);
        assert!(!empty_config.enable_cppcheck);
        assert!(!empty_config.enable_include_what_you_use);

        let all_config = QualityConfig::new(&["clang-tidy", "cppcheck", "include-what-you-use"]);
        assert!(all_config.enable_clang_tidy);
        assert!(all_config.enable_cppcheck);
        assert!(all_config.enable_include_what_you_use);
    }

    #[test]
    fn test_quality_config_display() {
        let config = QualityConfig::new(&["clang-tidy", "cppcheck"]);
        assert_eq!(config.to_string(), "clang-tidy, cppcheck");

        let empty_config = QualityConfig::new(&[]);
        assert_eq!(empty_config.to_string(), "");

        let single_config = QualityConfig::new(&["cppcheck"]);
        assert_eq!(single_config.to_string(), "cppcheck");
    }

    #[test]
    fn test_code_formatter_new() {
        let formatter = CodeFormatter::new(&["clang-format"]);
        assert!(formatter.enable_clang_format);
        assert!(!formatter.enable_cmake_format);

        let empty_formatter = CodeFormatter::new(&[]);
        assert!(!empty_formatter.enable_clang_format);
        assert!(!empty_formatter.enable_cmake_format);

        let all_formatter = CodeFormatter::new(&["clang-format", "cmake-format"]);
        assert!(all_formatter.enable_clang_format);
        assert!(all_formatter.enable_cmake_format);
    }

    #[test]
    fn test_code_formatter_display() {
        let formatter = CodeFormatter::new(&["clang-format", "cmake-format"]);
        assert_eq!(formatter.to_string(), "clang-format, cmake-format");

        let empty_formatter = CodeFormatter::new(&[]);
        assert_eq!(empty_formatter.to_string(), "");

        let single_formatter = CodeFormatter::new(&["cmake-format"]);
        assert_eq!(single_formatter.to_string(), "cmake-format");
    }

    #[test]
    fn test_test_framework_display() {
        assert_eq!(TestFramework::Doctest.to_string(), "doctest");
        assert_eq!(TestFramework::GTest.to_string(), "gtest");
        assert_eq!(TestFramework::Catch2.to_string(), "catch2");
        assert_eq!(TestFramework::BoostTest.to_string(), "boost");
        assert_eq!(TestFramework::None.to_string(), "none");
    }
}
