use rusqlite::{params, Connection, Result};
pub mod conn;

let db_path: String = "../db/akronym.db"

pub fn connect(db_rel_path, Connection) -> Result<()>, conn {
    let conn = Connection::open(db_rel_path)?
};
