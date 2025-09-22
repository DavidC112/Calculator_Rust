use std::io;
pub fn askNumber() -> f64{
    loop {
        println!("Enter a number: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input!");
        match input.trim().parse::<f64>(){
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a number!");
            }
        };
    }
}

pub fn askOperator() -> String{
    loop{
        println!("Enter an opertaion (+|-|*|/): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Please enter an operator!");
        match input.trim(){
            "+" => return "+".to_string(), 
            "-" => return "-".to_string(),
            "*" => return "*".to_string(),
            "/" => return "/".to_string(),
            _ =>{
                println!("Invalid operator!")
            }
        }
    }
}

pub fn calculate() -> f64{
    
    let number_a = askNumber();
    let number_b = askNumber();
    loop {
        let operation = askOperator();
            let result = match operation.as_str() {
            "+" => return number_a + number_b,
            "-" => return number_a - number_b,
            "*" => return number_a * number_b,
            "/" => {
                if number_b == 0.00 {
                    println!("You cannot divide by 0!");
                    continue;
                }
                return number_a / number_b
            }
            _ => {
                println!("Something went wrong!");
                continue;
            }
        };
    }
}