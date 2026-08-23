use rusqlite::{Connection, Result};

pub fn init_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute_batch("PRAGMA foreign_keys = ON")?;

    // Create tables if they don't exist
    conn.execute(
        "
        CREATE TABLE IF NOT EXIST labels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL
        );
    ",
        (),
    )?;

    crate::storage::labels::create_table(&conn)?;
    crate::storage::tasks::create_table(&conn)?;
    crate::storage::time::create_table(&conn)?;

    Ok(conn)
}

pub fn init_test_data(conn: &Connection) -> Result<()> {
    conn.execute(
        "insert into tasks (title, description, created_at) values (?1, ?2, datetime('now'))",
        (
            "Set up database",
            "Initialize SQLite database and write schema migrations.",
        ),
    )?;

    conn.execute(
        "insert into tasks (title, description, created_at) values (?1, ?2, datetime('now'))",
        (
            "Write API endpoints",
            "Implement CRUD endpoints for managing tasks and labels.",
        ),
    )?;

    conn.execute(
        "INSERT INTO tasks (title, description, created_at) VALUES (?1, ?2, datetime('now'))",
        (
            "Write unit tests",
            "Cover all database operations and handlers with tests.",
        ),
    )?;

    // 2. Insert sample labels
    conn.execute(
        "INSERT OR IGNORE INTO labels (name) VALUES (?1)",
        ("Backend",),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO labels (name) VALUES (?1)",
        ("Urgent",),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO labels (name) VALUES (?1)",
        ("Testing",),
    )?;

    // 3. Link tasks and labels in the junction table
    // Task 1 (Set up database) -> Backend, Urgent
    conn.execute(
        "INSERT OR IGNORE INTO tasks_labels (task_id, label_id) VALUES (1, 1)",
        (),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO tasks_labels (task_id, label_id) VALUES (1, 2)",
        (),
    )?;

    // Task 2 (Write API endpoints) -> Backend
    conn.execute(
        "INSERT OR IGNORE INTO tasks_labels (task_id, label_id) VALUES (2, 1)",
        (),
    )?;

    // Task 3 (Write unit tests) -> Testing, Urgent
    conn.execute(
        "INSERT OR IGNORE INTO tasks_labels (task_id, label_id) VALUES (3, 3)",
        (),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO tasks_labels (task_id, label_id) VALUES (3, 2)",
        (),
    )?;

    Ok(())
}
