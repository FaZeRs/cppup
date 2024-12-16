mod cli;
mod project;
mod templates;

use crate::cli::Cli;
use crate::project::{
    BuildSystem, CppStandard, License, PackageManager, ProjectConfig, ProjectType,
};
use anyhow::{Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

fn generate_project(config: &ProjectConfig) -> Result<()> {
    println!("\nCreating C++ project with following settings:");
    println!("Project Name: {}", config.name);
    println!("Project Type: {:?}", config.project_type);
    println!("Build System: {:?}", config.build_system);
    println!("C++ Standard: {:?}", config.cpp_standard);
    println!("Enable Tests: {}", config.enable_tests);
    println!("Package Manager: {:?}", config.package_manager);
    println!("License: {:?}", config.license);
    println!("Initialize Git: {}", config.use_git);
    println!("Project Path: {}", config.path.display());

    let pb = ProgressBar::new(9);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Create directory structure
    pb.set_message("Creating directory structure...");
    config.create_directory_structure()?;
    pb.inc(1);

    // Generate build system files
    pb.set_message("Generating build system files...");
    match config.build_system {
        BuildSystem::CMake => config.generate_cmake_file()?,
        BuildSystem::Make => config.generate_makefile()?,
    }
    pb.inc(1);

    // Generate source files
    pb.set_message("Generating source files...");
    config.generate_source_files()?;
    pb.inc(1);

    // Generate package manager files
    pb.set_message("Setting up package manager...");
    config.generate_package_manager_files()?;
    pb.inc(1);

    // Generate test files
    pb.set_message("Generating test files...");
    config.generate_test_files()?;
    pb.inc(1);

    // Generate README
    pb.set_message("Generating README...");
    config.generate_readme()?;
    pb.inc(1);

    pb.set_message("Generating license...");
    config.generate_license()?;
    pb.inc(1);

    // Initialize git if requested
    if config.use_git {
        pb.set_message("Initializing git repository...");
        config.initialize_git()?;
    }
    pb.inc(1);

    // Generate .clang-format
    pb.set_message("Generating .clang-format...");
    config.generate_clang_format()?;
    pb.inc(1);

    pb.finish_with_message("Done!");
    Ok(())
}

fn create_config_from_cli(cli: &Cli) -> Result<ProjectConfig> {
    let name = cli
        .name
        .clone()
        .context("Project name is required in non-interactive mode")?;

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

    let package_manager = match cli.package_manager.as_str() {
        "conan" => PackageManager::Conan,
        "vcpkg" => PackageManager::Vcpkg,
        _ => PackageManager::None,
    };

    let license = match cli.license.as_str() {
        "MIT" => License::MIT,
        "Apache-2.0" => License::Apache,
        "GPL-3.0" => License::GPL,
        "BSD-3-Clause" => License::BSD,
        _ => unreachable!(),
    };

    Ok(ProjectConfig {
        name,
        project_type,
        build_system,
        cpp_standard,
        use_git: cli.git,
        path,
        enable_tests: cli.enable_tests,
        package_manager,
        license,
    })
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("Welcome to CPP Project Generator!");

    let config = if cli.non_interactive {
        create_config_from_cli(&cli)?
    } else {
        ProjectConfig::new(Some(&cli))?
    };

    // Check prerequisites before proceeding
    config.check_prerequisites()?;

    generate_project(&config)?;

    println!("\n✨ Project created successfully!");

    // Print next steps
    println!("\nNext steps:");
    println!("1. cd {}", config.path.display());

    match config.package_manager {
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
        PackageManager::None => match config.build_system {
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

    Ok(())
}
