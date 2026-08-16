use rusqlite::{Connection, Result};

pub fn init_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute_batch("PRAGMA foreign_keys = ON")?;

    // Create tables if they don't exist
    // crate::task::create_table(&conn)?;
    // crate::time::create_table(&conn)?;

    Ok(conn)
}
