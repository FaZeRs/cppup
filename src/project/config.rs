use super::{BuildSystem, CodeFormatter, License, PackageManager, QualityConfig, TestFramework};
use crate::cli::Cli;
use anyhow::{Context, Result};
use inquire::validator::Validation;
use inquire::{Confirm, MultiSelect, Select, Text};
use std::fs;
use std::path::PathBuf;

const DEFAULT_VERSION: &str = "0.1.0";
const DEFAULT_DESCRIPTION: &str = "A C++ project generated with cppup";

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub project_type: ProjectType,
    pub build_system: BuildSystem,
    pub cpp_standard: CppStandard,
    pub test_framework: TestFramework,
    pub package_manager: PackageManager,
    pub license: License,
    pub use_git: bool,
    pub path: PathBuf,
    pub author: String,
    pub version: String,
    pub quality_config: QualityConfig,
    pub code_formatter: CodeFormatter,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectType {
    Executable,
    Library,
}

impl std::fmt::Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProjectType::Executable => write!(f, "executable"),
            ProjectType::Library => write!(f, "library"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CppStandard {
    Cpp11,
    Cpp14,
    Cpp17,
    Cpp20,
    Cpp23,
}

impl std::fmt::Display for CppStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CppStandard::Cpp11 => write!(f, "11"),
            CppStandard::Cpp14 => write!(f, "14"),
            CppStandard::Cpp17 => write!(f, "17"),
            CppStandard::Cpp20 => write!(f, "20"),
            CppStandard::Cpp23 => write!(f, "23"),
        }
    }
}

// Validation functions
fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Project name cannot be empty"));
    }
    if name.len() > 100 {
        return Err(anyhow::anyhow!("Project name is too long"));
    }
    if name.starts_with(|c: char| c.is_numeric()) {
        return Err(anyhow::anyhow!("Project name cannot start with a number"));
    }
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(anyhow::anyhow!(
            "Project name can only contain alphanumeric characters, '-' and '_'"
        ));
    }
    Ok(())
}

fn validate_project_path(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "Directory doesn't exist: {}",
            path.display()
        ));
    }
    if !path.is_dir() {
        return Err(anyhow::anyhow!(
            "Path is not a directory: {}",
            path.display()
        ));
    }
    // Check if we have write permissions
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.permissions().readonly() {
                return Err(anyhow::anyhow!(
                    "Directory is read-only: {}",
                    path.display()
                ));
            }
        }
        Err(_) => {
            return Err(anyhow::anyhow!(
                "Cannot access directory: {}",
                path.display()
            ))
        }
    }
    Ok(())
}

