mod cli;
mod commands;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    // Parse the terminal input
    let cli = Cli::parse();

    // Create database connection
    let db_path = "~/app.db";
    let conn = crate::storage::db::init_db(&db_path);

    match cli.command {
        Commands::ClockIn => {
            commands::clockin::execute();
        }
        _ => {
            println!("Command not implemented");
        }
    }
}
