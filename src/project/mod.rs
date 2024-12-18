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
            BuildSystem::CMake => write!(f, "CMake"),
            BuildSystem::Make => write!(f, "Make"),
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
            PackageManager::Conan => write!(f, "Conan"),
            PackageManager::Vcpkg => write!(f, "Vcpkg"),
            PackageManager::None => write!(f, "None"),
        }
    }
}
