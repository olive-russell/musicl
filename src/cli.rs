use clap::{Parser, Subcommand};
// use std::path::PathBuf;

/// Music library manager
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
    },
    Add {
        path: std::path::PathBuf,
    },
    Archive {
        path: std::path::PathBuf,
    },
    Unarchive {
        path: std::path::PathBuf,
    },
    Remove {
        path: std::path::PathBuf,
    },
    Check {
    },
    Clean {
    },
}
