use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

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
