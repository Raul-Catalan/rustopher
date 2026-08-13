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

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Startup,
    Shutdown,
    ClockIn,
    ClockOut,
    Tasks {},
    Review,
}
