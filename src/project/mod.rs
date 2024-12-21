mod builder;
mod config;
mod validator;

pub use builder::ProjectBuilder;
pub use config::ProjectConfig;
pub use validator::ProjectValidator;

// Keep enums and other public types here
#[derive(Debug, Clone)]
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
    pub enable_clang_format: bool,
}

impl QualityConfig {
    pub fn new(tools: &[&str]) -> Self {
        Self {
            enable_clang_tidy: tools.contains(&"clang-tidy"),
            enable_cppcheck: tools.contains(&"cppcheck"),
            enable_clang_format: tools.contains(&"clang-format"),
        }
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
