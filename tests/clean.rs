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

    // Rename otherwise correctly located file
    let file = temp_dir.path().join("library/Los Felinos/A Bailar Country Rock/06 No Rompas Más Mi Pobre Corazón.mp3");
    let new_path = temp_dir.path().join("library/Los Felinos/A Bailar Country Rock/bad file name.mp3");
    std::fs::rename(&file, &new_path).expect("Failed to rename file in test.");

    // Run clean subcommand
    common::run_musicl(&["clean"], &temp_dir).success();
    
    // Assert file at incorrect place is gone
    assert!(!new_path.is_file());

    // Assert file exists again
    assert!(file.is_file());
}

#[test]
fn clean_leaves_correctly_located_file() {
    let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_small_library(&temp_dir);

    // Run clean subcommand
    common::run_musicl(&["clean"], &temp_dir).success();

    // Assert file still exists
    assert!(temp_dir.path().join("library/Talking Heads/Naked/05 (Nothing But) Flowers.mp3").exists());
    assert!(temp_dir.path().join("archive/Jacques Dutronc/Best Of Jacques Dutronc/05 Les gens sont fous, les temps sont flous (Remastered).mp3").exists());
}

#[test]
fn clean_survives_bad_metadata() {
        let temp_dir = tempdir().unwrap();

    // Create demo library
    common::make_small_library(&temp_dir);

    // Add file with bad metadata
    let bad_metadata_file_path = "tests/data/bad_metadata_file1/Parliament/Funkentelechy Vs. The Placebo Syndrome/1-03 Wizard Of Finance.mp3";
    let destination = temp_dir.path().join("library/Parliament/Funkentelechy Vs. The Placebo Syndrome/1-03 Wizard Of Finance.mp3");
    let parent_dir = destination.parent().expect("Failed to get parent directory for bad metadata file");
    std::fs::create_dir_all(&parent_dir).expect("Failed to create parent directories for bad metadata file");
    std::fs::copy(bad_metadata_file_path, &destination).expect("Failed to copy bad metadata file");

    // Run clean subcommand
    common::run_musicl(&["clean"], &temp_dir).success();
}