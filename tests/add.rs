use std::{fs, path::PathBuf};
use tempfile::tempdir;

mod common;

#[test]
fn add_rejects_bad_metadata() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, place a bad file at root
    common::make_tiny_library(&temp_dir);
    let bad_metadata_path = PathBuf::from("tests/data/bad_metadata_file1/Parliament/Funkentelechy Vs. The Placebo Syndrome/1-03 Wizard Of Finance.mp3");
    fs::copy(&bad_metadata_path, &temp_dir.path().join("1-03 Wizard of Finance.mp3")).expect("Failed to copy bad file in to root");
    
    // Run add subcommand
    common::run_musicl_get_stdout(&["add", bad_metadata_path.to_str().expect("Failed to strify")], &temp_dir);

    // Assert file not moved, status not changed
    assert!(bad_metadata_path.is_file());
    common::assert_file_contents_eq(temp_dir.path().join(".musicl/status"), PathBuf::from("tests/data/tiny_library/.musicl/status"))
}

#[test]
fn add_rejects_non_music_file() {
    common::rejects_non_music_file("add");
}

#[test]
fn add_moves_good_file() {
    common::moves_good_file("add");
}

#[test]
fn add_updates_status() {
    common::updates_status("add");
}

#[test]
fn add_creates_library_folder() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, place a good file at root
    common::make_empty_library(&temp_dir);
    let demo_good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let good_file_path = temp_dir.path().join("01 Stereo.mp3");
    fs::copy(&demo_good_file_path, &good_file_path).expect("Failed to copy good file in to library");

    // Run add subcommand
    common::run_musicl_get_stdout(&["add", good_file_path.to_str().expect("Failed to strify")], &temp_dir);

    // Assert library folder exists now
    assert!(temp_dir.path().join("library").is_dir());
}

#[test]
fn add_correctly_selects_new_location() {
    common::correctly_selects_new_location("add");
}

#[test]
fn add_only_appends_to_status() {
    common::only_appends_to_status("add");
}

#[test]
fn add_rejects_path_inside_library() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, place a good file in library
    common::make_tiny_library(&temp_dir);
    let demo_good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let good_file_path = temp_dir.path().join("library/01 Stereo.mp3");
    fs::copy(&demo_good_file_path, &good_file_path).expect("Failed to copy good file in to library");
    
    // Run add subcommand with path inside library
    common::run_musicl(&["add", temp_dir.path().join("library").to_str().expect("Failed to strify")], &temp_dir).failure();

    // Assert file not moved, status not changed
    assert!(good_file_path.is_file());
    common::assert_file_contents_eq(temp_dir.path().join(".musicl/status"), PathBuf::from("tests/data/tiny_library/.musicl/status"))
}

#[test]
fn add_rejects_path_inside_archive() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, place a good file in archive
    common::make_small_library(&temp_dir);
    let demo_good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let good_file_path = temp_dir.path().join("archive/01 Stereo.mp3");
    fs::copy(&demo_good_file_path, &good_file_path).expect("Failed to copy good file in to archive");
    
    // Run add subcommand with path inside archive
    common::run_musicl(&["add", temp_dir.path().join("archive").to_str().expect("Failed to strify")], &temp_dir).failure();

    // Assert file not moved, status not changed
    assert!(good_file_path.is_file());
    common::assert_file_contents_eq(temp_dir.path().join(".musicl/status"), PathBuf::from("tests/data/small_library/.musicl/status"))
}

#[test]
fn add_multiple_files_works() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, place some good files in a new directory at root
    common::make_tiny_library(&temp_dir);
    let demo_good_file1_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let demo_good_file2_path =  PathBuf::from("tests/data/correctly_located_file2/Fenne Lily/Hypochondriac/01 Hypochondriac.mp3");
    let good_file1_path = temp_dir.path().join("new/01 Stereo.mp3");
    let good_file2_path = temp_dir.path().join("new/02 Cut Your Hair.mp3");
    fs::create_dir(&temp_dir.path().join("new")).expect("Failed to create new directory");
    fs::copy(&demo_good_file1_path, &good_file1_path).expect("Failed to copy good file in to new directory");
    fs::copy(&demo_good_file2_path, &good_file2_path).expect("Failed to copy good file in to new directory");

    // Run add subcommand with wildcard
    common::run_musicl(&["add", temp_dir.path().join("new/*").to_str().expect("Failed to strify")], &temp_dir).success();

    // Assert both files moved, status updated
    assert!(!good_file1_path.is_file());
    assert!(!good_file2_path.is_file());
    common::assert_file_contents_ne(temp_dir.path().join(".musicl/status"), PathBuf::from("tests/data/tiny_library/.musicl/status"))
}

#[test]
fn add_moves_paired_lrc() {
    common::moves_paired_lrc("add");
}