use std::{fs, path};

use anyhow::{Result, bail};

pub fn handle() -> Result<()> {
    // If .musicl directory exists, report and exit
    if path::Path::new(".musicl").exists() {
        bail!("Library already exists in working directory.");
    }

    // Create .musicl directory
    fs::create_dir(".musicl")?;

    // Create status file
    fs::File::create(".musicl/status")?;

    // Report back
    println!("Library created in working directory.");
    Ok(())
}
