mod calculator;
use calculator::{calculate_operation, starter, format_output, read_config};

use crate::calculator::Config;
fn main() {
    starter();
    read_config();
    let config: Config = read_config();
    loop{
        match calculate_operation() {
            Ok(n) => println!("{}", format_output("Result", n, config.decimal_precision)),  
            Err(msg) => println!("{}",msg)
        }
    }
}