use rusqlite::{Connection, Result};

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        create table if not exists tasks (
            id integer primary key autoincrement,
            title text not null,
            description text,
            created_at text not null default current_timestamp
        );
    ",
        (),
    )?;

    conn.execute(
        "
        create table if not exists tasks_labels (
            task_id integer,
            label_id integer,
            primary key (task_id, label_id),
            foreign key (task_id) references tasks(id) on delete cascade,
            foreign key (label_id) references labels(id) on delete cascade
        );
    ",
        (),
    )?;

    Ok(())
}
