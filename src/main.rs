mod calculator;
use calculator::{calculate_operation, starter};

fn main() {
        starter();
        while true{
            let res = calculate_operation();
            println!("Result: {}", res); 
        }
}

