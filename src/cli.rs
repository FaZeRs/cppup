use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Name of the project
    #[arg(short, long)]
    pub name: Option<String>,

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

    #[arg(long, default_value_t = true)]
    pub enable_tests: bool,

    #[arg(long, value_parser = ["conan", "vcpkg", "none"], default_value = "none")]
    pub package_manager: String,
}
