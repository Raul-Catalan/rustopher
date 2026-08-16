mod cli;
mod commands;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    // Parse the terminal input
    let cli = Cli::parse();

    match cli.command {
        Commands::ClockIn => {
            commands::clockin::execute();
        }
        _ => {
            println!("Command not implemented");
        }
    }
}
