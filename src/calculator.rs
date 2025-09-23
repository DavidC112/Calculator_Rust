
use std::{io};
/*pub fn askNumber() -> f64{
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
*/
/*pub fn askOperator() -> String{
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
*/
/*pub fn calculate() -> f64{
    
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
}*/

pub fn ask_for_operation() -> String{
    loop {
        println!("Enter an operation: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Something went wrong!");
        let operation = input.trim();
        if operation.is_empty(){
            continue;
        }
        return operation.to_ascii_lowercase();   
    }
}


pub  fn calculate_operation() -> f64{
    let input = ask_for_operation();
    let mut result = 0.00;
    let mut number:Vec<f64> = vec![];
    let mut operators :Vec<String> = vec![];
    let operation: Vec<&str> = input.split_whitespace().collect();
    for (i, value) in operation.iter().enumerate() {
        if i % 2 == 0 {
            match value.parse::<f64>() {
                Ok(num) => number.push(num),
                Err(_) => println!("Something went wrong!")
            }
        } else {
            operators.push(value.to_string());
        }
    }
    result = number[0];
    for (i, value) in operators.iter().enumerate(){
        if value == "+" {
            result += number[i+1]
        }
        if value == "-"
        {
            result -= number[i+1]
        }
    }
    return result;

}