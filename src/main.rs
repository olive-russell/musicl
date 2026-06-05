mod cli;
mod config;
mod commands;
mod ctx;

// use diesel::prelude::*;
// use musicl::models::*;
// use diesel_demo::*;

mod models;
mod schema; // required for diesel
// use crate::models::Song;
// use diesel::prelude::*;

use anyhow::{Result};
use clap::Parser;
use cli::{Cli, Commands};
use musicl::establish_connection;
use ctx::Ctx;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup(args) => commands::setup::handle(cli.db, args)?,
        _ => {
            let mut ctx = Ctx {
                connection: &mut establish_connection(cli.db),
            };
            match cli.command {
                
                Commands::Play(args) => commands::play::handle(&mut ctx, args)?,
                Commands::Add(args) => commands::add::handle(&mut ctx, args)?,
                Commands::Sync(args) => commands::sync::handle(&mut ctx, args)?,
                Commands::Playlist(args) => commands::playlist::handle(&mut ctx, args)?,
                Commands::Archive(args) => commands::archive::handle(&mut ctx, args)?,
                Commands::Unarchive(args) => commands::unarchive::handle(&mut ctx, args)?,
                Commands::Remove(args) => commands::remove::handle(&mut ctx, args)?,
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}
