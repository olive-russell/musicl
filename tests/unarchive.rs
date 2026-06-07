use std::{fs, path::PathBuf};

use tempfile::tempdir;


mod common;

#[test]
fn unarchive_rejects_non_music_file() {
    common::rejects_non_music_file("unarchive");
}

#[test]
fn unarchive_moves_good_file() {
    common::moves_good_file("unarchive");
}

#[test]
fn unarchive_updates_status() {
    common::updates_status("unarchive");
}

#[test]
fn unarchive_correctly_selects_new_location() {
    common::correctly_selects_new_location("unarchive");
}

#[test]
fn unarchive_only_appends_to_status() {
    common::only_appends_to_status("unarchive");
}

#[test]
fn unarchive_rejects_path_inside_library() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, place a good file in library
    common::make_small_library(&temp_dir);
    let demo_good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let good_file_path = temp_dir.path().join("library/01 Stereo.mp3");
    fs::copy(&demo_good_file_path, &good_file_path).expect("Failed to copy good file in to library");
    
    // Run unarchive subcommand with path inside library
    common::run_musicl(&["unarchive", good_file_path.to_str().expect("Failed to strify")], &temp_dir).failure();

    // Assert file not moved, status not changed
    assert!(good_file_path.is_file());
    common::assert_file_contents_eq(temp_dir.path().join(".musicl/status"), PathBuf::from("tests/data/small_library/.musicl/status"))
}

#[test]
fn unarchive_rejects_path_outside_library() {
    common::rejects_path_outside_library("unarchive");
}

#[test]
fn unarchive_rejects_path_at_root() {
    common::rejects_path_at_root("unarchive");
}

#[test]
fn unarchive_rejects_multiple_files() {
    common::rejects_multiple_files("unarchive");
}