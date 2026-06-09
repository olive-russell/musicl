use anyhow::Result;
use musicl::is_music_file;
use std::{path::PathBuf, process::exit};

use crate::ctx::Ctx;

pub fn handle(ctx: &mut Ctx, path: std::path::PathBuf) -> Result<()> {
    print!("{}: ", path.to_str().unwrap());
    // Check file is in library or archive
    if !(path_in_library(file_path) || path_in_archive(file_path)) {
        println!("Not removed. Path is not within library/archive.");
        exit(1);
    }
    
    // Check that it is a music file
    if (!is_music_file(path)) {
        println!("Not removed. Is not a music file.");
        exit(1);
    }

    // Move file in and update status
    remove_music(path);
    println!("Added.");
    Ok(())
}
