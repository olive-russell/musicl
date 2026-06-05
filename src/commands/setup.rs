use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct SetupArgs {
    // / Path to new music library database file to be created
    // new_db: PathBuf,
}

pub fn handle(db: PathBuf, args: SetupArgs) -> Result<()> {
    _ = args;
    println!("Setting things up: {:?}", db);
    Ok(())
}
