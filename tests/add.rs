use std::{fs, path::PathBuf};
use chrono::Local;
use tempfile::tempdir;

use crate::common::all_but_last_line;

mod common;

#[test]
fn add_rejects_bad_metadata() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, place a bad file at root
    common::make_tiny_library(&temp_dir);
    let bad_metadata_path = PathBuf::from("tests/data/bad_metadata_file1/Parliament/Funkentelechy Vs. The Placebo Syndrome/1-03 Wizard Of Finance.mp3");
    fs::copy(&bad_metadata_path, &temp_dir).expect("Failed to copy bad file in to root");
    
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
    let temp_dir = tempdir().unwrap();

    // Create demo library, place a good file at root
    common::make_tiny_library(&temp_dir);
    let good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    fs::copy(&good_file_path, &temp_dir).expect("Failed to copy good file in to library");
    
    // Run add subcommand
    common::run_musicl_get_stdout(&["add", good_file_path.to_str().expect("Failed to strify")], &temp_dir);

    // Assert file moved, status changed
    assert!(!good_file_path.is_file());
    common::assert_file_contents_ne(temp_dir.path().join(".musicl/status"), PathBuf::from("tests/data/tiny_library/.musicl/status"))
}

#[test]
fn add_updates_status() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, place a good file at root
    common::make_tiny_library(&temp_dir);
    let good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    fs::copy(&good_file_path, &temp_dir.path().join("stereo.mp3")).expect("Failed to copy good file in to library");
    
    // Run add subcommand
    common::run_musicl_get_stdout(&["add", good_file_path.to_str().expect("Failed to strify")], &temp_dir);

    // Assert file moved, status changed
    let correct_status = format!("{},USMTD9719701,Add", Local::now().format("%Y-%m-%d"));
    assert!(!good_file_path.is_file());
    assert_eq!(common::last_line(&temp_dir.path().join(".musicl/status")), correct_status);
}

#[test]
fn add_creates_library_folder() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, place a good file at root
    common::make_empty_library(&temp_dir);
    let good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    fs::copy(&good_file_path, &temp_dir.path().join("stereo.mp3")).expect("Failed to copy good file in to library");
    
    // Run add subcommand
    common::run_musicl_get_stdout(&["add", good_file_path.to_str().expect("Failed to strify")], &temp_dir);

    // Assert library folder exists now
    assert!(temp_dir.path().join("library").is_dir());
}

#[test]
fn add_correctly_selects_new_location() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, place a good file at root
    common::make_tiny_library(&temp_dir);
    let good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    fs::copy(&good_file_path, &temp_dir.path().join("stereo.mp3")).expect("Failed to copy good file in to library");
    
    // Run add subcommand
    common::run_musicl_get_stdout(&["add", good_file_path.to_str().expect("Failed to strify")], &temp_dir);

    // Assert file moved, status changed
    let correct_location = temp_dir.path().join("library/Pavement/Brighten the Corners/01 Stereo.mp3");
    assert!(correct_location.is_file());
}

#[test]
fn add_only_appends_to_status() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, place a good file at root
    common::make_tiny_library(&temp_dir);
    let good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    fs::copy(&good_file_path, &temp_dir.path().join("stereo.mp3")).expect("Failed to copy good file in to library");
  
    // Run add subcommand
    common::run_musicl_get_stdout(&["add", good_file_path.to_str().expect("Failed to strify")], &temp_dir);

    // Assert file moved, status identical besides last line
    assert_eq!(fs::read_to_string("tests/data/tiny_library/.musicl/status").expect("Failed to read demo status"), all_but_last_line(&temp_dir.path().join(".musicl/status")));
}