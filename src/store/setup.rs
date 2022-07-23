use rusqlite::{Connection, Result};

pub fn open_db() -> Result<Connection> {
    let conn = Connection::open("./db/data.db")?;

    Ok(conn)
}
