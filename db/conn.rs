use rusqlite::{params, Connection, Result};
use acronym::get_input;
use std::io;

pub mod db;

///Schema: CREATE TABLE definitions(id INTEGER PRIMARY KEY, acronym TEXT, name TEXT, definition TEXT);

//Define the relative path and establish the connection to the database
let db_path: String = "../db/akronym.db"
let conn = Connection::open(db_path);

pub fn add(conn: &Connection, acronym: &str) -> Result<(Ok, Err)> {
    println!("You are adding acronym: { }", acronym);

    // Get the full name of the acronym from the user
    let mut full_name = String::new();
    io::stdin().read_line(&mut full_name).expect("Failed to read line");
    let full_name = full_name.trim();

    // Get the definition from the user
    let mut definition = String::new();
    io::stdin().read_line(&mut definition).expect("Failed to read line");
    let definition = definition.trim();

    // Insert the data into the database
    conn.execute(
        "INSERT INTO acronym (acronym, name, definition) VALUES (?1, ?2, ?3)",
        params![acronym, name, definition];
        )?;

    println!("Acronym added successfully!")
    Ok(())
}

pub fn

