mod cli;
mod config;
mod commands;
mod ctx;

mod models;
mod schema; // required for diesel

use anyhow::{Result};
use clap::Parser;
use cli::{Cli, Commands};
use musicl::establish_connection;
use ctx::Ctx;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut db_path = std::env::current_dir()?;
    db_path.push(".musicl");
    db_path.push("musicl.db");

    match cli.command {
        Commands::Init{} => commands::init::handle()?,
        _ => {
            let mut ctx = Ctx {
                connection: &mut establish_connection(db_path),
            };
            match cli.command {
                Commands::Add{path} => commands::add::handle(&mut ctx, path)?,
                Commands::Archive{path} => commands::archive::handle(&mut ctx, path)?,
                Commands::Unarchive{path} => commands::unarchive::handle(&mut ctx, path)?,
                Commands::Remove{path} => commands::remove::handle(&mut ctx, path)?,
                Commands::Check{} => commands::check::handle(&mut ctx)?,
                Commands::Clean{} => commands::clean::handle(&mut ctx)?,
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}
