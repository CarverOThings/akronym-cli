use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    
    let conn = Connection::open("akronym.db")?;

    println!("Thank You For Using Akronym!");

    loop {
        //Display the main menu
    println!("1. Read a definition.");      
    println!("2. Insert a defninition");  
    println!("3. Quit Akronym");    
    }   
    
}
