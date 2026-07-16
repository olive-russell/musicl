use std::{fs, io::Write};

use musicl::missing_metadata;
use tempfile::tempdir;

mod common;

#[test]
fn check_reports_incorrectly_located_file() {
    let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_tiny_library(&temp_dir);
    
    // Run check subcommand
    let lines_before = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();

    // Rename otherwise correctly located file
    let file_path = temp_dir.path().join("library/Los Felinos/A Bailar Country Rock/06 No Rompas Más Mi Pobre Corazón.mp3");
    let new_path = temp_dir.path().join("library/Los Felinos/A Bailar Country Rock/Incorrect name.mp3");
    std::fs::rename(file_path, new_path).expect("Failed to rename file in test.");

    // Run check subcommand
    let out_after = common::run_musicl_get_stdout(&["check"], &temp_dir);
    let lines_after = out_after.lines().count();
    println!("lines_after: {lines_after}");
    println!("stdout_after:\n{out_after}");

    // Assert file reported
    assert_eq!(lines_after - lines_before, 1);
}

#[test]
fn check_reports_file_not_in_status() {
    let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_tiny_library(&temp_dir);

    // Run check subcommand
    let lines_before = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();

    // Add file in without adding to status
    common::copy_correctly_located_file1(&temp_dir, "library");

    // Run check subcommand
    let lines_after = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();
    
    // Assert file reported
    assert_eq!(lines_after - lines_before, 1);
}

#[test]
fn check_reports_archived_file_not_in_status() {
    let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_small_library(&temp_dir);

    // Run check subcommand
    let lines_before = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();

    // Add file in without adding to status
    common::copy_correctly_located_file1(&temp_dir, "archive");

    // Run check subcommand
    let lines_after = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();
    
    // Assert file reported
    assert_eq!(lines_after - lines_before, 1);
}

#[test]
fn check_reports_missing_file() {
    let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_tiny_library(&temp_dir);

    // Run check subcommand
    let out_before = common::run_musicl_get_stdout(&["check"], &temp_dir);
    let lines_before = out_before.lines().count();
    println!("lines_before: {lines_before}");
    println!("stdout_before:\n{out_before}");

    // Add file to status
    let status_path = temp_dir.path().join(".musicl").join("status");
    let mut file = std::fs::OpenOptions::new().append(true).open(&status_path).expect("Failed to open status file");
    writeln!(file, "2026-06-06,USMTD9719701,add").expect("Failed to append to status file");
    
    // Run check subcommand
    let out_after = common::run_musicl_get_stdout(&["check"], &temp_dir);
    let lines_after = out_after.lines().count();
    println!("lines_after: {lines_after}");
    println!("stdout_after:\n{out_after}");

    // Assert file reported
    assert_eq!(lines_after - lines_before, 1);
}

#[test]
fn check_reports_missing_archive_file() {
    let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_tiny_library(&temp_dir);

    // Run check subcommand
    let out_before = common::run_musicl_get_stdout(&["check"], &temp_dir);
    let lines_before = out_before.lines().count();
    println!("lines_before: {lines_before}");
    println!("stdout_before:\n{out_before}");

    // Add file to status
    let status_path = temp_dir.path().join(".musicl").join("status");
    let mut file = std::fs::OpenOptions::new().append(true).open(&status_path).expect("Failed to open status file");
    writeln!(file, "2026-06-06,USMTD9719701,add").expect("Failed to append to status file for first line");
    writeln!(file, "2026-06-06,USMTD9719701,archive").expect("Failed to append to status file for second line");
    
    // Run check subcommand
    let out_after = common::run_musicl_get_stdout(&["check"], &temp_dir);
    let lines_after = out_after.lines().count();
    println!("lines_after: {lines_after}");
    println!("stdout_after:\n{out_after}");

    // Assert file reported
    assert_eq!(lines_after - lines_before, 1);
}

#[test]
fn check_reports_archive_file_in_library() {
    let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_small_library(&temp_dir);

    // Run check subcommand
    let out1 = common::run_musicl_get_stdout(&["check"], &temp_dir);
    let lines_before = out1.lines().count();
    println!("lines_before: {lines_before}");
    println!("stdout1:\n{out1}");

    // Add file to library, add archived status
    // common::copy_correctly_located_file1(&temp_dir, "library");
    let status_path = temp_dir.path().join(".musicl").join("status");
    let mut file = std::fs::OpenOptions::new().append(true).open(&status_path).expect("Failed to open status file");
    writeln!(file, "2026-06-06,USWB10502859,archive").expect("Failed to append to status file");
    
    // Run check subcommand
    let out2 = common::run_musicl_get_stdout(&["check"], &temp_dir);
    let lines_after = out2.lines().count();
    println!("lines_before: {lines_before}, lines_after: {lines_after}");
    println!("stdout2:\n{out2}");
    // Assert file reported
    assert_eq!(lines_after - lines_before, 1);
}

