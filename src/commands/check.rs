use anyhow::Result;
use musicl::get_correct_location;

pub fn handle() -> Result<()> {
    // Get current status of all items (ISRC and sublibrary)
    let mut status = get_current_status();

    // Get list of all files across archive and library
    for file in get_music_files() {
        // Get ISRC and check status
        let isrc = get_isrc();
        let file_status = status.find(|&x| x.isrc == isrc);
        
        // If not found report that
        if (!file_status) {
            println!("{file}: Not found in status.");
            continue;
        }

        // If status and folder mismatch
        if (get_sublibrary(file) != file_status.action.tolower()) {
            println!("{file}: In wrong sub-library.");
            continue;
        }

        // Check file is in correct place
        if (file != get_correct_location(file)) {
            println!("{file}: In wrong location.");
            continue;
        }
        
        // Remove item from status array
        status.retain(|&x| x != file_status);
    }

    // Look through lyric files
    for file in get_lrc_files() {
        // If paired file doesn't exist
        file_without_extension = file.with_extension("").join(".*");
        if glob::glob(file_without_extension).expect("Failed to glob path").count() != 0 {
            println!("{file}: Is orphaned lyric file.");
        }
    }

    // Report remaining status items
    if status.count() != 0 {
        for item in status {
            println("{:} is missing from sub-library {:}", item.isrc, item.sublibrary);
        }
    }

    Ok(())
}