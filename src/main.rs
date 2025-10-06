mod calculator;
use calculator::{calculate_operation, starter};

fn main() {
    starter();
    loop{
        match calculate_operation() {
            Ok(n) => println!("Result: {}", n),
            Err(msg) => println!("{}",msg)
        }
    }
}

