use std::{env, path::Path};

use fs_extra::dir;
use tempfile::{TempDir, tempdir};

use super::*;

// AI written struct to restore working directory after test
struct CurrentDirGuard {
    previous: PathBuf,
}

// As above
impl CurrentDirGuard {
    fn new<P: AsRef<Path>>(new_dir: P) -> std::io::Result<Self> {
        let previous = env::current_dir()?;
        env::set_current_dir(new_dir)?;
        Ok(Self { previous })
    }
}

// As above
impl Drop for CurrentDirGuard {
    fn drop(&mut self) {
        let _ = env::set_current_dir(&self.previous);
    }
}

// Copied from mod.rs
pub fn make_tiny_library(temp_dir: &TempDir) {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/tiny_library", temp_dir.path(), &options).expect("Failed to create demo tiny library.");
}

// Copied from mod.rs
pub fn make_small_library(temp_dir: &TempDir) {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/small_library", temp_dir.path(), &options).expect("Failed to create demo small library.");
}

// Copied from mod.rs
pub fn make_medium_library(temp_dir: &TempDir) {
    let mut options = dir::CopyOptions::new();
    options.content_only = true;
    dir::copy("tests/data/medium_library", temp_dir.path(), &options).expect("Failed to create demo medium library.");
}

#[test]
fn test_in_library() {
    let ok1 = in_library(&std::env::current_dir().unwrap());
    assert!(ok1.is_ok());
    assert_eq!(ok1.unwrap(), false);
    let ok2 = in_library(&std::env::current_dir().unwrap().join("library").join("thing"));
    assert!(ok2.is_ok());
    assert_eq!(ok2.unwrap(), true);
}

#[test]
fn test_in_archive() {
    let ok1 = in_archive(&std::env::current_dir().unwrap());
    assert!(ok1.is_ok());
    assert_eq!(ok1.unwrap(), false);
    let ok2 = in_archive(&std::env::current_dir().unwrap().join("archive").join("thing"));
    assert!(ok2.is_ok());
    assert_eq!(ok2.unwrap(), true);
}

#[test]
fn test_library_path() {
    println!("{}", library_path().unwrap().display());
    assert!(library_path().is_ok());
}

#[test]
fn test_archive_path() {
    println!("{}", archive_path().unwrap().display());
    assert!(archive_path().is_ok());
}

#[test]
fn test_status_path() {
    println!("{}", status_path().unwrap().display());
    assert!(status_path().is_ok());
}

#[test]
fn test_is_music_file() {
    let yes = is_music_file(&PathBuf::from("file.mp3"));
    assert!(yes);
    let no = is_music_file(&PathBuf::from("file.txt"));
    assert!(!no);
}

#[test]
fn test_valid_music_files() {
    let valid = valid_music_files();
    assert!(valid.contains(&"mp3"));
    assert!(!valid.contains(&"txt"));
}

#[test]
fn test_isrc_in_use() {
    let temp_dir = tempdir().unwrap();
    let old_dir = env::current_dir().unwrap();
    make_tiny_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();

    // Fails on non-existent file
    let bad1 = isrc_in_use(&PathBuf::from("fake/path.mp3"));
    assert!(bad1.is_err());
    
    // False when not used
    let false1 = isrc_in_use(&old_dir.join("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3"));
    assert!(false1.is_ok());
    assert!(!false1.unwrap());

    // True when used
    let good1 = isrc_in_use(&old_dir.join("tests/data/tiny_library/library/Los Felinos/A Bailar Country Rock/06 No Rompas Más Mi Pobre Corazón.mp3"));
    assert!(good1.is_ok());
    assert!(good1.unwrap());
}

#[test]
fn test_find_current_status() {
    let temp_dir = tempdir().unwrap();
    make_tiny_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();
    
    // None when not in library
    let false1 = find_current_status(&String::from("US0000000000"));
    assert!(false1.is_ok());
    assert!(false1.unwrap().is_none());

    // Status line when used
    let good1 = find_current_status(&String::from("MXF369733932"));
    assert!(good1.is_ok());
    let unwrapped_good1 = good1.unwrap();
    assert!(unwrapped_good1.is_some());
    assert_eq!(unwrapped_good1.unwrap().isrc, "MXF369733932");

}

