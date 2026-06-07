use std::{fs, io::Write};

use tempfile::tempdir;

mod common;

#[test]
fn check_reports_incorrectly_located_file() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, add a correct file in
    common::make_tiny_library(&temp_dir);
    let correct_file1_path = common::copy_correctly_located_file1(&temp_dir, "library");

    // Add file to status
    let status_path = temp_dir.path().join(".musicl").join("status");
    let mut file = std::fs::OpenOptions::new().append(true).open(&status_path).expect("Failed to open status file");
    writeln!(file, "2026-06-06,USMTD9719701,Add").expect("Failed to append to status file");
    
    // Run check subcommand
    let lines_before = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();

    // Rename otherwise correctly located file
    let mut incorrect_path = correct_file1_path.clone();
    incorrect_path.set_file_name("Incorrect Name.mp3");
    std::fs::rename(&correct_file1_path, &incorrect_path).expect("Failed to rename file in test.");

    // Run check subcommand
    let lines_after = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();
    
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
    let lines_before = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();

    // Add file to status
    let status_path = temp_dir.path().join(".musicl").join("status");
    let mut file = std::fs::OpenOptions::new().append(true).open(&status_path).expect("Failed to open status file");
    writeln!(file, "2026-06-06,USMTD9719701,Add").expect("Failed to append to status file");
    
    // Run check subcommand
    let lines_after = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();
    
    // Assert file reported
    assert_eq!(lines_after - lines_before, 1);
}

#[test]
fn check_reports_missing_archive_file() {
    let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_tiny_library(&temp_dir);

    // Run check subcommand
    let lines_before = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();

    // Add file to status
    let status_path = temp_dir.path().join(".musicl").join("status");
    let mut file = std::fs::OpenOptions::new().append(true).open(&status_path).expect("Failed to open status file");
    writeln!(file, "2026-06-06,USMTD9719701,Add").expect("Failed to append to status file for first line");
    writeln!(file, "2026-06-06,USMTD9719701,Archive").expect("Failed to append to status file for second line");
    
    // Run check subcommand
    let lines_after = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();
    
    // Assert file reported
    assert_eq!(lines_after - lines_before, 1);
}

#[test]
fn check_reports_archive_file_in_library() {
    let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_small_library(&temp_dir);

    // Run check subcommand
    let lines_before = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();

    // Add file to library, add archived status
    common::copy_correctly_located_file1(&temp_dir, "library");
    let status_path = temp_dir.path().join(".musicl").join("status");
    let mut file = std::fs::OpenOptions::new().append(true).open(&status_path).expect("Failed to open status file");
    writeln!(file, "2026-06-06,USMTD9719701,Add").expect("Failed to append to status file for first line");
    writeln!(file, "2026-06-06,USMTD9719701,Archive").expect("Failed to append to status file for second line");
    
    // Run check subcommand
    let lines_after = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();
    
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
    writeln!(file, "2026-06-06,USMTD9719701,Add").expect("Failed to append to status file");
    
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
    let lines_before = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();

    // Add file to archive, add library status
    common::copy_bad_metadata_file1(&temp_dir, "archive");
    let status_path = temp_dir.path().join(".musicl").join("status");
    let mut file = std::fs::OpenOptions::new().append(true).open(&status_path).expect("Failed to open status file");
    writeln!(file, "2026-06-06,USWWW0125792,Add").expect("Failed to append to status file");
    
    // Run check subcommand
    let lines_after = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();
    
    // Assert file reported
    assert!(lines_after - lines_before > 0);
}

#[test]
fn check_reports_orphaned_lrc() {
    let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_small_library(&temp_dir);

    // Run check subcommand
    let lines_before = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();

    // Add lrc file without mp3 file
    let lrc_file_path = temp_dir.path().join("library/Fiona Apple/Tidal/06 The First Taste.lrc");
    fs::create_dir_all(lrc_file_path.parent().expect("Failed to get parent directory")).expect("Failed to create directory for lrc file");
    fs::File::create(&lrc_file_path).expect("Failed to create lrc file");

    // Run check subcommand
    let lines_after = common::run_musicl_get_stdout(&["check"], &temp_dir).lines().count();
    
    // Assert file reported
    assert_eq!(lines_after - lines_before, 1);
}