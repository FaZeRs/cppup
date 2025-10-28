use super::config::{ProjectConfig, ProjectType};
use super::{BuildSystem, PackageManager, TestFramework};
use crate::templates::{ProjectTemplateData, TemplateRenderer};
use anyhow::{Context, Result};
use chrono::prelude::*;
use std::fs;
use std::process::Command;

pub struct ProjectBuilder {
    config: ProjectConfig,
    template_renderer: TemplateRenderer,
    template_data: ProjectTemplateData,
}

fn create_template_data(config: &ProjectConfig) -> ProjectTemplateData {
    ProjectTemplateData {
        name: config.name.clone(),
        cpp_standard: config.cpp_standard.to_string(),
        is_library: matches!(config.project_type, ProjectType::Library),
        namespace: config.name.replace('-', "_"),
        build_system: config.build_system.to_string(),
        description: config.description.clone(),
        author: config.author.clone(),
        version: config.version.to_string(),
        year: Local::now().year().to_string(),
        enable_tests: config.test_framework != TestFramework::None,
        test_framework: config.test_framework.to_string(),
        package_manager: config.package_manager.to_string(),
        quality_config: config.quality_config.to_string(),
        code_formatter: config.code_formatter.to_string(),
    }
}

impl ProjectBuilder {
    pub fn new(config: ProjectConfig) -> Self {
        let template_data = create_template_data(&config);
        Self {
            config,
            template_renderer: TemplateRenderer::new(),
            template_data,
        }
    }

    pub fn build(&self) -> Result<()> {
        self.create_directory_structure()?;
        self.render_templates()?;
        self.setup_package_manager()?;
        self.initialize_git()?;
        self.print_success_message();
        Ok(())
    }

    fn create_directory_structure(&self) -> Result<()> {
        // Create main project directory
        fs::create_dir_all(&self.config.path).with_context(|| {
            format!(
                "Failed to create project directory at {:?}",
                self.config.path
            )
        })?;

        // Create standard directories
        let mut dirs = vec![
            "src",
            "cmake",
            "include",
            match self.config.project_type {
                ProjectType::Library => "examples",
                ProjectType::Executable => "assets",
            },
        ];

        if self.config.test_framework != TestFramework::None {
            dirs.push("tests");
        }

        for dir in dirs {
            fs::create_dir_all(self.config.path.join(dir))
                .with_context(|| format!("Failed to create {} directory", dir))?;
        }

        Ok(())
    }

    fn render_templates(&self) -> Result<()> {
        match self.config.build_system {
            BuildSystem::CMake => self.generate_cmake_files()?,
            BuildSystem::Make => self.generate_makefile()?,
        }
        self.generate_source_files()?;
        self.generate_test_files()?;
        self.generate_readme()?;
        self.generate_quality_files()?;
        self.generate_code_formatter_files()?;
        self.generate_license()?;
        Ok(())
    }

    fn initialize_git(&self) -> Result<()> {
        if self.config.use_git {
            Command::new("git")
                .arg("init")
                .current_dir(&self.config.path)
                .output()
                .context("Failed to initialize git repository")?;

            self.template_renderer.render(
                "gitignore",
                &self.template_data,
                &self.config.path.join(".gitignore"),
            )?;
        }
        Ok(())
    }

    fn setup_package_manager(&self) -> Result<()> {
        match self.config.package_manager {
            PackageManager::Conan => {
                self.template_renderer.render(
                    "conanfile.txt",
                    &self.template_data,
                    &self.config.path.join("conanfile.txt"),
                )?;
            }
            PackageManager::Vcpkg => {
                self.template_renderer.render(
                    "vcpkg.json",
                    &self.template_data,
                    &self.config.path.join("vcpkg.json"),
                )?;
            }
            PackageManager::None => {}
        }
        Ok(())
    }

    fn generate_cmake_files(&self) -> Result<()> {
        self.template_renderer.render(
            "CMakeLists.txt",
            &self.template_data,
            &self.config.path.join("CMakeLists.txt"),
        )?;

        self.template_renderer.render(
            "options.cmake",
            &self.template_data,
            &self.config.path.join("cmake/options.cmake"),
        )?;

        self.template_renderer.render(
            "compilation-flags.cmake",
            &self.template_data,
            &self.config.path.join("cmake/compilation-flags.cmake"),
        )?;

        self.template_renderer.render(
            "source.cmake",
            &self.template_data,
            &self.config.path.join("src/CMakeLists.txt"),
        )?;

        if self.config.project_type == ProjectType::Library {
            self.template_renderer.render(
                "example.cmake",
                &self.template_data,
                &self.config.path.join("examples/CMakeLists.txt"),
            )?;
        }

        Ok(())
    }

