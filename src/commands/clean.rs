use anyhow::Result;

pub fn handle() -> Result<()> {
    // Remove empty folders in library
    remove_empty_directories("library");
    remove_empty_directories("archive");

    // Get list of all files across library and archive
    for file in get_music_files() {
        // Check file is in correct place
        if (file != get_correct_location(file)) {
            println!("{file}: Moved.");
            move_music(file, get_sublibrary(file));
        }
    }
    Ok(())
}