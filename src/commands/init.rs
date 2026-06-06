use anyhow::Result;
use clap::Args;
// use std::path::PathBuf;

#[derive(Args)]
pub struct InitArgs {
    // / Path to new music library database file to be created
    // new_db: PathBuf,
}

pub fn handle() -> Result<()> {
    println!("Initialising");
    Ok(())
}
