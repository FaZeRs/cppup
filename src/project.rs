use crate::cli::Cli;
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

        Ok(ProjectConfig {
            name,
            project_type,
            build_system,
            cpp_standard,
            use_git,
            path: project_path,
        })
    }

    pub fn create_directory_structure(&self) -> Result<()> {
        // Create main project directory
        fs::create_dir_all(&self.path)
            .with_context(|| format!("Failed to create project directory at {:?}", self.path))?;

        // Create standard directories
        let dirs = vec![
            "src",
            "include",
            "build",
            "tests",
            match self.project_type {
                ProjectType::Library => "examples",
                ProjectType::Executable => "assets",
            },
        ];

        for dir in dirs {
            fs::create_dir_all(self.path.join(dir))
                .with_context(|| format!("Failed to create {} directory", dir))?;
        }

        Ok(())
    }

    pub fn generate_cmake_file(&self) -> Result<()> {
        let cmake_version = "3.15";
        let cpp_standard = self.cpp_standard.to_string();

        let cmake_content = match self.project_type {
            ProjectType::Executable => format!(
                r#"cmake_minimum_required(VERSION {})
project({} LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD {})
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Add compiler warnings
if(MSVC)
    add_compile_options(/W4)
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()

# Enable testing
enable_testing()

# Main executable
add_executable(${{PROJECT_NAME}} src/main.cpp)
target_include_directories(${{PROJECT_NAME}} PRIVATE include)

# Tests
add_executable(${{PROJECT_NAME}}_tests tests/main_test.cpp)
target_include_directories(${{PROJECT_NAME}}_tests PRIVATE include)
add_test(NAME ${{PROJECT_NAME}}_tests COMMAND ${{PROJECT_NAME}}_tests)
"#,
                cmake_version, self.name, cpp_standard
            ),
            ProjectType::Library => format!(
                r#"cmake_minimum_required(VERSION {})
project({} LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD {})
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Add compiler warnings
if(MSVC)
    add_compile_options(/W4)
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()

# Enable testing
enable_testing()

# Library
add_library(${{PROJECT_NAME}} STATIC
    src/lib.cpp
)
target_include_directories(${{PROJECT_NAME}} PUBLIC include)

# Example executable
add_executable(${{PROJECT_NAME}}_example examples/example.cpp)
target_link_libraries(${{PROJECT_NAME}}_example PRIVATE ${{PROJECT_NAME}})

# Tests
add_executable(${{PROJECT_NAME}}_tests tests/lib_test.cpp)
target_link_libraries(${{PROJECT_NAME}}_tests PRIVATE ${{PROJECT_NAME}})
add_test(NAME ${{PROJECT_NAME}}_tests COMMAND ${{PROJECT_NAME}}_tests)
"#,
                cmake_version, self.name, cpp_standard
            ),
        };

        fs::write(self.path.join("CMakeLists.txt"), cmake_content)
            .context("Failed to write CMakeLists.txt")?;

        Ok(())
    }

    pub fn generate_makefile(&self) -> Result<()> {
        let cpp_standard = match self.cpp_standard {
            CppStandard::Cpp11 => "11",
            CppStandard::Cpp14 => "14",
            CppStandard::Cpp17 => "17",
            CppStandard::Cpp20 => "20",
            CppStandard::Cpp23 => "23",
        };

        let makefile_content = match self.project_type {
            ProjectType::Executable => format!(
                r#"CXX = g++
CXXFLAGS = -std=c++{} -Wall -Wextra -I include
TARGET = {}
BUILD_DIR = build

SRCS = $(wildcard src/*.cpp)
OBJS = $(SRCS:%.cpp=$(BUILD_DIR)/%.o)

$(TARGET): $(OBJS)
	$(CXX) $(OBJS) -o $(BUILD_DIR)/$(TARGET)

$(BUILD_DIR)/%.o: %.cpp
	@mkdir -p $(@D)
	$(CXX) $(CXXFLAGS) -c $< -o $@

.PHONY: clean
clean:
	rm -rf $(BUILD_DIR)
"#,
                cpp_standard, self.name
            ),
            ProjectType::Library => format!(
                r#"CXX = g++
CXXFLAGS = -std=c++{} -Wall -Wextra -I include
LIB_TARGET = lib{}.a
BUILD_DIR = build

SRCS = $(wildcard src/*.cpp)
OBJS = $(SRCS:%.cpp=$(BUILD_DIR)/%.o)

$(LIB_TARGET): $(OBJS)
	ar rcs $(BUILD_DIR)/$(LIB_TARGET) $(OBJS)

$(BUILD_DIR)/%.o: %.cpp
	@mkdir -p $(@D)
	$(CXX) $(CXXFLAGS) -c $< -o $@

.PHONY: clean
clean:
	rm -rf $(BUILD_DIR)
"#,
                cpp_standard, self.name
            ),
        };

        fs::write(self.path.join("Makefile"), makefile_content)
            .context("Failed to write Makefile")?;

        Ok(())
    }

    pub fn generate_source_files(&self) -> Result<()> {
        match self.project_type {
            ProjectType::Executable => {
                let main_content = r#"#include <iostream>

int main() {
    std::cout << "Hello, World!\n";
    return 0;
}
"#;
                let test_content = r#"#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

TEST_CASE("Basic test") {
    CHECK(1 + 1 == 2);
}
"#;
                fs::write(self.path.join("src/main.cpp"), main_content)
                    .context("Failed to write main.cpp")?;
                fs::write(self.path.join("tests/main_test.cpp"), test_content)
                    .context("Failed to write main_test.cpp")?;
            }
            ProjectType::Library => {
                let header_content = format!(
                    r#"#pragma once

namespace {} {{

class Calculator {{
public:
    static int add(int a, int b);
}};

}} // namespace {}
"#,
                    self.name, self.name
                );

                let source_content = format!(
                    r#"#include "{}.hpp"

namespace {} {{

int Calculator::add(int a, int b) {{
    return a + b;
}}

}} // namespace {}
"#,
                    self.name, self.name, self.name
                );

                let example_content = format!(
                    r#"#include <iostream>
#include "{}.hpp"

int main() {{
    int result = {}::Calculator::add(40, 2);
    std::cout << "40 + 2 = " << result << "\n";
    return 0;
}}
"#,
                    self.name, self.name
                );

                fs::write(
                    self.path.join(format!("include/{}.hpp", self.name)),
                    header_content,
                )
                .context("Failed to write header file")?;
                fs::write(self.path.join("src/lib.cpp"), source_content)
                    .context("Failed to write library source file")?;
                fs::write(self.path.join("examples/example.cpp"), example_content)
                    .context("Failed to write example file")?;
            }
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

            let gitignore_content = r#"# Build directory
build/

# IDE specific files
.vscode/
.idea/
*.swp
*~

# Compiled files
*.o
*.out
*.exe
*.dll
*.so
*.dylib

# CMake files
CMakeCache.txt
CMakeFiles/
cmake_install.cmake
compile_commands.json
"#;

            fs::write(self.path.join(".gitignore"), gitignore_content)
                .context("Failed to write .gitignore")?;
        }
        Ok(())
    }

    pub fn generate_readme(&self) -> Result<()> {
        let build_instructions = match self.build_system {
            BuildSystem::CMake => format!(
                r#"```bash
# Create a build directory
mkdir -p build && cd build

# Generate build files
cmake ..

# Build the project
cmake --build .

# Run the executable
./{}{}
</code_block_to_apply_changes_from>
"#,
                self.name, self.cpp_standard
            ),
            BuildSystem::Make => format!(
                r#"```bash
# Create a build directory
mkdir -p build && cd build

# Build the project
make

# Run the executable
./{}
</code_block_to_apply_changes_from>
"#,
                self.name
            ),
        };

        let project_structure = match self.project_type {
            ProjectType::Executable => {
                r#"
```
src/          # Source files
├── main.cpp  # Main application entry point
include/      # Header files
build/        # Build output directory
tests/        # Test files
assets/       # Application assets
```"#
            }
            ProjectType::Library => {
                r#"
```
src/          # Source files
├── lib.cpp   # Library implementation
include/      # Header files
├── *.hpp     # Public headers
build/        # Build output directory
tests/        # Test files
examples/     # Example usage
```"#
            }
        };

        let readme_content = format!(
            r#"# {}

## Description
Add your project description here.

## Building the Project

### Prerequisites
- C++ compiler with C++{} support
- {}

### Build Instructions
{}

### Project Structure
{}
"#,
            self.name, self.cpp_standard, self.build_system, build_instructions, project_structure
        );

        fs::write(self.path.join("README.md"), readme_content)
            .context("Failed to write README.md")?;

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
        let clang_format_content = r#"---
Language: Cpp
BasedOnStyle: Google
IndentWidth: 4
ColumnLimit: 100
---"#;

        fs::write(self.path.join(".clang-format"), clang_format_content)
            .context("Failed to write .clang-format")?;

        Ok(())
    }
}
