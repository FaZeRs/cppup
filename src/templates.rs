use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProjectTemplateData {
    pub name: String,
    pub cpp_standard: String,
    pub is_library: bool,
    pub namespace: String,
    pub build_system: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub version: String,
    pub license: Option<String>,
    pub enable_tests: bool,
}

pub fn create_template_registry() -> Handlebars<'static> {
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
        (
            "main_test.cpp",
            include_str!("../templates/main_test.cpp.hbs"),
        ),
        ("gitignore", include_str!("../templates/gitignore.hbs")),
        ("README.md", include_str!("../templates/README.md.hbs")),
        (
            "clang-format",
            include_str!("../templates/clang-format.hbs"),
        ),
    ];

    for (name, content) in templates {
        handlebars
            .register_template_string(name, content)
            .unwrap_or_else(|e| panic!("Failed to register template {}: {}", name, e));
    }

    handlebars
}
