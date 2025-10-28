use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

// ============================================================================
// Basic Command Tests
// ============================================================================

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "interactive C++ project generator",
    ));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("cppup"));
}

// ============================================================================
// Basic Project Creation Tests
// ============================================================================

#[test]
fn test_non_interactive_project_creation() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "test-project",
        "--description",
        "Test project",
        "--project-type",
        "executable",
        "--build-system",
        "cmake",
        "--cpp-standard",
        "17",
        "--package-manager",
        "none",
        "--test-framework",
        "none",
        "--license",
        "MIT",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    // Verify project structure
    assert!(project_path.exists());
    assert!(project_path.join("src").exists());
    assert!(project_path.join("src/main.cpp").exists());
    assert!(project_path.join("CMakeLists.txt").exists());
    assert!(project_path.join("README.md").exists());
    assert!(project_path.join("LICENSE").exists());
}

#[test]
fn test_library_project_creation() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-lib");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "test-lib",
        "--description",
        "Test library",
        "--project-type",
        "library",
        "--build-system",
        "cmake",
        "--cpp-standard",
        "20",
        "--package-manager",
        "none",
        "--test-framework",
        "none",
        "--license",
        "MIT",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    // Verify library-specific structure
    assert!(project_path.exists());
    assert!(project_path.join("src").exists());
    assert!(project_path.join("include").exists());
    assert!(project_path.join("examples").exists());
    assert!(project_path.join("src/lib.cpp").exists());
    assert!(project_path.join("include/test-lib.hpp").exists());
}

// ============================================================================
// Build System Tests
// ============================================================================

#[test]
fn test_make_build_system_executable() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("make-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "make-project",
        "--project-type",
        "executable",
        "--build-system",
        "make",
        "--cpp-standard",
        "17",
        "--test-framework",
        "none",
        "--license",
        "MIT",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    // Verify Makefile exists
    assert!(project_path.join("Makefile").exists());
    assert!(!project_path.join("CMakeLists.txt").exists());
}

#[test]
fn test_make_build_system_library() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("make-lib");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "make-lib",
        "--project-type",
        "library",
        "--build-system",
        "make",
        "--cpp-standard",
        "17",
        "--test-framework",
        "none",
        "--license",
        "MIT",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    assert!(project_path.join("Makefile").exists());
    assert!(project_path.join("include").exists());
    assert!(project_path.join("examples").exists());
}

// ============================================================================
// Test Framework Tests
// ============================================================================

#[test]
fn test_doctest_framework() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("doctest-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "doctest-project",
        "--project-type",
        "executable",
        "--test-framework",
        "doctest",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    // Verify test directory and files
    assert!(project_path.join("tests").exists());
    assert!(project_path.join("tests/main_test.cpp").exists());
    assert!(project_path.join("tests/CMakeLists.txt").exists());
}

#[test]
fn test_gtest_framework() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("gtest-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "gtest-project",
        "--project-type",
        "executable",
        "--test-framework",
        "gtest",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    assert!(project_path.join("tests").exists());
    assert!(project_path.join("tests/main_test.cpp").exists());
}

#[test]
fn test_catch2_framework() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("catch2-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "catch2-project",
        "--project-type",
        "executable",
        "--test-framework",
        "catch2",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    assert!(project_path.join("tests").exists());
    assert!(project_path.join("tests/main_test.cpp").exists());
}

#[test]
fn test_boosttest_framework() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("boost-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "boost-project",
        "--project-type",
        "executable",
        "--test-framework",
        "boosttest",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    assert!(project_path.join("tests").exists());
    assert!(project_path.join("tests/main_test.cpp").exists());
}

// ============================================================================
// Package Manager Tests
// ============================================================================

#[test]
fn test_conan_package_manager() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("conan-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "conan-project",
        "--project-type",
        "executable",
        "--package-manager",
        "conan",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    // Verify Conan configuration file exists
    assert!(project_path.join("conanfile.txt").exists());
}

#[test]
fn test_vcpkg_package_manager() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("vcpkg-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "vcpkg-project",
        "--project-type",
        "executable",
        "--package-manager",
        "vcpkg",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    // Verify vcpkg configuration file exists
    assert!(project_path.join("vcpkg.json").exists());
}

// ============================================================================
// C++ Standard Tests
// ============================================================================

#[test]
fn test_cpp11_standard() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("cpp11-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "cpp11-project",
        "--project-type",
        "executable",
        "--cpp-standard",
        "11",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();
    assert!(project_path.exists());
}

#[test]
fn test_cpp14_standard() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("cpp14-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "cpp14-project",
        "--project-type",
        "executable",
        "--cpp-standard",
        "14",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();
    assert!(project_path.exists());
}

#[test]
fn test_cpp20_standard() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("cpp20-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "cpp20-project",
        "--project-type",
        "executable",
        "--cpp-standard",
        "20",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();
    assert!(project_path.exists());
}

#[test]
fn test_cpp23_standard() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("cpp23-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "cpp23-project",
        "--project-type",
        "executable",
        "--cpp-standard",
        "23",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();
    assert!(project_path.exists());
}

// ============================================================================
// License Tests
// ============================================================================

#[test]
fn test_apache_license() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("apache-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "apache-project",
        "--project-type",
        "executable",
        "--license",
        "Apache-2.0",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    let license_content = fs::read_to_string(project_path.join("LICENSE")).unwrap();
    assert!(license_content.contains("Apache License"));
}