    fn generate_makefile(&self) -> Result<()> {
        self.template_renderer.render(
            "Makefile",
            &self.template_data,
            &self.config.path.join("Makefile"),
        )?;

        Ok(())
    }

    fn generate_source_files(&self) -> Result<()> {
        match self.config.project_type {
            ProjectType::Executable => {
                self.template_renderer.render(
                    "main.cpp",
                    &self.template_data,
                    &self.config.path.join("src/main.cpp"),
                )?;
            }
            ProjectType::Library => {
                self.template_renderer.render(
                    "header.hpp",
                    &self.template_data,
                    &self
                        .config
                        .path
                        .join(format!("include/{}.hpp", self.config.name)),
                )?;
                self.template_renderer.render(
                    "library.cpp",
                    &self.template_data,
                    &self.config.path.join("src/lib.cpp"),
                )?;
                self.template_renderer.render(
                    "example.cpp",
                    &self.template_data,
                    &self.config.path.join("examples/example.cpp"),
                )?;
            }
        }

        Ok(())
    }

    fn generate_test_files(&self) -> Result<()> {
        if self.config.test_framework != TestFramework::None {
            if self.config.build_system == BuildSystem::CMake {
                self.template_renderer.render(
                    "tests.cmake",
                    &self.template_data,
                    &self.config.path.join("tests/CMakeLists.txt"),
                )?;
            }

            match self.config.test_framework {
                TestFramework::Doctest => {
                    self.template_renderer.render(
                        "doctest_main.cpp",
                        &self.template_data,
                        &self.config.path.join("tests/main_test.cpp"),
                    )?;
                }
                TestFramework::GTest => {
                    self.template_renderer.render(
                        "gtest_main.cpp",
                        &self.template_data,
                        &self.config.path.join("tests/main_test.cpp"),
                    )?;
                }
                TestFramework::BoostTest => {
                    self.template_renderer.render(
                        "boost_test_main.cpp",
                        &self.template_data,
                        &self.config.path.join("tests/main_test.cpp"),
                    )?;
                }
                TestFramework::Catch2 => {
                    self.template_renderer.render(
                        "catch2_main.cpp",
                        &self.template_data,
                        &self.config.path.join("tests/main_test.cpp"),
                    )?;
                }
                TestFramework::None => {}
            }
        }
        Ok(())
    }

    fn generate_readme(&self) -> Result<()> {
        self.template_renderer.render(
            "README.md",
            &self.template_data,
            &self.config.path.join("README.md"),
        )?;

        Ok(())
    }

    fn generate_license(&self) -> Result<()> {
        self.template_renderer.render(
            &self.config.license.to_string(),
            &self.template_data,
            &self.config.path.join("LICENSE"),
        )?;

        Ok(())
    }

    fn generate_quality_files(&self) -> Result<()> {
        if self.config.quality_config.enable_clang_tidy {
            self.template_renderer.render(
                "clang-tidy",
                &self.template_data,
                &self.config.path.join(".clang-tidy"),
            )?;
        }
        if self.config.quality_config.enable_cppcheck {
            self.template_renderer.render(
                "cppcheck-suppressions.xml",
                &self.template_data,
                &self.config.path.join("cppcheck-suppressions.xml"),
            )?;
        }
        Ok(())
    }

    fn generate_code_formatter_files(&self) -> Result<()> {
        if self.config.code_formatter.enable_clang_format {
            self.template_renderer.render(
                "clang-format",
                &self.template_data,
                &self.config.path.join(".clang-format"),
            )?;
        }
        if self.config.code_formatter.enable_cmake_format {
            self.template_renderer.render(
                "cmake-format",
                &self.template_data,
                &self.config.path.join("cmake-format.yaml"),
            )?;
        }
        Ok(())
    }

