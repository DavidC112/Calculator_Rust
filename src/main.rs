mod calculator;
use calculator::{calculate};



fn main() {

    let result = calculate();

    println!("The result is: {:.2}", result);

}

