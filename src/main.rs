/// Akronym Command Line Interface. A convenient way to store and query acronyms.
// December 2023, Ronnie Reel @ adahywoodcraft@gmail.com

mod lib;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "akrym")]
#[command(author, version, about, long_about = None)]
struct Cli {
    
    #[command(subcommand)]
    command: Commands,

}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help = true)]
enum Commands {
    // Adds a definition 
    Add  {
        acronym: String,
    },

    //Prints a given acronym
    // Set behavior to ask if user wants to add an acronym if not found
    Search {
        acronym: String,
    },

    Delete {
        acronym: String,
    },

    //Update command here!
}


fn main() {
   let args = Cli::parse();
   match args.command {
        
       //insert a definition into the database via sqlite
       Commands::Add { acronym } => {
            if let Err(e) = lib::add() {
            println!("Error adding acronym: {})", e);
           }
       }

       //query an acronym from the sqlite database
       Commands::Search { acronym } => {
           println!("Acronym: {:#?} , some definition", acronym );
       }

       // drop a definition in the sqlite db
       Commands::Delete { acronym } => {
           println!("Deleted {:#?}", acronym );
       }
   }   

}


