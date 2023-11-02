use rusqlite::{Connection, Result};

/// Creates a new table
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS content_table (
             id    INTEGER PRIMARY KEY,
             content  TEXT NOT NULL
         )",
        [],
    )?;
    Ok(())
}

/// Add
pub fn create_entry(conn: &Connection, data: &str) -> Result<()> {
    conn.execute("INSERT INTO content_table (content) VALUES (?1)", [data])?;
    Ok(())
}

/// Reads all
pub fn read_entries(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, content FROM content_table")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;

    for row in rows {
        let (id, data) = row?;
        println!("Found row: id = {}, content = {}", id, data);
    }
    Ok(())
}

/// Update
pub fn update_entry(conn: &Connection, id: i32, data: &str) -> Result<()> {
    conn.execute(
        "UPDATE content_table SET content = ?1 WHERE id = ?2",
        [data, &id.to_string()],
    )?;
    Ok(())
}

/// Delete
pub fn delete_entry(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM content_table WHERE id = ?1", [&id.to_string()])?;
    Ok(())
}
