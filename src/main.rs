mod calculator;
mod configuration;
mod user_interaction;
use calculator::{calculate_operation, format_output};
use user_interaction:: {starter};

fn main() {
    starter();

    loop{
        match calculate_operation() {
            Ok(n) => format_output(n),
            Err(msg) => println!("{}",msg)
        }
    }
}