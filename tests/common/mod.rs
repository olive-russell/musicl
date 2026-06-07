#![allow(dead_code)]

use std::{fs, path::PathBuf};

use assert_cmd::{Command, assert::Assert};
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

pub fn make_empty_library(temp_dir: &TempDir) {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/empty_library", temp_dir.path(), &options).expect("Failed to create demo empty library.");
}

pub fn make_tiny_library(temp_dir: &TempDir) {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/tiny_library", temp_dir.path(), &options).expect("Failed to create demo tiny library.");
}

pub fn make_small_library(temp_dir: &TempDir) {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/small_library", temp_dir.path(), &options).expect("Failed to create demo small library.");
}

pub fn make_medium_library(temp_dir: &TempDir) {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/medium_library", temp_dir.path(), &options).expect("Failed to create demo medium library.");
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

pub fn copy_bad_metadata_file1(temp_dir: &TempDir, sub_library: &str) -> PathBuf {
    // Check path has the sub library
    let library_path = temp_dir.path().join(sub_library);
    if !library_path.exists() {
        println!("{sub_library} does not exist in test directory.");
        std::process::exit(1);
    }

    // Copy 
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/bad_metadata_file1", library_path, &options).expect("Failed to add file to sub library.");

    // Returne the hardcoded path to the mp3
    temp_dir.path().join("library").join("Parliament/Funkentelechy Vs. The Placebo Syndrome/1-03 Wizard Of Finance.mp3")
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
    run_musicl_get_stdout(&[subcommand, non_music_file_path.to_str().expect("Failed to strify")], &temp_dir);

    // Assert file not moved, status not changed
    assert!(non_music_file_path.is_file());
    assert_file_contents_eq(temp_dir.path().join(".musicl/status"), PathBuf::from("tests/data/small_library/.musicl/status"))
}