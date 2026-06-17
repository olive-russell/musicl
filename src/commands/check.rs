use anyhow::Result;
use glob::glob;
use musicl::{find_current_status, get_isrc, get_lrc_files, get_music_files, get_status_all, get_sublibrary, has_correct_location, sublibrary_from_action};

pub fn handle() -> Result<()> {
    // Get current status of all items (ISRC and sublibrary)
    // let mut current_status_all = get_current_status_all()?;
    let mut status_all = get_status_all()?;

    // Get list of all files across archive and library
    for path in get_music_files()? {
        // Get ISRC and check status
        let isrc = get_isrc(&path).unwrap();
        let status = find_current_status(&isrc);
        
        // If not found report that
        if status.is_none() {
            println!("{}: Not found in status.", path.to_str().unwrap());
            continue;
        }

        // If status and folder mismatch
        if get_sublibrary(&path)? != sublibrary_from_action(status.unwrap().action.as_str()) {
            println!("{}: In wrong sub-library.", path.to_str().unwrap());
            continue;
        }

        // Check file is in correct place
        if has_correct_location(&path)? {
            println!("{}: In wrong location.", path.to_str().unwrap());
            continue;
        }
        
        // Remove item from status array
        status_all.retain(|x| x.isrc != isrc);
    }

    // Look through lyric files
    for path in get_lrc_files()? {
        // If paired file doesn't exist, report
        let path_without_extension = path.with_extension("").join(".*");
        if glob(path_without_extension.to_str().unwrap()).expect("Failed to glob path").count() != 0 {
            println!("{}: Is orphaned lyric file.", path.to_str().unwrap());
        }
    }

    // Report remaining status items
    if status_all.len() != 0 {
        for status in status_all {
            println!("{} is missing from {}", status.isrc, sublibrary_from_action(status.action.as_str()).to_str().unwrap());
        }
    }

    Ok(())
}