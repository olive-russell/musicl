use anyhow::Result;
use musicl::{get_music_files, remove_empty_directories};

pub fn handle() -> Result<()> {
    // Remove empty folders in library
    remove_empty_directories("library");
    remove_empty_directories("archive");

    // Get list of all files across library and archive
    for path in get_music_files() {
        // Check file is in correct place
        if has_correct_location(path) {
            println!("{path}: Moved.");
            move_music(path, get_sublibrary(path));
        }
    }
    Ok(())
}