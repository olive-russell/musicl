use anyhow::Result;
use musicl::{find_current_status, get_isrc, get_lrc_files, get_music_files, get_status_all, get_sublibrary, has_correct_location, missing_metadata, sublibrary_from_action, valid_music_files};

pub fn handle() -> Result<()> {
    // Get current status of all items (ISRC and sublibrary)
    // let mut current_status_all = get_current_status_all()?;
    let mut status_all = get_status_all()?;

    // Get list of all files across archive and library
    for path in get_music_files()? {
        // Check for bad metadata
        let missing_metadata = missing_metadata(&path);
        if missing_metadata.len() != 0 {
            println!("{}: Missing metadata ({})", path.to_str().unwrap(), missing_metadata.join(", "));
            continue
        }

        // Get ISRC
        let isrc = get_isrc(&path)?.unwrap();
        
        // Remove ISRC from status array
        status_all.retain(|row| row.isrc != isrc);
        
        // If not found report that
        let status = find_current_status(&isrc).unwrap();
        if status.is_none() {
            println!("{}: Not found in status.", path.to_str().unwrap());
            continue;
        }

        // If status and folder mismatch
        if get_sublibrary(&path)? != sublibrary_from_action(status.unwrap().action.as_str())? {
            println!("{}: In wrong sub-library.", path.to_str().unwrap());
            continue;
        }

        // Check file is in correct place
        if !has_correct_location(&path)? {
            println!("{}: In wrong location.", path.to_str().unwrap());
            continue;
        }
    }

    // Look through lyric files
    for path in get_lrc_files()? {
        // If paired file doesn't exist, report
        for extension in valid_music_files() {
            let path_with_extension = path.with_extension(extension);
            if path_with_extension.exists() {
                continue;
            }
            println!("{}: Is orphaned lyric file.", path.to_str().unwrap());
        }
    }

    // Report remaining status items, filtering for unique ISRC (do not report removed items as missing)
    status_all.reverse();
    status_all.dedup_by(|a, b| a.isrc == b.isrc);
    status_all.retain(|row| row.action != "remove");
    if status_all.len() != 0 {
        for status in status_all {
            println!("{} is missing from {}", status.isrc, sublibrary_from_action(status.action.as_str())?.to_str().unwrap());
        }
    }

    Ok(())
}