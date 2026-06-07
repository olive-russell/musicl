#![allow(dead_code)]

use std::path::PathBuf;

use assert_cmd::{Command, assert::Assert};
use fs_extra::dir;
use tempfile::TempDir;

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

// pub fn make_empty_library(temp_dir: &TempDir) {
//     fs::copy("tests/data/tiny_library/.musicl", &temp_dir).expect("Failed to create demo .musicl directory.");
// }

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
        print!("Library does not exist in test directory.");
        std::process::exit(1);
    }

    // Copy 
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/bad_metadata_file1", library_path, &options).expect("Failed to add file to sub library.");

    // Returne the hardcoded path to the mp3
    temp_dir.path().join("library").join("Parliament/Funkentelechy Vs. The Placebo Syndrome/1-03 Wizard Of Finance.mp3")
}