use crate::configuration;
use std:: io ;
use std::io::Write;
use configuration::{edit_config, exit, read_history, clear_history};




pub fn commands() {
    println!("Available commands:");
    println!("  edit history length <number>   - Set the history length (must provide a number)");
    println!("  edit decimal precision <number> - Set decimal precision (must provide a number)");
    println!("  history                        - Show history");
    println!("  clear history                  - Clear history");
    println!("  exit                           - Exit the program");
}

pub fn starter(){
    println!("Welcome to the Rust Calculator!");
    println!("You can enter operations like '3 + 5 * 2 - 4 / 2'");
    println!("Type 'commands' to see the commands.");
}


pub fn ask_for_operation() -> String{
    loop {

        print!("Enter an operation: ");
        io::stdout().flush().unwrap(); 
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error case 1");
        let operation = input.trim();
        if operation.to_ascii_lowercase().starts_with("edit"){
            edit_config(operation);
            continue;
        }
        if operation.to_ascii_lowercase() == "commands"{
            commands();
            continue;
        }
        if operation.to_ascii_lowercase() == "exit"{
            exit();
        }
        if operation.to_ascii_lowercase() == "history"{
            read_history();
            continue;
        }
        if operation.to_ascii_lowercase() == "clear history"{
            clear_history();
            println!("History cleared.");
            continue;
        }
        if operation.is_empty(){
            continue;
        }
        return operation.to_ascii_lowercase();   
    }
}