#[test]
fn check_reports_library_file_in_archive() {
    let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_small_library(&temp_dir);

    // Run check subcommand
    let lines_before = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();

    // Add file to archive, add library status
    common::copy_correctly_located_file1(&temp_dir, "archive");
    let status_path = temp_dir.path().join(".musicl").join("status");
    let mut file = std::fs::OpenOptions::new().append(true).open(&status_path).expect("Failed to open status file");
    writeln!(file, "2026-06-06,USMTD9719701,add").expect("Failed to append to status file");
    
    // Run check subcommand
    let lines_after = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();
    
    // Assert file reported
    assert_eq!(lines_after - lines_before, 1);
}

#[test]
fn check_reports_bad_metadata() {
    let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_small_library(&temp_dir);

    // Run check subcommand
    let out_before = common::run_musicl_get_stdout(&["check"], &temp_dir);
    let lines_before = out_before.lines().count();
    println!("lines_before: {lines_before}");
    println!("stdout_before:\n{out_before}");

    // Copy bad file into library and update status
    let bad_metadata_file_path = "tests/data/bad_metadata_file1/Parliament/Funkentelechy Vs. The Placebo Syndrome/1-03 Wizard Of Finance.mp3";
    let destination = temp_dir.path().join("library/Parliament/Funkentelechy Vs. The Placebo Syndrome/1-03 Wizard Of Finance.mp3");
    let parent_dir = destination.parent().expect("Failed to get parent directory for bad metadata file");
    std::fs::create_dir_all(&parent_dir).expect("Failed to create parent directories for bad metadata file");
    std::fs::copy(bad_metadata_file_path, &destination).expect("Failed to copy bad metadata file");
    let status_path = temp_dir.path().join(".musicl").join("status");
    let mut file = std::fs::OpenOptions::new().append(true).open(&status_path).expect("Failed to open status file");
    writeln!(file, "2026-06-06,USWWW0125792,add").expect("Failed to append to status file");
    
    let missing = missing_metadata(&destination);
    println!("Missing metadata: {}", missing.join(", "));
    println!("Missing metadata count: {}", missing.len());
    
    // Run check subcommand
    let out_after = common::run_musicl_get_stdout(&["check"], &temp_dir);
    let lines_after = out_after.lines().count();
    println!("lines_after: {lines_after}");
    println!("stdout_after:\n{out_after}");

    // Assert file reported
    assert!(lines_after - lines_before > 0);
}

#[test]
fn check_reports_orphaned_lrc() {
    let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_small_library(&temp_dir);

    // Run check subcommand
    let out_before = common::run_musicl_get_stdout(&["check"], &temp_dir);
    let lines_before = out_before.lines().count();
    println!("lines_before: {lines_before}");
    println!("stdout_before:\n{out_before}");

    // Add lrc file without mp3 file
    let lrc_file_path = temp_dir.path().join("library/Fiona Apple/Tidal/06 The First Taste.lrc");
    fs::create_dir_all(lrc_file_path.parent().expect("Failed to get parent directory")).expect("Failed to create directory for lrc file");
    fs::File::create(&lrc_file_path).expect("Failed to create lrc file");

    // Run check subcommand
    let out_after = common::run_musicl_get_stdout(&["check"], &temp_dir);
    let lines_after = out_after.lines().count();
    println!("lines_after: {lines_after}");
    println!("stdout_after:\n{out_after}");
    
    // Assert file reported
    assert_eq!(lines_after - lines_before, 1);
}

#[test]
fn check_doesnt_report_removed_files() {
        let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_small_library(&temp_dir);

    // Run check subcommand
    let out_before = common::run_musicl_get_stdout(&["check"], &temp_dir);
    let lines_before = out_before.lines().count();
    println!("lines_before: {lines_before}");
    println!("stdout_before:\n{out_before}");

    // Remove file, update status
    let path_to_remove = temp_dir.path().join("library/Talking Heads/Naked/03 Totally Nude.mp3");
    std::fs::remove_file(path_to_remove).expect("Failed to remove file");
    let status_path = temp_dir.path().join(".musicl").join("status");
    let mut file = std::fs::OpenOptions::new().append(true).open(&status_path).expect("Failed to open status file");
    writeln!(file, "2026-06-06,GB01A0500050,remove").expect("Failed to append to status file");
    

    // Run check subcommand
    let out_after = common::run_musicl_get_stdout(&["check"], &temp_dir);
    let lines_after = out_after.lines().count();
    println!("lines_after: {lines_after}");
    println!("stdout_after:\n{out_after}");
    
    // Assert file reported
    assert_eq!(lines_after - lines_before, 0);
}