    fn print_success_message(&self) {
        println!("\n✨ Project created successfully!");

        // Print next steps
        println!("\nNext steps:");
        println!("1. cd {}", self.config.path.display());

        match self.config.package_manager {
            PackageManager::Conan => {
                println!("2. mkdir build && cd build");
                println!("3. conan install .. --output-folder=. --build=missing");
                println!("4. cmake .. -DCMAKE_TOOLCHAIN_FILE=./conan_toolchain.cmake");
                println!("5. cmake --build .");
            }
            PackageManager::Vcpkg => {
                println!("2. mkdir build && cd build");
                println!(
                    "3. cmake .. -DCMAKE_TOOLCHAIN_FILE=${{VCPKG_ROOT}}/scripts/buildsystems/vcpkg.cmake"
                );
                println!("4. cmake --build .");
            }
            PackageManager::None => match self.config.build_system {
                BuildSystem::CMake => {
                    println!("2. mkdir build && cd build");
                    println!("3. cmake ..");
                    println!("4. cmake --build .");
                }
                BuildSystem::Make => {
                    println!("2. make");
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::config::CppStandard;
    use crate::project::{CodeFormatter, License, QualityConfig};

    fn create_test_config() -> ProjectConfig {
        ProjectConfig {
            name: "test-project".to_string(),
            description: "A test project".to_string(),
            project_type: ProjectType::Executable,
            build_system: BuildSystem::CMake,
            cpp_standard: CppStandard::Cpp17,
            test_framework: TestFramework::Doctest,
            package_manager: PackageManager::Conan,
            license: License::MIT,
            use_git: true,
            path: std::path::PathBuf::from("/tmp/test-project"),
            author: "Test Author".to_string(),
            version: "1.0.0".to_string(),
            quality_config: QualityConfig::new(&["clang-tidy", "cppcheck"]),
            code_formatter: CodeFormatter::new(&["clang-format"]),
        }
    }

    #[test]
    fn test_create_template_data_executable() {
        let config = create_test_config();
        let data = create_template_data(&config);

        assert_eq!(data.name, "test-project");
        assert_eq!(data.cpp_standard, "17");
        assert_eq!(data.is_library, false);
        assert_eq!(data.namespace, "test_project");
        assert_eq!(data.build_system, "cmake");
        assert_eq!(data.description, "A test project");
        assert_eq!(data.author, "Test Author");
        assert_eq!(data.version, "1.0.0");
        assert_eq!(data.enable_tests, true);
        assert_eq!(data.test_framework, "doctest");
        assert_eq!(data.package_manager, "conan");
    }

    #[test]
    fn test_create_template_data_library() {
        let mut config = create_test_config();
        config.project_type = ProjectType::Library;
        let data = create_template_data(&config);

        assert_eq!(data.is_library, true);
        assert_eq!(data.name, "test-project");
    }

    #[test]
    fn test_create_template_data_namespace_conversion() {
        let mut config = create_test_config();
        config.name = "my-awesome-project".to_string();
        let data = create_template_data(&config);

        assert_eq!(data.namespace, "my_awesome_project");
    }

    #[test]
    fn test_create_template_data_no_tests() {
        let mut config = create_test_config();
        config.test_framework = TestFramework::None;
        let data = create_template_data(&config);

        assert_eq!(data.enable_tests, false);
        assert_eq!(data.test_framework, "none");
    }

    #[test]
    fn test_create_template_data_different_standards() {
        let mut config = create_test_config();
        config.cpp_standard = CppStandard::Cpp20;
        let data = create_template_data(&config);
        assert_eq!(data.cpp_standard, "20");

        config.cpp_standard = CppStandard::Cpp11;
        let data = create_template_data(&config);
        assert_eq!(data.cpp_standard, "11");
    }

    #[test]
    fn test_create_template_data_package_managers() {
        let mut config = create_test_config();

        config.package_manager = PackageManager::Vcpkg;
        let data = create_template_data(&config);
        assert_eq!(data.package_manager, "vcpkg");

        config.package_manager = PackageManager::None;
        let data = create_template_data(&config);
        assert_eq!(data.package_manager, "none");
    }

    #[test]
    fn test_project_builder_creation() {
        let config = create_test_config();
        let builder = ProjectBuilder::new(config.clone());

        assert_eq!(builder.config.name, "test-project");
        assert_eq!(builder.template_data.name, "test-project");
    }
}