#[test]
fn test_has_correct_location() {
    let temp_dir = tempdir().unwrap();
    make_tiny_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();

    // True when agrees with get_correct_location
    let good_path1 = PathBuf::from("library/Los Felinos/A Bailar Country Rock/06 No Rompas Más Mi Pobre Corazón.mp3");
    let good1 = has_correct_location(&good_path1);
    assert!(good1.is_ok());
    assert!(good1.unwrap());

    // False when correct file is moved to wrong location
    let wrong_location = PathBuf::from("library/Los Felinos/A Bailar Country Rock/06 No Rompas Más Mi Pobre Corazón (wrong location).mp3");
    std::fs::rename(&good_path1, &wrong_location).expect("Failed to move file to wrong location.");
    let bad1 = has_correct_location(&wrong_location);
    assert!(bad1.is_ok());
    assert!(!bad1.unwrap());
}

#[test]
fn test_get_correct_location() {
    let temp_dir = tempdir().unwrap();
    make_tiny_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();

    // Correct location for a file in the library (use absolute path)
    let good_path1 = std::fs::canonicalize(temp_dir.path().join("library/Los Felinos/A Bailar Country Rock/06 No Rompas Más Mi Pobre Corazón.mp3")).unwrap();
    let correct1 = get_correct_location(&good_path1);
    assert!(correct1.is_ok());
    assert_eq!(correct1.unwrap(), good_path1);
}

