mod cli;
mod commands;
use anyhow::{Result};
use clap::Parser;
use cli::{Cli, Commands};

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

            // Dispatch subcommand
            match cli.command {
                Commands::Add{path} => commands::add::handle(path)?,
                Commands::Archive{path} => commands::archive::handle(path)?,
                Commands::Unarchive{path} => commands::unarchive::handle(path)?,
                Commands::Remove{path} => commands::remove::handle(path)?,
                Commands::Check{} => commands::check::handle()?,
                Commands::Clean{} => commands::clean::handle()?,
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}
