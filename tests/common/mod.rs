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

// pub fn make_empty_library(temp_dir: &TempDir) {
//     fs::copy("tests/data/tiny_library/.musicl", &temp_dir).expect("Failed to create demo .musicl directory.");
// }

pub fn make_tiny_library(temp_dir: &TempDir) {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/tiny_library", temp_dir.path(), &options).expect("Failed to create demo tiny library.");
}

// pub fn make_small_library(temp_dir: &TempDir) {
//     let mut options = dir::CopyOptions::new();
//     options.content_only = true;
//     dir::copy("tests/data/small_library", temp_dir.path(), &options).expect("Failed to create demo small library.");
// }
