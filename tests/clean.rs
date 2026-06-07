mod common;

use tempfile::tempdir;

#[test]
fn clean_removes_empty_directory() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, add some empty folders into the library folder
    common::make_tiny_library(&temp_dir);
    std::fs::create_dir(temp_dir.path().join("library/empty1")).expect("Failed to create empty directory");
    std::fs::create_dir(temp_dir.path().join("library/empty1/empty2")).expect("Failed to create empty directory");

    // Run clean subcommand
    common::run_musicl(&["clean"], &temp_dir).success();

    // Assert folders should no longer exist
    assert!(!temp_dir.path().join("library/empty1").is_dir());
    assert!(!temp_dir.path().join("library/empty1/empty2").is_dir());
}

#[test]
fn clean_keeps_non_empty_directory() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, add some empty folders into the library folder
    common::make_tiny_library(&temp_dir);
    std::fs::create_dir(temp_dir.path().join("library/notempty1")).expect("Failed to create directory");
    std::fs::create_dir(temp_dir.path().join("library/notempty1/notempty2")).expect("Failed to create directory");
    std::fs::File::create(temp_dir.path().join("library/notempty1/notempty2/content")).expect("Failed to create file");

    // Run clean subcommand
    common::run_musicl(&["clean"], &temp_dir).success();

    // Assert file and folders still exist
    assert!(temp_dir.path().join("library/notempty1/notempty2/").is_dir());
    assert!(temp_dir.path().join("library/notempty1/notempty2/content").is_file());
}

#[test]
fn clean_moves_incorrectly_located_file() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, add a correct file in
    common::make_tiny_library(&temp_dir);
    let correct_file1_path = common::copy_correctly_located_file1(&temp_dir, "library");

    // Rename otherwise correctly located file
    let mut incorrect_path = correct_file1_path.clone();
    incorrect_path.set_file_name("Incorrect Name.mp3");
    std::fs::rename(&correct_file1_path, &incorrect_path).expect("Failed to rename file in test.");

    // Run clean subcommand
    common::run_musicl(&["clean"], &temp_dir).success();
    
    // Assert file at incorrect place is gone
    assert!(!incorrect_path.is_file());

    // Assert file exists again
    assert!(correct_file1_path.is_file());
}

#[test]
fn clean_leaves_correctly_located_file() {
    let temp_dir = tempdir().unwrap();

    // Create demo library, add a correct file in
    common::make_tiny_library(&temp_dir);
    let correct_file1_path = common::copy_correctly_located_file1(&temp_dir, "library");

    // Run clean subcommand
    common::run_musicl(&["clean"], &temp_dir).success();

    // Assert file still exists
    assert!(correct_file1_path.is_file());
}