#[test]
fn test_gpl_license() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("gpl-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "gpl-project",
        "--project-type",
        "executable",
        "--license",
        "GPL-3.0",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    let license_content = fs::read_to_string(project_path.join("LICENSE")).unwrap();
    assert!(license_content.contains("GNU GENERAL PUBLIC LICENSE"));
}

#[test]
fn test_bsd_license() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("bsd-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "bsd-project",
        "--project-type",
        "executable",
        "--license",
        "BSD-3-Clause",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    let license_content = fs::read_to_string(project_path.join("LICENSE")).unwrap();
    assert!(license_content.contains("BSD") || license_content.contains("Redistribution"));
}

// ============================================================================
// Quality Tools and Formatter Tests
// ============================================================================

#[test]
fn test_quality_tools_clang_tidy() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("quality-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "quality-project",
        "--project-type",
        "executable",
        "--quality-tools",
        "clang-tidy",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    // Verify quality tool configuration file exists
    assert!(project_path.join(".clang-tidy").exists());
}

#[test]
fn test_quality_tools_cppcheck() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("cppcheck-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "cppcheck-project",
        "--project-type",
        "executable",
        "--quality-tools",
        "cppcheck",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    assert!(project_path.join("cppcheck-suppressions.xml").exists());
}

#[test]
fn test_code_formatter_clang_format() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("format-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "format-project",
        "--project-type",
        "executable",
        "--code-formatter",
        "clang-format",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    assert!(project_path.join(".clang-format").exists());
}

#[test]
fn test_code_formatter_cmake_format() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("cmake-format-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "cmake-format-project",
        "--project-type",
        "executable",
        "--code-formatter",
        "cmake-format",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    assert!(project_path.join("cmake-format.yaml").exists());
}

// ============================================================================
// Git Tests
// ============================================================================

#[test]
fn test_git_initialization() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("git-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "git-project",
        "--project-type",
        "executable",
        "--git",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    // Verify git repository and .gitignore exist
    assert!(project_path.join(".git").exists());
    assert!(project_path.join(".gitignore").exists());
}

// ============================================================================
// Error Condition Tests
// ============================================================================

#[test]
fn test_invalid_project_name() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "123invalid",
        "--project-type",
        "executable",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().failure();
}

#[test]
fn test_project_name_with_spaces() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "invalid name",
        "--project-type",
        "executable",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().failure();
}

#[test]
fn test_project_name_with_special_chars() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "invalid@project!",
        "--project-type",
        "executable",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().failure();
}

#[test]
fn test_missing_required_name() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--project-type",
        "executable",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().failure();
}

#[test]
fn test_missing_required_project_type() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "test-project",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().failure();
}

#[test]
fn test_duplicate_project_creation() {
    let temp_dir = TempDir::new().unwrap();

    // Create first project successfully
    let mut cmd1 = Command::cargo_bin("cppup").unwrap();
    cmd1.args([
        "--name",
        "duplicate-project",
        "--project-type",
        "executable",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);
    cmd1.assert().success();

    // Try to create the same project again - should fail
    let mut cmd2 = Command::cargo_bin("cppup").unwrap();
    cmd2.args([
        "--name",
        "duplicate-project",
        "--project-type",
        "executable",
        "--test-framework",
        "none",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);
    cmd2.assert().failure();
}

// ============================================================================
// Complex Integration Tests
// ============================================================================

#[test]
fn test_full_featured_project() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("full-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "full-project",
        "--description",
        "A fully featured test project",
        "--author",
        "Test Author",
        "--project-type",
        "library",
        "--build-system",
        "cmake",
        "--cpp-standard",
        "20",
        "--package-manager",
        "conan",
        "--test-framework",
        "doctest",
        "--license",
        "MIT",
        "--quality-tools",
        "clang-tidy,cppcheck",
        "--code-formatter",
        "clang-format",
        "--git",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    // Verify all expected files exist
    assert!(project_path.join("src/lib.cpp").exists());
    assert!(project_path.join("include/full-project.hpp").exists());
    assert!(project_path.join("examples").exists());
    assert!(project_path.join("tests").exists());
    assert!(project_path.join("CMakeLists.txt").exists());
    assert!(project_path.join("conanfile.txt").exists());
    assert!(project_path.join(".clang-tidy").exists());
    assert!(project_path.join("cppcheck-suppressions.xml").exists());
    assert!(project_path.join(".clang-format").exists());
    assert!(project_path.join(".gitignore").exists());
    assert!(project_path.join("LICENSE").exists());
    assert!(project_path.join("README.md").exists());
}

#[test]
fn test_executable_with_make_and_tests() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("make-test-project");

    let mut cmd = Command::cargo_bin("cppup").unwrap();
    cmd.args([
        "--name",
        "make-test-project",
        "--project-type",
        "executable",
        "--build-system",
        "make",
        "--cpp-standard",
        "17",
        "--test-framework",
        "catch2",
        "--non-interactive",
        "--path",
        temp_dir.path().to_str().unwrap(),
    ]);

    cmd.assert().success();

    assert!(project_path.join("Makefile").exists());
    assert!(project_path.join("tests").exists());
    assert!(project_path.join("src/main.cpp").exists());
}
