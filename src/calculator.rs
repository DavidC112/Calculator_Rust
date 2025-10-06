
use core::f64;
use std::fs;
use std::{char, io};
use std::fs::OpenOptions;
use std::io::Write;


enum Token  {
    Number(f64),
    Operator(char),
}

pub fn ask_for_operation() -> String{
    loop {
        println!("Enter an operation: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Something went wrong!");
        let operation = input.trim();
        if operation == "exit"{
            exit();
        }
        if operation == "history"{
            open_history();
            continue;
        }
        if operation == "clear history"{
            clear_history();
            println!("History cleared.");
            continue;
        }
        if operation.is_empty(){
            continue;
        }
        return operation.to_ascii_lowercase();   
    }
}



pub fn calculate_operation() -> Result<f64, &'static str>{

    

    let input = ask_for_operation();
    let mut tokens: Vec<Token> = Vec::new();
    let operation: Vec<&str> = input.split_whitespace().collect();

    for i in operation{
        match i.parse::<f64>() {
            Ok(num) => tokens.push(Token::Number(num)),
            Err(_) =>
            {
                let op = i.chars().next();
                match op {
                    Some(c)=> tokens.push(Token::Operator(c)),
                    _ => return Err("Error"),
                }
            }
        }
    }
    if tokens.is_empty(){
        return Err("Error");
    }

    multiplication_divison(&mut tokens);
    let result: f64;
    match addition_subtraction(&mut tokens){
        Ok(res) => result = res,
        Err(e) => return Err(e),
    }

    if result.is_infinite() | result.is_nan(){
        return Err("Error");
    }
    else{
        let data = format!("{} = {}\n", input, result).trim().to_string();
        write_history(data);
        return Ok(result);
    }
}


pub fn starter(){
    println!("Welcome to the Rust Calculator!");
    println!("You can enter operations like '3 + 5 * 2 - 4 / 2'");
    println!("Type 'history' to see previous calculations.");
    println!("Type 'exit' to quit the calculator.");
}

fn exit(){
    std::process::exit(0)
}

fn write_history(data: String) {
    {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("history.txt")
            .expect("Something went wrong!");

        writeln!(file, "{}", data).expect("Something went wrong!");
    }
    let content = fs::read_to_string("history.txt").expect("Something went wrong!");
    let mut lines: Vec<&str> = content.lines().collect();

    if lines.len() > 10 {
        lines = lines[lines.len()-10..].to_vec();
        fs::write("history.txt", lines.join("\n") + "\n").expect("Something went wrong!");
    }
}

fn open_history(){
    let history = fs::read_to_string("history.txt").expect("Something went wrong!");
    println!("History:\n{}", history.trim_end());
}

fn clear_history(){
    fs::write("history.txt", "").expect("Something went wrong!");
}

fn multiplication_divison(tokens: &mut Vec<Token>){
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            Token::Operator('*') => {
                if let (Token::Number(left), Token::Number(right)) = (&tokens[i - 1], &tokens[i + 1]) {
                    let temp = left * right;
                    tokens.splice(i - 1..=i + 1, [Token::Number(temp)]);
                    i += 1;
                }
            }
            Token::Operator('/') => {
                if let (Token::Number(left), Token::Number(right)) = (&tokens[i - 1], &tokens[i + 1]) {
                    let temp = left / right;
                    tokens.splice(i - 1..=i + 1, [Token::Number(temp)]);
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }
}

fn addition_subtraction(tokens: &mut Vec<Token>) -> Result<f64, &'static str>{
    let mut result = match tokens.get(0) {
        Some(Token::Number(n)) => *n,
        _ => return Err("Error"),
    };
    let mut x = 1;
    while x < tokens.len() {
            if let (Token::Operator(op), Some(Token::Number(n))) = (&tokens[x], tokens.get(x + 1)) {
                match op {
                    '+' => result += *n,
                    '-' => result -= *n,
                    _ => return Err("Error"),
                }
            }
            else{
                return Err("Error");
            }
        x += 2;
    }
    return Ok(result)
}