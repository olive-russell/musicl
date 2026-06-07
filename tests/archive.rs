use std::{fs, path::PathBuf};

use tempfile::tempdir;


mod common;

#[test]
fn archive_rejects_non_music_file() {
    common::rejects_non_music_file("archive");
}

#[test]
fn archive_moves_good_file() {
    common::moves_good_file("archive");
}

#[test]
fn archive_updates_status() {
    common::updates_status("archive");
}

#[test]
fn archive_creates_archive_folder() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, place a good file in library
    common::make_empty_library(&temp_dir);
    let demo_good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let good_file_path = temp_dir.path().join("library/01 Stereo.mp3");
    fs::create_dir(temp_dir.path().join("library")).expect("Failed to create library directory");
    fs::copy(&demo_good_file_path, &good_file_path).expect("Failed to copy good file in to library");
    
    // Run archive subcommand
    common::run_musicl_get_stdout(&["archive", good_file_path.to_str().expect("Failed to strify")], &temp_dir);

    // Assert archive folder exists now
    assert!(temp_dir.path().join("archive").is_dir());
}

#[test]
fn archive_correctly_selects_new_location() {
    common::correctly_selects_new_location("archive");
}

#[test]
fn archive_only_appends_to_status() {
    common::only_appends_to_status("archive");
}

#[test]
fn archive_rejects_path_inside_archive() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, place a good file in archive
    common::make_small_library(&temp_dir);
    let demo_good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let good_file_path = temp_dir.path().join("archive/01 Stereo.mp3");
    fs::copy(&demo_good_file_path, &good_file_path).expect("Failed to copy good file in to archive");
    
    // Run archive subcommand with path inside archive
    common::run_musicl(&["archive", temp_dir.path().join("archive").to_str().expect("Failed to strify")], &temp_dir).failure();

    // Assert file not moved, status not changed
    assert!(good_file_path.is_file());
    common::assert_file_contents_eq(temp_dir.path().join(".musicl/status"), PathBuf::from("tests/data/small_library/.musicl/status"))
}

#[test]
fn archive_rejects_path_outside_library() {
    common::rejects_path_outside_library("archive");
}

#[test]
fn archive_rejects_path_at_root() {
    common::rejects_path_at_root("archive");
}

#[test]
fn archive_rejects_multiple_files() {
    common::rejects_multiple_files("archive");
}

#[test]
fn archive_moves_paired_lrc() {
    common::moves_paired_lrc("archive");
}