use anyhow::Result;
use musicl::{archive_path, get_music_files, has_correct_location, library_path, move_to_correct_location, remove_empty_directories};

pub fn handle() -> Result<()> {
    // Remove empty folders in library
    if library_path()?.is_dir() {
        remove_empty_directories(&library_path()?)?;
    }

    // Remove empty folders in archive
    if archive_path()?.is_dir() {
        remove_empty_directories(&archive_path()?)?;
    }

    // Get list of all files across library and archive
    for path in get_music_files()? {
        // Check file is in correct place
        if !has_correct_location(&path)? {
            move_to_correct_location(&path)?;
            println!("{}: Moved.", path.to_str().unwrap());
        }
    }
    Ok(())
}