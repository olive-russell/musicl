use std::fs::canonicalize;

use anyhow::{Result, bail};
use musicl::{in_archive, in_library, is_music_file, remove_music};

pub fn handle(path: std::path::PathBuf) -> Result<()> {
    print!("{}: ", path.to_str().unwrap());
    let cpath = canonicalize(path)?;

    // Check file is in library or archive
    if !(in_library(&cpath)? || in_archive(&cpath)?) {
        bail!("Not removed. Path is not within library/archive.");
    }
    
    // Check that it is a music file
    if !is_music_file(&cpath) {
        bail!("Not removed. Is not a music file.");
    }

    // Move file in and update status
    remove_music(&cpath)?;
    println!("Removed.");
    Ok(())
}
