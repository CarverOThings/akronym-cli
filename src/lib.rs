//Schema: CREATE TABLE definitions(id INTEGER PRIMARY KEY, acronym TEXT, name TEXT, definition TEXT);

use rusqlite::{Connection, Error, Result, Row};
use std::io;

pub fn open_db() -> Result<Connection, Error> {
    //Define the relative path and establish the connection to the database

    const DB_PATH: &str = "db/akronym.db";

    let conn = Connection::open(DB_PATH);
    return conn;
}

pub fn add(acronym: String) -> Result<()> {
    println!("You are adding an acronym");

    // Get the full name of the acronym from the user
    let prompt = "Full name of acronym: ";
    let full_name: String = read_input(prompt);

    // Get the definition from the user
    let prompt: &str = "Definition: ";
    let definition = read_input(prompt);

    // Insert the data into the database
    let conn = open_db();
    conn?.execute(
        "INSERT INTO definitions (acronym, name, definition) VALUES (?1, ?2, ?3)",
        rusqlite::params![acronym, full_name, definition],
    )?;

    println!("Acronym added successfully!");
    Ok(())
}

pub fn delete(acronym: String) -> Result<()> {
    println!("Deleting an acronym entry!");
    println!("Deleting acronym: {}", acronym);

    /// Confirmation to delete.
    // Connect to Sqlite Database and Execute the operation.
    let conn = open_db();
    conn?.execute(
        "DELETE FROM definitions WHERE acronym VALUES (?1)",
        rusqlite::params![acronym],
    )?;

    Ok(())
}

pub fn search(acronym: String) -> Result<()> {
    let conn = open_db();

    Ok(())
}

fn read_input(prompt: &str) -> String {
    println!("{ }", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}
