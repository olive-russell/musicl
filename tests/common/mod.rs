#![allow(dead_code)]

use std::{fs, path::PathBuf};

use assert_cmd::{Command, assert::Assert};
use chrono::Local;
use fs_extra::dir;
use tempfile::{TempDir, tempdir};

pub fn run_musicl(args: &[&str], temp_dir: &TempDir) -> Assert {
    Command::cargo_bin("musicl")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(args)
        .assert()
}

pub fn run_musicl_get_stdout(args: &[&str], temp_dir: &TempDir) -> String {
    let output = Command::cargo_bin("musicl")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(args)
        .output()
        .expect("Command failed to run");

    String::from_utf8(output.stdout).expect("Failed to parse output to string")
}

pub fn copy_correctly_located_file1(temp_dir: &TempDir, sub_library: &str) -> PathBuf {
    // Check path has the sub library
    let library_path = temp_dir.path().join(sub_library);
    if !library_path.exists() {
        print!("Library does not exist in test directory.");
        std::process::exit(1);
    }

    // Copy 
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/correctly_located_file1", library_path, &options).expect("Failed to add file to sub library.");

    // Returne the hardcoded path to the mp3
    temp_dir.path().join("library").join("Pavement/Brighten the Corners/01 Stereo.mp3")
}

pub fn make_empty_library(temp_dir: &TempDir) {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/empty_library", temp_dir.path(), &options).expect("Failed to create demo empty library.");
}

// Make sure you update this function in tests.rs too
pub fn make_tiny_library(temp_dir: &TempDir) {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/tiny_library", temp_dir.path(), &options).expect("Failed to create demo tiny library.");
}

// Make sure you update this function in tests.rs too
pub fn make_small_library(temp_dir: &TempDir) {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/small_library", temp_dir.path(), &options).expect("Failed to create demo small library.");
}

// Make sure you update this function in tests.rs too
pub fn make_medium_library(temp_dir: &TempDir) {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/medium_library", temp_dir.path(), &options).expect("Failed to create demo medium library.");
}

pub fn assert_file_contents_eq(path1: PathBuf, path2: PathBuf) {
    let actual = fs::read_to_string(path1).expect("Couldn't read file at path1.");
    let expected = fs::read_to_string(path2).expect("Couldn't read file at path2");
    assert_eq!(actual, expected);
}

pub fn assert_file_contents_ne(path1: PathBuf, path2: PathBuf) {
    let actual = fs::read_to_string(path1).expect("Couldn't read file at path1.");
    let expected = fs::read_to_string(path2).expect("Couldn't read file at path2");
    assert_ne!(actual, expected);
}

pub fn last_line(path: &PathBuf) -> String {
    fs::read_to_string(path)
        .expect("Failed to read file")
        .lines()
        .last()
        .unwrap_or("")
        .to_string()
}

pub fn all_lines(path: &PathBuf) -> String {
    let content = fs::read_to_string(path)
        .expect("Failed to read file");

    let lines: Vec<_> = content.lines().collect();
    lines.join("\n")
}

pub fn all_but_last_line(path: &PathBuf) -> String {
    let content = fs::read_to_string(path)
        .expect("Failed to read file");

    let mut lines: Vec<_> = content.lines().collect();
    lines.pop();
    lines.join("\n")
}

pub fn rejects_non_music_file(subcommand: &str) {
    let temp_dir = tempdir().unwrap();

    // Determine where to place file
    let destination = match subcommand {
        "add" => PathBuf::from(temp_dir.path()),
        "archive" => temp_dir.path().join("library"),
        "unarchive" => temp_dir.path().join("archive"),
        "remove" => temp_dir.path().join("library"),
        _ => panic!("Unimplemented sub-command"),
    };

    // Create demo library, place a bad file at root
    make_small_library(&temp_dir);
    let demo_non_music_file_path = PathBuf::from("tests/data/non_music_file.txt");
    let non_music_file_path = destination.join("non_music_file.txt");
    fs::copy(&demo_non_music_file_path, &non_music_file_path).expect("Failed to copy non-music file file in to root");
    
    // Run add subcommand
    let result = run_musicl(&[subcommand, non_music_file_path.to_str().expect("Failed to strify")], &temp_dir);
    if subcommand == "add" {
        result.success();
    } else {
        result.failure();
    }

    // Assert file not moved, status not changed
    assert!(non_music_file_path.is_file());
    assert_file_contents_eq(temp_dir.path().join(".musicl/status"), PathBuf::from("tests/data/small_library/.musicl/status"))
}

