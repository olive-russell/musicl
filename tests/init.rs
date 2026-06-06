use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn init_creates_library() {
    let temp_dir = tempdir().unwrap();

    Command::cargo_bin("musicl")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success();

    assert!(temp_dir.path().join(".musicl").is_dir());
    assert!(temp_dir.path().join(".musicl").join("status").is_file());
}

#[test]
fn init_fails_when_musicl_directory_already_exists() {
    let temp_dir = tempdir().unwrap();

    fs::create_dir(temp_dir.path().join(".musicl")).unwrap();

    // Run init
    Command::cargo_bin("musicl")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .failure();

    // Check status not created
    assert!(!temp_dir.path().join(".musicl").join("status").is_file())
}

#[test]
fn init_does_not_modify_status() {
    let temp_dir = tempdir().unwrap();

    // Create non-empty status file
    fs::create_dir(temp_dir.path().join(".musicl")).unwrap();
    let status_path = temp_dir.path().join(".musicl").join("status");
    fs::copy("tests/data/status_1_line", &status_path).expect("Failed to copy test file.");

    // Run init
    Command::cargo_bin("musicl")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .failure();

    // Check status exists and contents have not changed
    assert!(temp_dir.path().join(".musicl").join("status").is_file());
    let actual = fs::read_to_string(status_path).expect("Couldn't read status file.");
    let expected = fs::read_to_string("tests/data/status_1_line",).expect("Couldn't read exemplar");
    assert_eq!(actual, expected);
}