use rusqlite::{Connection, Result};

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        create table if not exists times (
            id integer primary key autoincrement,
            title text not null,
            description text,
            start_time text not null default current_timestamp,
            end_time text,
            created_at text not null default current_timestamp,
            updated_at text not null default current_timestamp,
            check (end_time is null or end_time >= start_time)
        );
    ",
        (),
    )?;

    conn.execute(
        "
        create table if not exists times_labels (
            time_id integer,
            label_id integer,
            primary key (time_id, label_id),
            foreign key (time_id) references times(id) on delete cascade,
            foreign key (label_id) references labels(id) on delete cascade
        );
    ",
        (),
    )?;

    Ok(())
}
