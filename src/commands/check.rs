use anyhow::Result;
use glob::glob;
use musicl::{find_current_status, get_correct_location, get_current_status_all, get_isrc, get_lrc_files, get_music_files};

pub fn handle() -> Result<()> {
    // Get current status of all items (ISRC and sublibrary)
    let mut current_status_all = get_current_status_all();

    // Get list of all files across archive and library
    for path in get_music_files() {
        // Get ISRC and check status
        let isrc = get_isrc(&path);
        let status = find_current_status(current_status_all, isrc);
        
        // If not found report that
        if (!status) {
            println!("{path}: Not found in status.");
            continue;
        }

        // If status and folder mismatch
        if (get_sublibrary(file) != status.tolower()) {
            println!("{path}: In wrong sub-library.");
            continue;
        }

        // Check file is in correct place
        if has_correct_location(&path) {
            println!("{path}: In wrong location.");
            continue;
        }
        
        // Remove item from status array
        current_status_all.retain(|&x| x != status);
    }

    // Look through lyric files
    for path in get_lrc_files() {
        // If paired file doesn't exist, report
        path_without_extension = path.with_extension("").join(".*");
        if glob(path_without_extension).expect("Failed to glob path").count() != 0 {
            println!("{path}: Is orphaned lyric file.");
        }
    }

    // Report remaining status items
    if status.count() != 0 {
        for item in status {
            println("{:} is missing from {:}", item.isrc, item.sublibrary);
        }
    }

    Ok(())
}