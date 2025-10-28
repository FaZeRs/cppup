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
    pub quality_config: String,
    pub code_formatter: String,
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

    #[allow(dead_code)]
    pub fn render_to_string<T: Serialize>(&self, template_name: &str, data: &T) -> Result<String> {
        self.registry
            .render(template_name, &data)
            .with_context(|| format!("Failed to render template {}", template_name))
    }
}

impl Default for TemplateRenderer {
    fn default() -> Self {
        Self::new()
    }
}

fn contains_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let list = h.param(0).and_then(|p| p.value().as_str()).unwrap_or("");

    let value = h.param(1).and_then(|p| p.value().as_str()).unwrap_or("");

    let contains = list.split(',').any(|item| item.trim() == value);

    // Return boolean value for Handlebars #if helper
    if contains {
        out.write("true")?;
    } else {
        out.write("")?; // Empty string is falsy in Handlebars
    }

    Ok(())
}

fn create_template_registry() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();

    // Register helper functions
    handlebars.register_helper("contains", Box::new(contains_helper));

    // Register all templates with proper error handling
    let templates = vec![
        ("main.cpp", include_str!("../templates/main.cpp.hbs")),
        (
            "CMakeLists.txt",
            include_str!("../templates/cmake/CMakeLists.txt.hbs"),
        ),
        (
            "options.cmake",
            include_str!("../templates/cmake/options.cmake.hbs"),
        ),
        (
            "compilation-flags.cmake",
            include_str!("../templates/cmake/compilation-flags.cmake.hbs"),
        ),
        (
            "source.cmake",
            include_str!("../templates/cmake/source.cmake.hbs"),
        ),
        ("Makefile", include_str!("../templates/Makefile.hbs")),
        ("header.hpp", include_str!("../templates/header.hpp.hbs")),
        ("library.cpp", include_str!("../templates/library.cpp.hbs")),
        ("example.cpp", include_str!("../templates/example.cpp.hbs")),
        (
            "example.cmake",
            include_str!("../templates/cmake/example.cmake.hbs"),
        ),
        ("gitignore", include_str!("../templates/gitignore.hbs")),
        ("README.md", include_str!("../templates/README.md.hbs")),
        (
            "conanfile.txt",
            include_str!("../templates/package-managers/conanfile.txt.hbs"),
        ),
        (
            "vcpkg.json",
            include_str!("../templates/package-managers/vcpkg.json.hbs"),
        ),
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
            include_str!("../templates/formatters/clang-format.hbs"),
        ),
        (
            "cmake-format",
            include_str!("../templates/formatters/cmake-format.yaml.hbs"),
        ),
        (
            "clang-tidy",
            include_str!("../templates/static-analyzers/clang-tidy.hbs"),
        ),
        (
            "cppcheck-suppressions.xml",
            include_str!("../templates/static-analyzers/cppcheck-suppressions.xml.hbs"),
        ),
        (
            "tests.cmake",
            include_str!("../templates/tests/tests.cmake.hbs"),
        ),
        (
            "boost_test_main.cpp",
            include_str!("../templates/tests/boost_test_main.cpp.hbs"),
        ),
        (
            "catch2_main.cpp",
            include_str!("../templates/tests/catch2_main.cpp.hbs"),
        ),
        (
            "gtest_main.cpp",
            include_str!("../templates/tests/gtest_main.cpp.hbs"),
        ),
        (
            "doctest_main.cpp",
            include_str!("../templates/tests/doctest_main.cpp.hbs"),
        ),
    ];

    for (name, content) in templates {
        handlebars
            .register_template_string(name, content)
            .unwrap_or_else(|e| panic!("Failed to register template {}: {}", name, e));
    }

    handlebars
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_data() -> ProjectTemplateData {
        ProjectTemplateData {
            name: "test-project".to_string(),
            cpp_standard: "17".to_string(),
            is_library: false,
            namespace: "test_project".to_string(),
            build_system: "cmake".to_string(),
            description: "A test project".to_string(),
            author: "Test Author".to_string(),
            version: "0.1.0".to_string(),
            year: "2024".to_string(),
            enable_tests: true,
            test_framework: "doctest".to_string(),
            package_manager: "none".to_string(),
            quality_config: "none".to_string(),
            code_formatter: "none".to_string(),
        }
    }

    #[test]
    fn test_template_renderer_creation() {
        let _renderer = TemplateRenderer::new();
        // Should not panic
    }

    #[test]
    fn test_render_main_cpp() {
        let renderer = TemplateRenderer::new();
        let data = create_test_data();

        let result = renderer.render_to_string("main.cpp", &data);
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("#include"));
    }

    #[test]
    fn test_render_cmake() {
        let renderer = TemplateRenderer::new();
        let data = create_test_data();

        let result = renderer.render_to_string("CMakeLists.txt", &data);
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("cmake_minimum_required"));
        assert!(content.contains("test-project"));
    }

    #[test]
    fn test_render_to_file() {
        let renderer = TemplateRenderer::new();
        let data = create_test_data();
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("test.cpp");

        let result = renderer.render("main.cpp", &data, &output_path);
        assert!(result.is_ok());
        assert!(output_path.exists());

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("#include"));
    }

    #[test]
    fn test_invalid_template() {
        let renderer = TemplateRenderer::new();
        let data = create_test_data();

        let result = renderer.render_to_string("nonexistent", &data);
        assert!(result.is_err());
    }

    #[test]
    fn test_contains_helper() {
        let data = ProjectTemplateData {
            name: "test-project".to_string(),
            cpp_standard: "17".to_string(),
            is_library: false,
            namespace: "test_project".to_string(),
            build_system: "cmake".to_string(),
            description: "A test project".to_string(),
            author: "Test Author".to_string(),
            version: "0.1.0".to_string(),
            year: "2024".to_string(),
            enable_tests: true,
            test_framework: "doctest".to_string(),
            package_manager: "none".to_string(),
            quality_config: "clang-tidy,cppcheck".to_string(),
            code_formatter: "clang-format".to_string(),
        };

        // Test template that uses the contains helper
        let template = "{{#if (contains quality_config 'clang-tidy')}}clang-tidy enabled{{/if}}";
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("test_contains", template)
            .unwrap();
        handlebars.register_helper("contains", Box::new(contains_helper));

        let result = handlebars.render("test_contains", &data).unwrap();
        assert_eq!(result, "clang-tidy enabled");

        // Test with value not in list
        let template2 =
            "{{#if (contains quality_config 'include-what-you-use')}}iwyu enabled{{/if}}";
        handlebars
            .register_template_string("test_contains2", template2)
            .unwrap();
        let result2 = handlebars.render("test_contains2", &data).unwrap();
        assert_eq!(result2, "");
    }
}
