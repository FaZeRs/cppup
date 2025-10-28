//! # cppup
//!
//! A powerful and interactive C++ project generator written in Rust.
//!
//! This library provides tools to create modern C++ projects with best practices,
//! supporting multiple build systems, package managers, testing frameworks, and
//! code quality tools.
//!
//! ## Features
//!
//! - Interactive CLI with smart defaults
//! - Multiple build systems (CMake, Make)
//! - Package manager integration (Conan, Vcpkg)
//! - Testing framework setup (doctest, Google Test, Catch2, Boost.Test)
//! - Code quality tools (clang-format, clang-tidy, cppcheck)
//! - License management (MIT, Apache-2.0, GPL-3.0, BSD-3-Clause)
//! - Project templates (Executable, Library)
//! - Git initialization
//!
//! ## Example
//!
//! ```no_run
//! use cppup::{ProjectConfig, ProjectValidator, ProjectBuilder};
//! use anyhow::Result;
//!
//! fn main() -> Result<()> {
//!     // Create project configuration (in real usage, this would be from CLI or interactive mode)
//!     // let config = ProjectConfig::new(None)?;
//!
//!     // Validate prerequisites
//!     // let validator = ProjectValidator::new(config.clone());
//!     // validator.check_prerequisites()?;
//!
//!     // Build the project
//!     // let builder = ProjectBuilder::new(config);
//!     // builder.build()?;
//!
//!     Ok(())
//! }
//! ```

pub mod cli;
pub mod project;
pub mod templates;

pub use project::{ProjectBuilder, ProjectConfig, ProjectValidator};
pub use templates::TemplateRenderer;
