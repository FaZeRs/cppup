use crate::cli::Cli;
use crate::templates::{create_template_registry, ProjectTemplateData};
use anyhow::{Context, Result};
use inquire::validator::Validation;
use inquire::{Confirm, Select, Text};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub struct ProjectConfig {
    pub name: String,
    pub project_type: ProjectType,
    pub build_system: BuildSystem,
    pub cpp_standard: CppStandard,
    pub use_git: bool,
    pub path: PathBuf,
    pub enable_tests: bool,
}

#[derive(Debug)]
pub enum ProjectType {
    Executable,
    Library,
}

#[derive(Debug)]
pub enum BuildSystem {
    CMake,
    Make,
}

#[derive(Debug)]
pub enum CppStandard {
    Cpp11,
    Cpp14,
    Cpp17,
    Cpp20,
    Cpp23,
}

impl std::fmt::Display for BuildSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BuildSystem::CMake => write!(f, "CMake"),
            BuildSystem::Make => write!(f, "Make"),
        }
    }
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

impl ProjectConfig {
    pub fn new(defaults: Option<&Cli>) -> Result<Self> {
        // Add validation for existing project directory
        let name = Text::new("What is your project name?")
            .with_default(
                defaults
                    .and_then(|d| d.name.as_deref())
                    .unwrap_or("my-cpp-project"),
            )
            .with_help_message("The name of your project (will be used as directory name)")
            .with_validator(|input: &str| {
                // Improved validation
                if input.is_empty() {
                    return Ok(Validation::Invalid("Project name cannot be empty".into()));
                }
                if input.len() > 100 {
                    return Ok(Validation::Invalid("Project name is too long".into()));
                }
                if input.starts_with(|c: char| c.is_numeric()) {
                    return Ok(Validation::Invalid(
                        "Project name cannot start with a number".into(),
                    ));
                }
                if input
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
                {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid(
                        "Project name can only contain alphanumeric characters, '-' and '_'".into(),
                    ))
                }
            })
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
                if !path.exists() {
                    return Ok(Validation::Invalid("Directory doesn't exist".into()));
                }
                if !path.is_dir() {
                    return Ok(Validation::Invalid("Path is not a directory".into()));
                }
                // Check if we have write permissions
                match fs::metadata(&path) {
                    Ok(metadata) => {
                        if metadata.permissions().readonly() {
                            return Ok(Validation::Invalid("Directory is read-only".into()));
                        }
                    }
                    Err(_) => return Ok(Validation::Invalid("Cannot access directory".into())),
                }
                Ok(Validation::Valid)
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

        // Git initialization
        let use_git = Confirm::new("Do you want to initialize git repository?")
            .with_default(true)
            .prompt()?;

        let enable_tests = Confirm::new("Do you want to include unit tests?")
            .with_default(true)
            .prompt()?;

        Ok(ProjectConfig {
            name,
            project_type,
            build_system,
            cpp_standard,
            use_git,
            path: project_path,
            enable_tests,
        })
    }

    pub fn create_directory_structure(&self) -> Result<()> {
        // Create main project directory
        fs::create_dir_all(&self.path)
            .with_context(|| format!("Failed to create project directory at {:?}", self.path))?;

        // Create standard directories
        let mut dirs = vec![
            "src",
            "include",
            match self.project_type {
                ProjectType::Library => "examples",
                ProjectType::Executable => "assets",
            },
        ];

        if self.enable_tests {
            dirs.push("tests");
        }

        for dir in dirs {
            fs::create_dir_all(self.path.join(dir))
                .with_context(|| format!("Failed to create {} directory", dir))?;
        }

        Ok(())
    }

    pub fn generate_cmake_file(&self) -> Result<()> {
        self.render_template("CMakeLists.txt", &self.path.join("CMakeLists.txt"))?;

        Ok(())
    }

    pub fn generate_makefile(&self) -> Result<()> {
        self.render_template("Makefile", &self.path.join("Makefile"))?;

        Ok(())
    }

    pub fn generate_source_files(&self) -> Result<()> {
        match self.project_type {
            ProjectType::Executable => {
                self.render_template("main.cpp", &self.path.join("src/main.cpp"))?;
            }
            ProjectType::Library => {
                self.render_template(
                    "header.hpp",
                    &self.path.join(format!("include/{}.hpp", self.name)),
                )?;
                self.render_template("library.cpp", &self.path.join("src/lib.cpp"))?;
                self.render_template("example.cpp", &self.path.join("examples/example.cpp"))?;
            }
        }

        Ok(())
    }

    pub fn generate_test_files(&self) -> Result<()> {
        if self.enable_tests {
            self.render_template("main_test.cpp", &self.path.join("tests/main_test.cpp"))?;
        }
        Ok(())
    }

    pub fn initialize_git(&self) -> Result<()> {
        if self.use_git {
            Command::new("git")
                .arg("init")
                .current_dir(&self.path)
                .output()
                .context("Failed to initialize git repository")?;

            self.render_template("gitignore", &self.path.join(".gitignore"))?;
        }
        Ok(())
    }

    pub fn generate_readme(&self) -> Result<()> {
        self.render_template("README.md", &self.path.join("README.md"))?;

        Ok(())
    }

    pub fn check_prerequisites(&self) -> Result<()> {
        // Check if required tools are installed
        let tools = match self.build_system {
            BuildSystem::CMake => vec!["cmake", "g++"],
            BuildSystem::Make => vec!["make", "g++"],
        };

        for tool in tools {
            if !Self::is_tool_installed(tool) {
                return Err(anyhow::anyhow!("{} is not installed", tool));
            }
        }

        // Check compiler version
        let compiler_version = Self::get_compiler_version()?;
        println!("Found compiler: {}", compiler_version);

        // Check if compiler supports the selected C++ standard
        let required_version = match self.cpp_standard {
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
                    self.cpp_standard,
                    required_version
                ));
            }
        }

        Ok(())
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

    fn is_tool_installed(tool: &str) -> bool {
        which::which(tool).is_ok()
    }

    // Add this helper method for version checks
    pub fn get_compiler_version() -> Result<String> {
        let output = Command::new("g++")
            .arg("--version")
            .output()
            .context("Failed to get g++ version")?;

        let version = String::from_utf8_lossy(&output.stdout);
        Ok(version.lines().next().unwrap_or("unknown").to_string())
    }

    pub fn generate_clang_format(&self) -> Result<()> {
        self.render_template("clang-format", &self.path.join(".clang-format"))?;

        Ok(())
    }

    fn create_template_data(&self) -> ProjectTemplateData {
        ProjectTemplateData {
            name: self.name.clone(),
            cpp_standard: self.cpp_standard.to_string(),
            is_library: matches!(self.project_type, ProjectType::Library),
            namespace: self.name.replace('-', "_"),
            build_system: self.build_system.to_string(),
            description: Some("A C++ project generated with cppup".to_string()),
            author: std::env::var("USER").ok(),
            version: "0.1.0".to_string(),
            license: Some("MIT".to_string()),
            enable_tests: self.enable_tests,
        }
    }

    fn render_template(&self, template_name: &str, path: &PathBuf) -> Result<()> {
        let handlebars = create_template_registry();
        let template_data = self.create_template_data();

        let rendered = handlebars
            .render(template_name, &template_data)
            .with_context(|| format!("Failed to render template {}", template_name))?;

        fs::write(path, rendered)
            .with_context(|| format!("Failed to write file {}", path.display()))?;

        Ok(())
    }
}
