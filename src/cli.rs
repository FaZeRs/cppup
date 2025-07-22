use clap::Parser;
use std::path::PathBuf;

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
    #[arg(short, long, value_parser = ["cmake", "make", "ninja", "bazel", "meson"], default_value = "cmake")]
    pub build_system: String,

    /// C++ standard to use
    #[arg(short = 's', long, value_parser = ["11", "14", "17", "20", "23", "26"], default_value = "17")]
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

    /// Enable C++20 modules support
    #[arg(long)]
    pub modules: bool,

    #[arg(long, value_parser = ["doctest", "gtest", "catch2", "boosttest", "none"], default_value = "none")]
    pub test_framework: String,

    #[arg(long, value_parser = ["conan", "vcpkg", "cpm", "hunter", "none"], default_value = "none")]
    pub package_manager: String,

    #[arg(long, value_parser = ["MIT", "Apache-2.0", "GPL-3.0", "BSD-3-Clause"], default_value = "MIT")]
    pub license: String,

    #[arg(long)]
    pub author: Option<String>,

    #[arg(long, value_delimiter = ',', value_parser = ["clang-tidy", "cppcheck", "clang-format", "cpplint", "include-what-you-use", "sanitizers"])]
    pub quality_tools: Vec<String>,

    /// Load configuration from file
    #[arg(short = 'c', long)]
    pub config: Option<PathBuf>,

    /// Save current configuration to file
    #[arg(long)]
    pub save_config: Option<PathBuf>,

    /// Generate CI/CD configuration
    #[arg(long, value_parser = ["github", "gitlab", "azure", "none"], default_value = "none")]
    pub ci: String,

    /// Enable Docker support
    #[arg(long)]
    pub docker: bool,

    /// Generate IDE configuration files
    #[arg(long, value_delimiter = ',', value_parser = ["vscode", "clion", "qtcreator", "vim"])]
    pub ide: Vec<String>,
}