pub fn moves_good_file(subcommand: &str) {
    let temp_dir = tempdir().unwrap();

    // Determine where to place file
    let destination = match subcommand {
        "add" => PathBuf::from(temp_dir.path()),
        "archive" => temp_dir.path().join("library"),
        "unarchive" => temp_dir.path().join("archive"),
        "remove" => temp_dir.path().join("library"),
        _ => panic!("Unimplemented sub-command"),
    };

    // Create demo library, place a good file at root
    make_small_library(&temp_dir);
    let demo_good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let good_file_path = destination.join("01 Stereo.mp3");
    fs::copy(&demo_good_file_path, &good_file_path).expect("Failed to copy good file in to library");
    
    // Run add subcommand
    run_musicl(&[subcommand, good_file_path.to_str().expect("Failed to strify")], &temp_dir).success();

    // Assert file moved, status changed
    assert!(!good_file_path.is_file());
    assert_file_contents_ne(temp_dir.path().join(".musicl/status"), PathBuf::from("tests/data/small_library/.musicl/status"))
}

pub fn updates_status(subcommand: &str) {
    let temp_dir = tempdir().unwrap();

    // Determine where to place file
    let destination = match subcommand {
        "add" => PathBuf::from(temp_dir.path()),
        "archive" => temp_dir.path().join("library"),
        "unarchive" => temp_dir.path().join("archive"),
        "remove" => temp_dir.path().join("library"),
        _ => panic!("Unimplemented sub-command"),
    };

    // Create demo library, place a good file at root
    make_small_library(&temp_dir);
    let demo_good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let good_file_path = destination.join("01 Stereo.mp3");
    fs::copy(&demo_good_file_path, &good_file_path).expect("Failed to copy good file in to library");
    
    // Run add subcommand
    run_musicl(&[subcommand, good_file_path.to_str().expect("Failed to strify")], &temp_dir).success();

    // Assert file moved, status changed
    let correct_status = format!("{},USMTD9719701,{}", Local::now().format("%Y-%m-%d"), subcommand);
    assert!(!good_file_path.is_file());
    assert_eq!(last_line(&temp_dir.path().join(".musicl/status")), correct_status);
}

pub fn correctly_selects_new_location(subcommand: &str) {
    let temp_dir = tempdir().unwrap();

    // Determine where to place file
    let destination = match subcommand {
        "add" => PathBuf::from(temp_dir.path()),
        "archive" => temp_dir.path().join("library"),
        "unarchive" => temp_dir.path().join("archive"),
        _ => panic!("Unimplemented sub-command"),
    };

    // Create demo library, place a good file at root
    make_small_library(&temp_dir);
    let demo_good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let good_file_path = destination.join("01 Stereo.mp3");
    fs::copy(&demo_good_file_path, &good_file_path).expect("Failed to copy good file in to library");
    
    // Run subcommand
    run_musicl(&[subcommand, good_file_path.to_str().expect("Failed to strify")], &temp_dir).success();

    // Determine correct location
    let correct_location = match subcommand {
        "add" => "library",
        "archive" => "archive",
        "unarchive" => "library",
        _ => panic!("Unimplemented sub-command"),
    };

    // Assert file moved
    let correct_location = temp_dir.path().join(format!("{}/Pavement/Brighten the Corners/01 Stereo.mp3", correct_location));
    assert!(correct_location.is_file());
}

pub fn only_appends_to_status(subcommand: &str) {
    let temp_dir = tempdir().unwrap();

    // Determine where to place file
    let destination = match subcommand {
        "add" => PathBuf::from(temp_dir.path()),
        "archive" => temp_dir.path().join("library"),
        "unarchive" => temp_dir.path().join("archive"),
        "remove" => temp_dir.path().join("library"),
        _ => panic!("Unimplemented sub-command"),
    };

    // Create demo library, place a good file at root
    make_small_library(&temp_dir);
    let demo_good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let good_file_path = destination.join("01 Stereo.mp3");
    fs::copy(&demo_good_file_path, &good_file_path).expect("Failed to copy good file in to library");

    // Run subcommand
    run_musicl(&[subcommand, good_file_path.to_str().expect("Failed to strify")], &temp_dir).success();

    // Assert file moved, status identical besides last line
    assert_eq!(all_but_last_line(&temp_dir.path().join(".musicl/status")), all_lines(&PathBuf::from("tests/data/small_library/.musicl/status")));
}

