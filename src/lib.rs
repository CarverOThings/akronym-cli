//Schema: CREATE TABLE definitions(id INTEGER PRIMARY KEY, acronym TEXT, name TEXT, definition TEXT);

use rusqlite::{Connection, Result, Error};
use std::io;

pub fn open_db() -> Result<Connection, Error> {
//Define the relative path and establish the connection to the database

    const DB_PATH: &str = "../db/akronym.db";

    let conn = Connection::open(DB_PATH);
    return conn;
}

pub fn add(acronym: String, conn: rusqlite::Result<Connection>) -> rusqlite::Result<()> {
    println!("You are adding an acronym");

    // Get the full name of the acronym from the user
    let prompt = "Full name of acronym: ";
    let full_name: String = read_input(prompt);

    // Get the definition from the user
    let prompt: &str = "Definition: ";
    let definition = read_input(prompt);

    // Insert the data into the database
    conn?.execute(
        "INSERT INTO acronym (acronym, name, definition) VALUES (?1, ?2, ?3)",
        rusqlite::params![acronym, full_name, definition],
    )?;

    println!("Acronym added successfully!");
    Ok(())
}

pub fn delete<OK>() -> rusqlite::Result<()> {
    // TODO
    Ok(())
}

pub fn search<OK>() -> rusqlite::Result<()> {
    // TODO
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
