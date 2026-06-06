mod common;

// use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

// use crate::common::run_musicl;

#[test]
fn init_creates_library() {
    let temp_dir = tempdir().unwrap();

    common::run_musicl(&["init"], &temp_dir).success();

    assert!(temp_dir.path().join(".musicl").is_dir());
    assert!(temp_dir.path().join(".musicl").join("status").is_file());
}

#[test]
fn init_fails_when_musicl_directory_already_exists() {
    let temp_dir = tempdir().unwrap();

    fs::create_dir(temp_dir.path().join(".musicl")).unwrap();

    common::run_musicl(&["init"], &temp_dir).failure();

    // Check status not created
    assert!(!temp_dir.path().join(".musicl").join("status").is_file())
}

#[test]
fn init_does_not_modify_status() {
    let temp_dir = tempdir().unwrap();

    // Create non-empty status file
    common::make_tiny_library(&temp_dir);

    // Run init
    common::run_musicl(&["init"], &temp_dir).failure();

    // Check status exists and contents have not changed
    assert!(temp_dir.path().join(".musicl").join("status").is_file());
    let actual = fs::read_to_string(temp_dir.path().join(".musicl/status")).expect("Couldn't read status file.");
    let expected = fs::read_to_string("tests/data/tiny_library/.musicl/status",).expect("Couldn't read exemplar");
    assert_eq!(actual, expected);
}