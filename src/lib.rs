
pub mod akronym;

use db::conn;

pub fn add(conn, &acronym, &def) -> Result<()> {
    let conn = conn(); 
    
}
