mod calculator;
use calculator::{calculate_operation, starter, format_output};

use crate::calculator::Config;
fn main() {
    starter();

    loop{
        match calculate_operation() {
            Ok(n) => format_output(n),
            Err(msg) => println!("{}",msg)
        }
    }
}