mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    // Parse the terminal input
    let cli = Cli::parse();

    match cli.command {
        Commands::Startup => {
            commands::startup::execute();
        }
        _ => {
            println!("Command not implemented");
        }
    }
}
