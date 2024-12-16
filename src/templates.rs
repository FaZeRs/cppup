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
    pub year: String,
    pub enable_tests: bool,
    pub package_manager: String,
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
    ];

    for (name, content) in templates {
        handlebars
            .register_template_string(name, content)
            .unwrap_or_else(|e| panic!("Failed to register template {}: {}", name, e));
    }

    handlebars
}
