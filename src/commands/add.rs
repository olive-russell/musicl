use anyhow::Result;
use glob::glob;
use musicl::{in_archive, in_library, is_music_file, isrc_in_use, missing_metadata, move_music};
use std::path::PathBuf;

pub fn handle(paths_wildcard: PathBuf) -> Result<()> {
    // Iterate files
    let paths_str = paths_wildcard.to_str().unwrap();
    let paths = glob(paths_str).expect("Failed to glob path");
    for path in paths {
        match path {
            Ok(path) => add_to_library(&path)?,
            Err(error) => println!("{:?}", error),
        }
    }
    Ok(())
}

pub fn add_to_library(path: &PathBuf) -> Result<()> {
    print!("{}: ", path.to_str().unwrap());
    // Check file is not in library or archive
    if in_library(&path) || in_archive(&path) {
        eprintln!("Not added. Path is within library/archive.");
        return Ok(());
    }

    // Check that it is a music file
    if !is_music_file(&path) {
        eprintln!("Not added. Is not a music file.");
        return Ok(());
    }

    // Check all required metadata fields are there
    let missing_metadata = missing_metadata(&path);
    if !missing_metadata.is_empty() {
        eprintln!(
            "Not added. Missing metadata: {}",
            missing_metadata.join(", ")
        );
        return Ok(());
    }

    // Check ISRC available
    if isrc_in_use(&path)? {
        eprintln!("Not added. ISRC already in use.");
        return Ok(());
    }

    // Move file in and update status
    move_music(&path, "add")?;
    println!("Added.");
    Ok(())
}
