use anyhow::{Result, bail};
use musicl::{in_library, is_music_file, missing_metadata, move_music};

pub fn handle(path: std::path::PathBuf) -> Result<()> {
    print!("{}: ", path.to_str().unwrap());
    // Check file is in library
    if !in_library(&path) {
        bail!("Not unarchived. Path is not in archive.");
    }
    
    // Check that it is a music file
    if !is_music_file(&path) {
        bail!("Not unarchived. Is not a music file.");
    }

    // Check all required metadata fields are there
    let missing_metadata = missing_metadata(&path);
    if !missing_metadata.is_empty() {
        bail!("Not archived. Missing: {}.", missing_metadata.join(", "))
    }


    // Move file in and update status
    move_music(&path, "unarchive")?;
    println!("Added.");
    Ok(())
}