pub fn rejects_path_outside_library(subcommand: &str) {
    let temp_dir = tempdir().unwrap();
    let temp_dir2 = tempdir().unwrap();

    // Create demo library, place a good file at root of different temp dir
    make_small_library(&temp_dir);
    let demo_good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let good_file_path = temp_dir2.path().join("01 Stereo.mp3");
    fs::copy(&demo_good_file_path, &good_file_path).expect("Failed to copy good file in to library");
    
    // Run subcommand with path outside library
    run_musicl(&[subcommand, good_file_path.to_str().expect("Failed to strify")], &temp_dir).failure();

    // Assert file not moved, status not changed
    assert!(good_file_path.is_file());
    assert_file_contents_eq(temp_dir.path().join(".musicl/status"), PathBuf::from("tests/data/small_library/.musicl/status"))
}

pub fn rejects_path_at_root(subcommand: &str) {
    let temp_dir = tempdir().unwrap();

    // Create demo library, place a good file at root
    make_small_library(&temp_dir);
    let demo_good_file_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let good_file_path = temp_dir.path().join("01 Stereo.mp3");
    fs::copy(&demo_good_file_path, &good_file_path).expect("Failed to copy good file in to library");
    
    // Run subcommand with path at root
    run_musicl(&[subcommand, good_file_path.to_str().expect("Failed to strify")], &temp_dir).failure();

    // Assert file not moved, status not changed
    assert!(good_file_path.is_file());
    assert_file_contents_eq(temp_dir.path().join(".musicl/status"), PathBuf::from("tests/data/small_library/.musicl/status"))
}

pub fn rejects_multiple_files(subcommand: &str) {
    let temp_dir = tempdir().unwrap();

    // Determine where to place files
    let destination = match subcommand {
        "archive" => temp_dir.path().join("library"),
        "unarchive" => temp_dir.path().join("archive"),
        "remove" => temp_dir.path().join("archive"),
        _ => panic!("Unimplemented sub-command"),
    };

    // Create demo library, place some good files in a new directory at root
    make_small_library(&temp_dir);
    let demo_good_file1_path =  PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let demo_good_file2_path =  PathBuf::from("tests/data/correctly_located_file2/Fenne Lily/Hypochondriac/01 Hypochondriac.mp3");
    let good_file1_path = destination.join("new/01 Stereo.mp3");
    let good_file2_path = destination.join("new/02 Cut Your Hair.mp3");
    fs::create_dir(&destination.join("new")).expect("Failed to create new directory");
    fs::copy(&demo_good_file1_path, &good_file1_path).expect("Failed to copy good file in to new directory");
    fs::copy(&demo_good_file2_path, &good_file2_path).expect("Failed to copy good file in to new directory");

    // Run subcommand with wildcard
    run_musicl(&[subcommand, destination.join("new/*").to_str().expect("Failed to strify")], &temp_dir).failure();

    // Assert both files rejected, status not updated
    assert!(good_file1_path.is_file());
    assert!(good_file2_path.is_file());
    assert_file_contents_eq(temp_dir.path().join(".musicl/status"), PathBuf::from("tests/data/small_library/.musicl/status"))
}

pub fn moves_paired_lrc(subcommand: &str) {
    let temp_dir = tempdir().unwrap();

    // Determine where to place file
    let destination = match subcommand {
        "add" => PathBuf::from(temp_dir.path()),
        "archive" => temp_dir.path().join("library"),
        "unarchive" => temp_dir.path().join("archive"),
        "remove" => temp_dir.path().join("library"),
        _ => panic!("Unimplemented sub-command"),
    };

    // Create demo library, place a file with paired lrc
    make_small_library(&temp_dir);
    let demo_folder_path = PathBuf::from("tests/data/file_with_paired_lrc");
    let good_file_path = destination.join("Fiona Apple/Tidal/06 The First Taste.mp3");
    let lrc_file_path = destination.join("Fiona Apple/Tidal/06 The First Taste.lrc");
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy(&demo_folder_path, destination, &options).expect("Failed to add file to sub library.");

    // Run subcommand
    run_musicl(&[subcommand, good_file_path.to_str().expect("Failed to strify")], &temp_dir).success();
    
    // Assert lrc file moved with mp3 file
    assert!(!lrc_file_path.is_file());
    
    // Check lyric file moved
    if subcommand != "remove" {
        // Determine correct location
        let correct_location = match subcommand {
            "add" => "library",
            "archive" => "archive",
            "unarchive" => "library",
            _ => panic!("Unimplemented sub-command"),
        };
        let correct_lrc_location = temp_dir.path().join(format!("{}/Fiona Apple/Tidal/06 The First Taste.lrc", correct_location));
        assert!(correct_lrc_location.is_file());
    }
}