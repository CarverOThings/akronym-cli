
pub mod akronym;

use db::conn::connect;


pub fn get_input( &acronym ) {
    println("Definition for { }: ", &acronym);
}