fn create_config_from_cli(cli: &Cli) -> Result<ProjectConfig> {
    let name = cli
        .name
        .clone()
        .context("Project name is required in non-interactive mode")?;

    // Validate project name
    validate_project_name(&name)?;

    // Validate project path
    validate_project_path(&cli.path)?;

    let description = cli
        .description
        .clone()
        .unwrap_or(DEFAULT_DESCRIPTION.to_string());

    let default_author = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME")) // Try Windows username
        .or_else(|_| Ok::<String, std::env::VarError>("Unknown".to_string()))
        .unwrap();
    let author = cli.author.clone().unwrap_or(default_author);

    let project_type = match cli.project_type.as_deref() {
        Some("executable") => ProjectType::Executable,
        Some("library") => ProjectType::Library,
        _ => {
            return Err(anyhow::anyhow!(
                "Project type is required in non-interactive mode"
            ))
        }
    };

    let build_system = match cli.build_system.as_str() {
        "cmake" => BuildSystem::CMake,
        "make" => BuildSystem::Make,
        _ => BuildSystem::CMake,
    };

    let cpp_standard = match cli.cpp_standard.as_str() {
        "11" => CppStandard::Cpp11,
        "14" => CppStandard::Cpp14,
        "17" => CppStandard::Cpp17,
        "20" => CppStandard::Cpp20,
        "23" => CppStandard::Cpp23,
        _ => CppStandard::Cpp17,
    };

    let path = cli.path.join(&name);

    // Check if project directory already exists
    if path.exists() {
        return Err(anyhow::anyhow!(
            "Project directory already exists: {}",
            path.display()
        ));
    }

    let package_manager = match cli.package_manager.as_str() {
        "conan" => PackageManager::Conan,
        "vcpkg" => PackageManager::Vcpkg,
        _ => PackageManager::None,
    };

    let license = match cli.license.as_str() {
        "MIT" => License::MIT,
        "Apache-2.0" => License::Apache2,
        "GPL-3.0" => License::GPL3,
        "BSD-3-Clause" => License::BSD3,
        _ => unreachable!(),
    };

    let quality_config = QualityConfig::new(
        &cli.quality_tools
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>(),
    );

    let code_formatter = CodeFormatter::new(
        &cli.code_formatter
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>(),
    );

    let test_framework = match cli.test_framework.as_str() {
        "doctest" => TestFramework::Doctest,
        "gtest" => TestFramework::GTest,
        "catch2" => TestFramework::Catch2,
        "boosttest" => TestFramework::BoostTest,
        "none" => TestFramework::None,
        _ => unreachable!(),
    };

    Ok(ProjectConfig {
        name,
        project_type,
        build_system,
        cpp_standard,
        use_git: cli.git,
        path,
        test_framework,
        package_manager,
        license,
        description,
        author,
        version: DEFAULT_VERSION.to_string(),
        quality_config,
        code_formatter,
    })
}

