use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Music library manager
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to the music library database
    #[arg(short, long, required=false)] //make this global=true idk?
    pub db: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialise new music library
    Setup(crate::commands::setup::SetupArgs),

    /// Play tracks from the library
    Play(crate::commands::play::PlayArgs),

    /// Add new music files to the library
    Add(crate::commands::add::AddArgs),

    /// Synchronise database with files
    Sync(crate::commands::sync::SyncArgs),

    /// Build a playlist
    Playlist(crate::commands::playlist::PlaylistArgs),

    /// Archive a track
    Archive(crate::commands::archive::ArchiveArgs),

    /// Unarchive a track
    Unarchive(crate::commands::unarchive::UnarchiveArgs),

    /// Remove a track
    Remove(crate::commands::remove::RemoveArgs),
}
