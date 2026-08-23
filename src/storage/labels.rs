use rusqlite::{Connection, Result};

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        create table if not exists labels (
            id integer primary key autoincrement,
            name text unique not null
        );
    ",
        (),
    )?;
    Ok(())
}
