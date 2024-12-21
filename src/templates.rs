use anyhow::{Context, Result};
use handlebars::Handlebars;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
pub struct ProjectTemplateData {
    pub name: String,
    pub cpp_standard: String,
    pub is_library: bool,
    pub namespace: String,
    pub build_system: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub year: String,
    pub enable_tests: bool,
    pub test_framework: String,
    pub package_manager: String,
}

pub struct TemplateRenderer {
    registry: Handlebars<'static>,
}

impl TemplateRenderer {
    pub fn new() -> Self {
        Self {
            registry: create_template_registry(),
        }
    }
    pub fn render<T: Serialize>(
        &self,
        template_name: &str,
        data: &T,
        output_path: &Path,
    ) -> Result<()> {
        let rendered = self
            .registry
            .render(template_name, &data)
            .with_context(|| format!("Failed to render template {}", template_name))?;

        fs::write(output_path, rendered)
            .with_context(|| format!("Failed to write file {}", output_path.display()))?;

        Ok(())
    }
}

fn create_template_registry() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();

    // Register all templates with proper error handling
    let templates = vec![
        ("main.cpp", include_str!("../templates/main.cpp.hbs")),
        (
            "CMakeLists.txt",
            include_str!("../templates/CMakeLists.txt.hbs"),
        ),
        ("Makefile", include_str!("../templates/Makefile.hbs")),
        ("header.hpp", include_str!("../templates/header.hpp.hbs")),
        ("library.cpp", include_str!("../templates/library.cpp.hbs")),
        ("example.cpp", include_str!("../templates/example.cpp.hbs")),
        ("gitignore", include_str!("../templates/gitignore.hbs")),
        ("README.md", include_str!("../templates/README.md.hbs")),
        (
            "conanfile.txt",
            include_str!("../templates/conanfile.txt.hbs"),
        ),
        ("vcpkg.json", include_str!("../templates/vcpkg.json.hbs")),
        ("MIT", include_str!("../templates/licenses/MIT.hbs")),
        ("GPL-3.0", include_str!("../templates/licenses/GPL-3.0.hbs")),
        (
            "BSD-3-Clause",
            include_str!("../templates/licenses/BSD-3-Clause.hbs"),
        ),
        (
            "Apache-2.0",
            include_str!("../templates/licenses/Apache-2.0.hbs"),
        ),
        (
            "clang-format",
            include_str!("../templates/clang-format.hbs"),
        ),
        ("clang-tidy", include_str!("../templates/clang-tidy.hbs")),
        (
            "cppcheck-suppressions.xml",
            include_str!("../templates/cppcheck-suppressions.xml.hbs"),
        ),
        ("tests.cmake", include_str!("../templates/tests.cmake.hbs")),
        (
            "boost_test_main.cpp",
            include_str!("../templates/boost_test_main.cpp.hbs"),
        ),
        (
            "catch2_main.cpp",
            include_str!("../templates/catch2_main.cpp.hbs"),
        ),
        (
            "gtest_main.cpp",
            include_str!("../templates/gtest_main.cpp.hbs"),
        ),
        (
            "doctest_main.cpp",
            include_str!("../templates/doctest_main.cpp.hbs"),
        ),
    ];

    for (name, content) in templates {
        handlebars
            .register_template_string(name, content)
            .unwrap_or_else(|e| panic!("Failed to register template {}: {}", name, e));
    }

    handlebars
}
