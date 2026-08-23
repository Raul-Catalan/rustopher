mod cli;
mod commands;
pub mod storage;

use clap::Parser;
use cli::{Cli, Commands};
use rusqlite::Result;

fn main() -> Result<()> {
    // Parse the terminal input
    let cli = Cli::parse();

    // Database path
    let home = std::env::var("HOME").expect("HOME env var is not set");
    let db_path = format!("{}/app.db", home);

    // Create Connection and initialize database and tables
    let conn = crate::storage::db::init_db(&db_path)?;

    // Create some dummy data
    storage::db::init_test_data(&conn)?;

    match cli.command {
        Commands::ClockIn => {
            commands::clockin::execute();
            Ok(())
        }
        _ => {
            println!("Command not implemented");
            Ok(())
        }
    }
}
