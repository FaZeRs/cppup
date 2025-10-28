//! Command-line interface definitions.
//!
//! This module defines the CLI structure and all command-line arguments
//! for the cppup project generator.

use clap::Parser;
use std::path::PathBuf;

/// Command-line interface for cppup.
///
/// This structure defines all available command-line arguments for
/// configuring a C++ project generation in non-interactive mode.
///
/// # Examples
///
/// ```no_run
/// use cppup::cli::Cli;
/// use clap::Parser;
///
/// let cli = Cli::parse();
/// ```
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Name of the project
    #[arg(short, long)]
    pub name: Option<String>,

    /// Project description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Project type (executable or library)
    #[arg(short = 't', long, value_parser = ["executable", "library"])]
    pub project_type: Option<String>,

    /// Build system to use
    #[arg(short, long, value_parser = ["cmake", "make"], default_value = "cmake")]
    pub build_system: String,

    /// C++ standard to use
    #[arg(short = 's', long, value_parser = ["11", "14", "17", "20", "23"], default_value = "17")]
    pub cpp_standard: String,

    /// Directory where to create the project
    #[arg(short = 'p', long, default_value = ".")]
    pub path: PathBuf,

    /// Initialize git repository
    #[arg(short, long, default_value_t = true)]
    pub git: bool,

    /// Non-interactive mode
    #[arg(short = 'i', long)]
    pub non_interactive: bool,

    #[arg(long, value_parser = ["doctest", "gtest", "catch2", "boosttest", "none"], default_value = "none")]
    pub test_framework: String,

    #[arg(long, value_parser = ["conan", "vcpkg", "none"], default_value = "none")]
    pub package_manager: String,

    #[arg(long, value_parser = ["MIT", "Apache-2.0", "GPL-3.0", "BSD-3-Clause"], default_value = "MIT")]
    pub license: String,

    #[arg(long)]
    pub author: Option<String>,

    #[arg(long, value_delimiter = ',', value_parser = ["clang-tidy", "cppcheck", "include-what-you-use"])]
    pub quality_tools: Vec<String>,

    #[arg(long, value_delimiter = ',', value_parser = ["clang-format", "cmake-format", "none"], default_value = "none")]
    pub code_formatter: Vec<String>,
}
