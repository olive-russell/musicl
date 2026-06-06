use anyhow::Result;

pub fn handle() -> Result<()> {
    // If .musicl directory exists, report and exit
    if std::path::Path::new(".musicl").exists() {
        print!("Library already exists in working directory.");
        std::process::exit(1);
    }

    // Create .musicl directory
    std::fs::create_dir(".musicl")?;

    // Create status file
    std::fs::File::create(".musicl/status")?;

    // Report back
    print!("Library created in working directory.");
    Ok(())
}
