
use core::f64;
use std::{char, io, ops::RemAssign};
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
    enum Token  
    {
        Number(f64),
        Operator(char),
    }
    let mut tokens: Vec<Token> = Vec::new();
    let operation: Vec<&str> = input.split_whitespace().collect();

    for i in operation{
        match i.parse::<f64>() {
            Ok(num) => tokens.push(Token::Number((num))),
            Err(_) =>
            {
                let op = i.chars().next().unwrap();
                tokens.push(Token::Operator((op)));
            }
        }
    }


    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            Token::Operator('*') => {
            if let (Token::Number(left), Token::Number(right)) = (&tokens[i-1], &tokens[i+1]) {
                let temp = left * right;
                tokens.splice(i-1..=i+1, [Token::Number(temp)]);
                i += 1;
            }
        },
        Token::Operator('/') => {
            if let (Token::Number(left), Token::Number(right)) = (&tokens[i-1], &tokens[i+1]){
                let temp = left / right;
                tokens.splice(i-1..=i+1,[Token::Number(temp)]);
                i += 1;
            }
        },
        _ => i += 1,
    }
}

    let mut result = if let Token::Number(n) = tokens[0] {n} else{0.00};
    let mut x = 1;
    while x < tokens.len(){
        if let Token::Operator(op) = tokens[x]{
            if let Token::Number(n) = tokens[x+1]{
                match op {
                    '+' => result += n,
                    '-' => result -= n,
                    _ =>unreachable!()
                }
            }
        }
        x += 2;
    }
    return  result;
}