#[test]
fn test_move_music() {
    let temp_dir = tempdir().unwrap();
    let old_dir = env::current_dir().unwrap();
    make_tiny_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();

    // Copy file to root and run move_music, assert file is moved to correct location and status is updated
    let source_path = old_dir.join("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let new_path = temp_dir.path().join("01 Stereo.mp3");
    std::fs::copy(&source_path, &new_path).expect("Failed to copy file to root.");
    let move_result = move_music(&new_path, "add");
    assert!(move_result.is_ok());
    assert!(!new_path.exists());
}

#[test]
fn test_sublibrary_from_action() {
    let add = sublibrary_from_action("add");
    assert!(add.is_ok());
    assert_eq!(add.unwrap(), library_path().unwrap());
    let archive = sublibrary_from_action("archive");
    assert!(archive.is_ok());
    assert_eq!(archive .unwrap(), archive_path().unwrap());
    let unarchive = sublibrary_from_action("unarchive");
    assert!(unarchive.is_ok());
    assert_eq!(unarchive.unwrap(), library_path().unwrap());
    let remove = sublibrary_from_action("remove");
    assert!(!remove.is_ok());
}

#[test]
fn test_move_to_correct_location() {
    let temp_dir = tempdir().unwrap();
    let old_dir = env::current_dir().unwrap();
    make_tiny_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();

    // Move file to root
    let source_path = old_dir.join("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let new_path = temp_dir.path().join("01 Stereo.mp3");
    std::fs::copy(&source_path, &new_path).expect("Failed to copy file to root.");

    // Append line to status file
    let isrc = get_isrc(&new_path).unwrap().unwrap();
    let date = Local::now().format("%Y-%m-%d").to_string();
    write_status(date, &isrc, "add").expect("Failed to write status.");

    // Check location
    let correct_location = PathBuf::from("library/Pavement/Brighten the Corners/01 Stereo.mp3");
    let move_result = move_to_correct_location(&new_path);
    assert!(move_result.is_ok());
    assert!(!new_path.exists());
    assert!(correct_location.exists());
}

#[test]
fn test_remove_music() {
    let temp_dir = tempdir().unwrap();
    make_tiny_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();

    // Remove No Rompas Más Mi Pobre Corazón.mp3 from library and check that it is removed
    let path_to_remove = PathBuf::from("library/Los Felinos/A Bailar Country Rock/06 No Rompas Más Mi Pobre Corazón.mp3");
    let isrc = get_isrc(&path_to_remove).unwrap().unwrap();
    let remove_result = remove_music(&path_to_remove);
    assert!(remove_result.is_ok());
    assert!(!path_to_remove.exists());

    // Check that status is updated
    let status = find_current_status(&isrc).unwrap();
    assert!(status.is_some());
    assert_eq!(status.unwrap().action, "remove");
}

#[test]
fn test_remove_file() {
    let temp_dir = tempdir().unwrap();
    make_tiny_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();

    // Remove No Rompas Más Mi Pobre Corazón.mp3 from library and check that it is removed
    let path_to_remove = PathBuf::from("library/Los Felinos/A Bailar Country Rock/06 No Rompas Más Mi Pobre Corazón.mp3");
    let remove_result = remove_music(&path_to_remove);
    assert!(remove_result.is_ok());
    assert!(!path_to_remove.exists());
}

#[test]
fn test_update_status() {
    let temp_dir = tempdir().unwrap();
    let old_dir = env::current_dir().unwrap();
    make_tiny_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();

    // Copy new file to root and run update_status, assert status is updated
    let source_path = old_dir.join("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let new_path = temp_dir.path().join("01 Stereo.mp3");
    std::fs::copy(&source_path, &new_path).expect("Failed to copy file to root.");
    let update_result = update_status(&new_path, "add");
    assert!(update_result.is_ok());
    assert!(find_current_status(&get_isrc(&new_path).unwrap().unwrap()).unwrap().unwrap().action == "add");

    // Archive the file and assert new status action is archive
    let archive_result = update_status(&new_path, "archive");
    assert!(archive_result.is_ok());
    assert!(find_current_status(&get_isrc(&new_path).unwrap().unwrap()).unwrap().unwrap().action == "archive");

    // Unarchive the file and assert new status action is unarchive
    let unarchive_result = update_status(&new_path, "unarchive");
    assert!(unarchive_result.is_ok());
    assert!(find_current_status(&get_isrc(&new_path).unwrap().unwrap()).unwrap().unwrap().action == "unarchive");

    // Remove the file and assert new status action is remove
    let remove_result = update_status(&new_path, "remove");
    assert!(remove_result.is_ok());
    assert!(find_current_status(&get_isrc(&new_path).unwrap().unwrap()).unwrap().unwrap().action == "remove");
}

#[test]
fn test_get_status_all() {
    let temp_dir = tempdir().unwrap();
    make_medium_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();

    let status_all = get_status_all();
    assert!(status_all.is_ok());

    // Check that the status_all vector contains the expected number of entries
    let status_entries = status_all.unwrap();
    assert_eq!(status_entries.len(), 37); // Adjust this number based on the expected number of entries in your medium_library status file

    // Check  that the status_all has date, isrc, and action fields for each entry
    for entry in &status_entries {
        assert!(!entry.date.is_empty());
        assert!(!entry.isrc.is_empty());
        assert!(!entry.action.is_empty());
    }

    // Check that the first entry has the expected values (adjust these values based on your medium_library status file)
    let first_entry = &status_entries[0];
    assert_eq!(first_entry.date, "2026-06-06");
    assert_eq!(first_entry.isrc, "USEE10250542");
    assert_eq!(first_entry.action, "add");

    // Check that the second entry has the expected values (adjust these values based on your medium_library status file)
    let second_entry = &status_entries[1];
    assert_eq!(second_entry.date, "2026-06-06");
    assert_eq!(second_entry.isrc, "AULI00629010");
    assert_eq!(second_entry.action, "add");

    // Check that the last entry has the expected values (adjust these values based on your medium_library status file)
    let last_entry = &status_entries[status_entries.len() - 1];
    assert_eq!(last_entry.date, "2026-06-06");
    assert_eq!(last_entry.isrc, "USRC10700021");
    assert_eq!(last_entry.action, "add");
}

#[test]
fn test_write_status() {
    let temp_dir = tempdir().unwrap();
    make_tiny_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();

    // Write a new status entry and check that it is added to the status file
    let date = "2026-06-07".to_string();
    let isrc = "TEST12345678".to_string();
    let action = "add".to_string();
    let write_result = write_status(date.clone(), &isrc, &action);
    assert!(write_result.is_ok());

    // Check that the new entry is in the status file, without using the get_status_all() helper
    let status_file = temp_dir.path().join(".musicl/status");
    let status_content = std::fs::read_to_string(&status_file).unwrap();
    assert_eq!(status_content, "2026-06-06,MXF369733932,add\n2026-06-07,TEST12345678,add\n");
}

#[test]
fn test_get_music_files() {
    let temp_dir = tempdir().unwrap();
    make_small_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();

    // Expected number of files
    let music_files = get_music_files();
    assert!(music_files.is_ok());
    let music_files_vec = music_files.unwrap();
    assert_eq!(music_files_vec.len(), 36);

    // Still the same when I copy a txt file in to the library
    let txt_file_path = temp_dir.path().join("library/Los Felinos/A Bailar Country Rock/test.txt");
    std::fs::write(&txt_file_path, "This is a test text file.").expect("Failed to write test text file.");
    let music_files_after_txt = get_music_files();
    assert!(music_files_after_txt.is_ok());
    let music_files_after_txt_vec = music_files_after_txt.unwrap();
    assert_eq!(music_files_after_txt_vec.len(), 36);
}

#[test]
fn test_get_lrc_files() {
    let temp_dir = tempdir().unwrap();
    make_small_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();

    // Add in a lrc file to the library
    let lrc_file_path = temp_dir.path().join("library/Talking Heads/Naked/1-03 Totally Nude.lrc");
    std::fs::write(&lrc_file_path, "This is a test lrc file.").expect("Failed to write test lrc file.");

    // Expected number of files
    let music_files = get_music_files();
    assert!(music_files.is_ok());
    let music_files_vec = music_files.unwrap();
    assert_eq!(music_files_vec.len(), 1);

    // Still the same when I copy a txt file in to the library
    let txt_file_path = temp_dir.path().join("library/Los Felinos/A Bailar Country Rock/test.txt");
    std::fs::write(&txt_file_path, "This is a test text file.").expect("Failed to write test text file.");
    let music_files_after_txt = get_music_files();
    assert!(music_files_after_txt.is_ok());
    let music_files_after_txt_vec = music_files_after_txt.unwrap();
    assert_eq!(music_files_after_txt_vec.len(), 1);
}

#[test]
fn test_get_files_with_extension() {
    // Create library directory with 3 text files
    let temp_dir = tempdir().unwrap();
    let library_dir = temp_dir.path().join("library");
    std::fs::create_dir_all(&library_dir).expect("Failed to create library directory.");
    std::fs::write(library_dir.join("file1.txt"), "File 1").expect("Failed to write file1.txt.");
    std::fs::write(library_dir.join("file2.txt"), "File 2").expect("Failed to write file2.txt.");
    std::fs::write(library_dir.join("file3.txt"), "File 3").expect("Failed to write file3.txt.");

    // Get files with .txt extension
    let txt_files = get_files_with_extension("txt");
    assert!(txt_files.is_ok());
    let txt_files_vec = txt_files.unwrap();
    assert_eq!(txt_files_vec.len(), 3);

    // Get files with .mp3 extension (should be empty)
    let mp3_files = get_files_with_extension("mp3");
    assert!(mp3_files.is_ok());
    let mp3_files_vec = mp3_files.unwrap();
    assert_eq!(mp3_files_vec.len(), 0);

    // Create archive directory with 2 text files
    let archive_dir = temp_dir.path().join("archive");
    std::fs::create_dir_all(&archive_dir).expect("Failed to create archive directory.");
    std::fs::write(archive_dir.join("file4.txt"), "File 4 ").expect("Failed to write file4.txt.");
    std::fs::write(archive_dir.join("file5.txt"), "File 5").expect("Failed to write file5.txt.");

    // Get files with .txt extension again (should include archive files)
    let txt_files_after_archive = get_files_with_extension("txt");
    assert!(txt_files_after_archive.is_ok());
    let txt_files_after_archive_vec = txt_files_after_archive.unwrap();
    assert_eq!(txt_files_after_archive_vec.len(), 5);
}

#[test]
fn test_get_isrc() {
    // Succeeds on a file with an ISRC tag
    let path_with_isrc = PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let isrc_result = get_isrc(&path_with_isrc);
    assert!(isrc_result.is_ok());
    let isrc = isrc_result.unwrap();
    assert!(isrc.is_some());
    assert_eq!(isrc.unwrap(), "USMTD9719701");

    // Fails on a file without an ISRC tag
    let path_without_isrc = PathBuf::from("tests/data/bad_metadata_file1/Parliament/Funkentelechy Vs. The Placebo Syndrome/1-03 Wizard Of Finance.mp3");
    let isrc_result_no_tag = get_isrc(&path_without_isrc);
    assert!(isrc_result_no_tag.is_ok());
    let isrc_no_tag = isrc_result_no_tag.unwrap();
    assert!(isrc_no_tag.is_none());
}

#[test]
fn test_remove_empty_directories() {
    let temp_dir = tempdir().unwrap();
    make_tiny_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();

    // Create an empty directory in the library
    let empty_dir_path = PathBuf::from("library/Los Felinos/A Bailar Country Rock/Empty Directory");
    std::fs::create_dir_all(&empty_dir_path).expect("Failed to create empty directory.");

    // Create a multi-level empty directory in the library
    let multi_level_empty_dir_path = PathBuf::from("library/Empty/Empty/Empty Directory/Level 2/Level 3");
    std::fs::create_dir_all(&multi_level_empty_dir_path).expect("Failed to create multi-level empty directory.");

    // Check that both empty directories exist
    assert!(empty_dir_path.exists());
    assert!(multi_level_empty_dir_path.exists());

    // Remove empty directories
    let remove_result = remove_empty_directories(&temp_dir.path().join("library"));
    assert!(remove_result.is_ok());

    // Check that both empty directories have been removed
    assert!(!empty_dir_path.exists());
    assert!(!multi_level_empty_dir_path.exists());

    // Check that non-empty directories still exist
    let non_empty_dir_path = PathBuf::from("library/Los Felinos/A Bailar Country Rock");
    assert!(non_empty_dir_path.exists());
}

#[test]
fn test_get_sublibrary() {
    let temp_dir = tempdir().unwrap();
    make_small_library(&temp_dir);
    let _guard = CurrentDirGuard::new(temp_dir.path()).unwrap();

    // Get sublibrary for library item
    let library_item_path = std::env::current_dir().unwrap().join("library/Otomo Yoshihide/Cathode/01 Modulation #2.mp3");
    assert!(library_item_path.exists());
    let add_sublibrary = get_sublibrary(&library_item_path);
    assert!(add_sublibrary.is_ok());
    // print temp_dir
    // println!("Temp dir: {}", temp_dir.path().display());
    // print working directory
    // println!("Working directory: {}", std::env::current_dir().unwrap().display());
    // print library path
    // println!("Library path: {}", library_path().unwrap().display());
    assert_eq!(add_sublibrary.unwrap(), library_path().unwrap());

    // Get sublibrary for archive item
    let archive_item_path = std::env::current_dir().unwrap().join("archive/Jacques Dutronc/Best Of Jacques Dutronc/05 Les gens sont fous, les temps sont flous (Remastered).mp3");
    assert!(archive_item_path.exists());
    let archive_sublibrary = get_sublibrary(&archive_item_path);
    assert!(archive_sublibrary.is_ok());
    assert_eq!(archive_sublibrary.unwrap(), archive_path().unwrap());

    // Attempt to get sublibrary for item at root
    let root_file_path = std::env::current_dir().unwrap().join("root_file.mp3");
    std::fs::write(&root_file_path, "This is a test music file at the root.").expect("Failed to write test music file at root.");
    assert!(root_file_path.exists());
    let root_sublibrary = get_sublibrary(&root_file_path);
    assert!(!root_sublibrary.is_ok());
}

#[test]
fn test_missing_metadata() {
    // Run on file with no metadata
    let path_without_metadata = PathBuf::from("tests/data/bad_metadata_file1/Parliament/Funkentelechy Vs. The Placebo Syndrome/1-03 Wizard Of Finance.mp3");
    let result = missing_metadata(&path_without_metadata);
    assert_eq!(result.len(), 6);
    assert!(result.contains(&String::from("title")));
    assert!(result.contains(&String::from("artist")));
    assert!(result.contains(&String::from("album")));
    assert!(result.contains(&String::from("track")));
    assert!(result.contains(&String::from("isrc")));
    assert!(result.contains(&String::from("cover")));

    // Run on file with all metadata
    let path_with_metadata = PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let result_with_metadata = missing_metadata(&path_with_metadata);
    assert_eq!(result_with_metadata.len(), 0);
}

#[test]
fn test_get_metadata() {
    // Ok for mp3
    let mp3_path = PathBuf::from("tests/data/correctly_located_file1/Pavement/Brighten the Corners/01 Stereo.mp3");
    let mp3_metadata = get_metadata(&mp3_path);
    assert!(mp3_metadata.is_ok());

    // Fail for txt
    let txt_path = PathBuf::from("tests/data/non_music_file.txt");
    let txt_metadata = get_metadata(&txt_path);
    assert!(txt_metadata.is_err());
}

#[test]
fn test_get_metadata_id3() {
    // Test metadata is correct for a file with ID3 tags
    let path = PathBuf::from("tests/data/file_with_disc_count/Parliament/Funkentelechy Vs. The Placebo Syndrome/1-03 Wizard Of Finance.mp3");
    let metadata = get_metadata(&path).unwrap();
    assert_eq!(metadata.title.unwrap(), "Wizard Of Finance");
    assert_eq!(metadata.artist.unwrap(), "Parliament");
    assert_eq!(metadata.album.unwrap(), "Funkentelechy Vs. The Placebo Syndrome");
    assert_eq!(metadata.disc.unwrap(), 1);
    assert_eq!(metadata.total_discs.unwrap(), 2);
    assert_eq!(metadata.track.unwrap(), 3);
    assert_eq!(metadata.isrc.unwrap(), "USWWW0125792");
    assert_eq!(metadata.has_cover, true);
}