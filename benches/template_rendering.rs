use cppup::templates::{ProjectTemplateData, TemplateRenderer};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn create_test_data() -> ProjectTemplateData {
    ProjectTemplateData {
        name: "benchmark-project".to_string(),
        cpp_standard: "20".to_string(),
        is_library: false,
        namespace: "benchmark_project".to_string(),
        build_system: "cmake".to_string(),
        description: "A benchmark project".to_string(),
        author: "Benchmark Author".to_string(),
        version: "1.0.0".to_string(),
        year: "2024".to_string(),
        enable_tests: true,
        test_framework: "gtest".to_string(),
        package_manager: "conan".to_string(),
        quality_tools: vec!["clang-format".to_string(), "clang-tidy".to_string()],
        ci: "github".to_string(),
        docker: true,
        ide: vec!["vscode".to_string()],
        modules: false,
    }
}

fn bench_template_rendering(c: &mut Criterion) {
    let renderer = TemplateRenderer::new();
    let data = create_test_data();

    c.bench_function("render_cmake", |b| {
        b.iter(|| {
            renderer
                .render_to_string(black_box("CMakeLists.txt"), black_box(&data))
                .unwrap()
        })
    });

    c.bench_function("render_main_cpp", |b| {
        b.iter(|| {
            renderer
                .render_to_string(black_box("main.cpp"), black_box(&data))
                .unwrap()
        })
    });

    c.bench_function("render_github_actions", |b| {
        b.iter(|| {
            renderer
                .render_to_string(black_box("github-actions.yml"), black_box(&data))
                .unwrap()
        })
    });
}

criterion_group!(benches, bench_template_rendering);
criterion_main!(benches);
