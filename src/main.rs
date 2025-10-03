mod calculator;
use calculator::{calculate_operation, starter};

fn main() {
    starter();
    loop{
        let res = calculate_operation();
        match res {
            Ok(n) => println!("Result: {}", n),
            Err(msg) => println!("{}",msg)
        }
    }
}

