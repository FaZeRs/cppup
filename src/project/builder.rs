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
            BuildSystem::CMake => self.generate_cmake_file()?,
            BuildSystem::Make => self.generate_makefile()?,
        }
        self.generate_source_files()?;
        self.generate_test_files()?;
        self.generate_readme()?;
        self.generate_quality_files()?;
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

    fn generate_cmake_file(&self) -> Result<()> {
        self.template_renderer.render(
            "CMakeLists.txt",
            &self.template_data,
            &self.config.path.join("CMakeLists.txt"),
        )?;

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
            self.template_renderer.render(
                "tests.cmake",
                &self.template_data,
                &self.config.path.join("tests/CMakeLists.txt"),
            )?;

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
        if self.config.quality_config.enable_clang_format {
            self.template_renderer.render(
                "clang-format",
                &self.template_data,
                &self.config.path.join(".clang-format"),
            )?;
        }
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
