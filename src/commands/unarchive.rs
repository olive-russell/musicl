use anyhow::Result;
use musicl::is_music_file;
use std::{path::PathBuf, process::exit};

pub fn handle(path: std::path::PathBuf) -> Result<()> {
    print!("{}: ", path.to_str().unwrap());
    // Check file is in library
    if !path_in_library(path) {
        println!("Not unarchived. Path is not in archive.");
        exit(1);
    }
    
    // Check that it is a music file
    if (!is_music_file(path)) {
        println!("Not unarchived. Is not a music file.");
        exit(1);
    }

    // Check all required metadata fields are there
    let missing_metadata = missing_metadata(path);
    if missing_metadata {
        println!("Not unarchived. Missing metadata: {missing_metadata}");
        exit(1);
    }

    // Move file in and update status
    move_music(path, "library");
    println!("Added.");
    Ok(())
}
