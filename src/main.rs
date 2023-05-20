use std::io;
use std::file::File;

struct Defs{
    name: String
        definition: String
        industry: String
}

fn main() {
    println!("Thank You For Using Akronym!");

    let mut input = String::new();

    loop {
        //Display the main menu
        println!("1. Read a definition.");
        println!("2. Insert a defninition");
        println!("3. Quit Akronym");

        io::stdin()
            .read_line(&mut input)
            .expect("Please choose betweeen options 1, 2 or 3.");

        if input == "3" {
            break;
        } else if input == "1"{
            //TODO somefunction( queary the json file for the definiton.)
        } else if input == "2"{
            //TODO somefunction( take name and insert def into a json file)
        }

        break;
    }
}

fn insert_definition() => Ok(Result){
    //TODO
}
