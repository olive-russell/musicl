use anyhow::Result;
use glob::glob;
use musicl::is_music_file;
use std::path::PathBuf;

use crate::ctx::Ctx;

pub fn handle(ctx: &mut Ctx, path: PathBuf) -> Result<()> {
    // Iterate files
    for file_path in glob(path.to_str().unwrap()).expect("Failed to glob path") {
        match file_path {
            Ok(file_path) => {
                print!("{}: ", file_path.to_str().unwrap());
                // Check file is not in library or archive
                if (path_in_library(file_path) || path_in_archive(file_path)) {
                    println!("Not added. Path is within library/archive.");
                    continue;
                }
                
                // Check that it is a music file
                if (!is_music_file(file_path)) {
                    println!("Not added. Is not a music file.");
                    continue;
                }

                // Check all required metadata fields are there
                let missing_metadata = missing_metadata(file_path);
                if missing_metadata {
                    println!("Not added. Missing metadata: {missing_metadata}");
                    continue;
                }

                // Check ISRC available
                if isrc_in_library(file_path) {
                    println!("Not added. ISRC already in use.");
                    continue;
                }

                // Move file in and update status
                move_music(file_path, "library");
                println!("Added.");
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}
