mod cli;
mod commands;
mod ctx;

use anyhow::{Result};
use clap::Parser;
use cli::{Cli, Commands};
use ctx::Ctx;

fn main() -> Result<()> {
    // Parse CLI
    let cli = Cli::parse();

    // Calculate status path from working directory
    let mut status_path = std::env::current_dir()?;
    status_path.push(".musicl");
    status_path.push("status");

    // Dispatch subcommand (init or something else)
    match cli.command {
        Commands::Init{} => commands::init::handle()?,
        _ => {
            // Test status path
            if !status_path.exists() {
                println!("Status file not found at: {:?}. Please run 'musicl init' first.", status_path);
                std::process::exit(1);
            }

            // Establish database connection
            let mut ctx = Ctx {
                status_path: status_path,
            };

            // Dispatch subcommand
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
