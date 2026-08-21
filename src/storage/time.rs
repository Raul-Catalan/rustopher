use rusqlite::{Connection, Result};

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        create table if not exist times (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
    ",
        (),
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXIST labels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL
        );
    ",
        (),
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXIST tasks_labels (
            task_id INTEGER,
            label_id INTEGER,
            PRIMARY KEY (task_id, label_id),
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
            FOREIGN KEY (label_id) REFERENCES labels(id) ON DELETE CASCADE
        );
    ",
        (),
    )?;

    Ok(())
}
