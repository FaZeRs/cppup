mod cli;
mod project;
mod templates;

use crate::cli::Cli;
use crate::project::{ProjectBuilder, ProjectConfig, ProjectValidator};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("Welcome to CPP Project Generator!");

    let config = ProjectConfig::new(Some(&cli))?;

    let validator = ProjectValidator::new(config.clone());
    validator.check_prerequisites()?;

    let builder = ProjectBuilder::new(config);
    builder.build()?;

    Ok(())
}