impl ProjectConfig {
    pub fn new(defaults: Option<&Cli>) -> Result<Self> {
        if let Some(default) = defaults {
            if default.non_interactive {
                return create_config_from_cli(default);
            }
        }

        let name = Text::new("What is your project name?")
            .with_default(
                defaults
                    .and_then(|d| d.name.as_deref())
                    .unwrap_or("my-cpp-project"),
            )
            .with_help_message("The name of your project (will be used as directory name)")
            .with_validator(|input: &str| match validate_project_name(input) {
                Ok(()) => Ok(Validation::Valid),
                Err(e) => Ok(Validation::Invalid(e.to_string().into())),
            })
            .prompt()?;

        let description = Text::new("Project description:")
            .with_default(
                defaults
                    .and_then(|d| d.description.as_deref())
                    .unwrap_or(DEFAULT_DESCRIPTION),
            )
            .prompt()?;

        let default_author = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME")) // Try Windows username
            .or_else(|_| Ok::<String, std::env::VarError>("Unknown".to_string()))
            .unwrap();
        let author = Text::new("Author:")
            .with_default(
                defaults
                    .and_then(|d| d.author.as_deref())
                    .unwrap_or(&default_author),
            )
            .prompt()?;

        // Add validation for project path
        let path = Text::new("Where do you want to create the project?")
            .with_default(
                defaults
                    .map(|d| d.path.to_string_lossy().to_string())
                    .as_deref()
                    .unwrap_or("."),
            )
            .with_validator(|input: &str| {
                let path = PathBuf::from(input);
                match validate_project_path(&path) {
                    Ok(()) => Ok(Validation::Valid),
                    Err(e) => Ok(Validation::Invalid(e.to_string().into())),
                }
            })
            .prompt()?;

        let project_path = PathBuf::from(&path).join(&name);

        // Check if project directory already exists
        if project_path.exists() {
            return Err(anyhow::anyhow!(
                "Project directory already exists: {}",
                project_path.display()
            ));
        }

        // Get project type
        let project_type = Select::new(
            "What type of project do you want to create?",
            vec![
                "Basic (Simple executable)",
                "Library (Static/Dynamic library)",
            ],
        )
        .prompt()?;

        let project_type = match project_type {
            "Basic (Simple executable)" => ProjectType::Executable,
            "Library (Static/Dynamic library)" => ProjectType::Library,
            _ => unreachable!(),
        };

        // Choose build system
        let build_system = Select::new(
            "Which build system do you want to use?",
            vec!["CMake", "Make"],
        )
        .with_help_message("CMake is recommended for complex projects")
        .prompt()?;

        let build_system = match build_system {
            "CMake" => BuildSystem::CMake,
            "Make" => BuildSystem::Make,
            _ => unreachable!(),
        };

        // Choose C++ standard
        let cpp_standard = Select::new(
            "Which C++ standard do you want to use?",
            vec!["C++11", "C++14", "C++17", "C++20", "C++23"],
        )
        .prompt()?;

        let cpp_standard = match cpp_standard {
            "C++11" => CppStandard::Cpp11,
            "C++14" => CppStandard::Cpp14,
            "C++17" => CppStandard::Cpp17,
            "C++20" => CppStandard::Cpp20,
            "C++23" => CppStandard::Cpp23,
            _ => unreachable!(),
        };

        let package_manager = Select::new(
            "Which package manager would you like to use?",
            vec!["None", "Conan", "Vcpkg"],
        )
        .with_help_message("Package managers help manage external dependencies")
        .prompt()?;

        let package_manager = match package_manager {
            "None" => PackageManager::None,
            "Conan" => PackageManager::Conan,
            "Vcpkg" => PackageManager::Vcpkg,
            _ => unreachable!(),
        };

        let test_framework = Select::new(
            "Select testing framework:",
            vec![
                TestFramework::None,
                TestFramework::Doctest,
                TestFramework::GTest,
                TestFramework::Catch2,
                TestFramework::BoostTest,
            ],
        )
        .prompt()?;

        // Git initialization
        let use_git = Confirm::new("Do you want to initialize git repository?")
            .with_default(true)
            .prompt()?;

        let license = Select::new(
            "Which license do you want to use?",
            vec!["MIT", "Apache-2.0", "GPL-3.0", "BSD-3-Clause"],
        )
        .prompt()?;

        let license = match license {
            "MIT" => License::MIT,
            "Apache-2.0" => License::Apache2,
            "GPL-3.0" => License::GPL3,
            "BSD-3-Clause" => License::BSD3,
            _ => unreachable!(),
        };

        let quality_config = if Confirm::new("Do you want to set up code quality tools?")
            .with_default(true)
            .prompt()?
        {
            let tools = MultiSelect::new(
                "Which code quality tools would you like to use?",
                vec![
                    "clang-tidy (Static analysis)",
                    "cppcheck (Static analysis)",
                    "include-what-you-use (Static analysis)",
                ],
            )
            .with_help_message("Use space to select/deselect, enter to confirm")
            .with_default(&[0])
            .prompt()?;

            let selected_tools: Vec<&str> = tools
                .iter()
                .map(|t| match *t {
                    "clang-tidy (Static analysis)" => "clang-tidy",
                    "cppcheck (Static analysis)" => "cppcheck",
                    "include-what-you-use (Static analysis)" => "include-what-you-use",
                    _ => unreachable!(),
                })
                .collect();
            QualityConfig::new(&selected_tools)
        } else {
            QualityConfig::new(&[])
        };

        let code_formatter = if Confirm::new("Do you want to set up code formatter?")
            .with_default(true)
            .prompt()?
        {
            let tools = MultiSelect::new(
                "Which code formatter would you like to use?",
                vec![
                    "clang-format (Code formatting)",
                    "cmake-format (Code formatting)",
                ],
            )
            .with_help_message("Use space to select/deselect, enter to confirm")
            .with_default(&[0])
            .prompt()?;

            let selected_tools: Vec<&str> = tools
                .iter()
                .map(|t| match *t {
                    "clang-format (Code formatting)" => "clang-format",
                    "cmake-format (Code formatting)" => "cmake-format",
                    _ => unreachable!(),
                })
                .collect();
            CodeFormatter::new(&selected_tools)
        } else {
            CodeFormatter::new(&[])
        };

        Ok(ProjectConfig {
            name,
            project_type,
            build_system,
            cpp_standard,
            use_git,
            path: project_path,
            package_manager,
            license,
            author,
            description,
            version: DEFAULT_VERSION.to_string(),
            quality_config,
            code_formatter,
            test_framework,
        })
    }
}
