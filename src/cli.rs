// This file is defining the interface on what the user can type.
use clap::{Parser, Subcommand};

/// Rustopher - An opinionated CLI workflow
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Name of the person to greet
    #[command(subcommand)]
    pub command: Commands,
}

// Subcommands Test
#[derive(Subcommand)]
pub enum Commands {
    Init,
    ClockIn,
    ClockOut,
    Tasks {},
    Test,
    Review,
}
