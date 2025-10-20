use core::f64;
use std::fs::{self};
use std::{char, io};
use serde::{Deserialize, Serialize};
use chrono::{ DateTime, Local};
use std::io::Write;

#[derive(Clone)]
enum Token  {
    Number(f64),
    Operator(char),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct History{
    expression: String,
    result: f64,
    time: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    history_length: usize,
    pub decimal_precision: usize
}


pub fn ask_for_operation() -> String{
    loop {

        print!("Enter an operation: ");
        io::stdout().flush().unwrap(); 
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error case 1");
        let operation = input.trim();
        if operation.to_ascii_lowercase().starts_with("edit"){
            edit_config(operation);
            continue;
        }
        if operation.to_ascii_lowercase() == "commands"{
            commands();
        }
        if operation.to_ascii_lowercase() == "exit"{
            exit();
        }
        if operation.to_ascii_lowercase() == "history"{
            read_history();
            continue;
        }
        if operation.to_ascii_lowercase() == "clear history"{
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
    let date = Local::now();
    let input = ask_for_operation();
    let mut tokens: Vec<Token> = Vec::new();
    let operation: Vec<&str> = input.split_whitespace().collect();

    for i in operation{
        if i == "sqrt" {
            tokens.push(Token::Operator('s'));
            continue;
        }
        match i.parse::<f64>() {
            Ok(num) => tokens.push(Token::Number(num)),
            Err(_) =>
            {
                let op = i.chars().next();
                match op {
                    Some(c)=> tokens.push(Token::Operator(c)),
                    _ => return Err("error case 2"),
                }
            }
        }
    }

    if tokens.is_empty(){
        return Err("error case 3");
    }
    brackets(&mut tokens)?;
    power_root(&mut tokens);

    multiplication_divison(&mut tokens);

    let result: f64;

    match addition_subtraction(&mut tokens){
        Ok(res) => result = res,
        Err(e) => return Err(e),
    }

    if result.is_infinite() | result.is_nan(){
        return Err("error case 4");
    }
    else{
        write_history(input, format_history(result), date);
        return Ok(result);
    }
}


pub fn starter(){
    println!("Welcome to the Rust Calculator!");
    println!("You can enter operations like '3 + 5 * 2 - 4 / 2'");
    println!("Type 'commands' to see the commands.");
}

fn exit(){
    std::process::exit(0)
}


fn clear_history(){
    fs::write("history.json", "").expect("error case 10");
}

fn multiplication_divison(tokens: &mut Vec<Token>){
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            Token::Operator('*') => {
                if let (Token::Number(left), Token::Number(right)) = (&tokens[i - 1], &tokens[i + 1]) {
                    let temp = left * right;
                    tokens.splice(i - 1..=i + 1, [Token::Number(temp)]);
                    i = i.saturating_sub(1)
                }
            }
            Token::Operator('/') => {
                if let (Token::Number(left), Token::Number(right)) = (&tokens[i - 1], &tokens[i + 1]) {
                    let temp = left / right;
                    tokens.splice(i - 1..=i + 1, [Token::Number(temp)]);
                    i = i.saturating_sub(1)
                }
            }
            _ => i += 1,
        }
    }
}

fn addition_subtraction(tokens: &mut Vec<Token>) -> Result<f64, &'static str>{
    let mut result = match tokens.get(0) {
        Some(Token::Number(n)) => *n,
        _ => return Err("error case 11"),
    };
    let mut i = 1;
    while i < tokens.len() {
            if let(Token::Operator(op), Some(Token::Number(n))) = (&tokens[i], tokens.get(i + 1)){
                match op {
                    '+' => result += *n,
                    '-' => result -= *n,
                    _ => return Err("error case 12"),
                }
            }
            else{
                return Err("error case 13");
            }
        i += 2;
    }
    return Ok(result)
}

fn power_root(tokens:&mut Vec<Token>){
    let mut i = 0;
    while i < tokens.len(){
        match tokens[i]{
            Token::Operator('s') =>{
                if i  == 0 {
                     if let Token::Number(right) = &tokens[i + 1] {
                        let temp = right.sqrt();
                        tokens.splice(i..=i + 1, [Token::Number(temp)]);
                        i = i.saturating_sub(1);
                    }
                }
                else if let (Token::Number(left), Token::Number(right)) = (&tokens[i - 1], &tokens[i + 1]) {
                    let temp = left.powf(1.00/ *right);
                    tokens.splice(i-1..=i + 1, [Token::Number(temp)]);
                    i = i.saturating_sub(1);
                }
                else if let(Token::Operator(_left ), Token::Number(right)) = (&tokens[i - 1], &tokens[i + 1]) {
                    let temp = right.sqrt();
                    tokens.splice(i..=i + 1, [Token::Number(temp)]);
                    i = i.saturating_sub(1);
                }
            }
            Token::Operator('^') =>{
                if let (Token::Number(left), Token::Number(right)) = (&tokens[i - 1], &tokens[i + 1]) {
                    let temp = left.powf(*right);
                    tokens.splice(i-1..=i + 1, [Token::Number(temp)]);
                    i = i.saturating_sub(1);
                }
            }
            _ => i += 1
        }
    }
}

pub fn read_config() -> Config {
    let json = fs::read_to_string("config.json").expect("error case 14");
    let config: Config = serde_json::from_str(&json).expect("error case 15"); 
    return config;
}

pub fn format_output(n: f64) {
    let config: Config = read_config();
    let precision = config.decimal_precision;
    let s = n.to_string();
    let digits = s.split(".").nth(1).map(|part|part.len()).unwrap_or(0);

    if digits > precision{
        println!("Result = {:.prec$}", n, prec = precision)
    }
    else{
        println!("Result = {}", n)
    }
}

fn format_history(n: f64 ) -> f64{
    let config: Config = read_config();
    let precision = config.decimal_precision;
    let s = n.to_string();
    let digits = s.split(".").nth(1).map(|part| part.len()).unwrap_or(0);
    if digits > precision{
        let pow = 10.0_f64.powf(precision as f64);
        let y = (n * pow).round() / pow;
        return y;
    }
    else {
        return n
    }
}

fn read_history() -> Vec<History>{
    let json = fs::read_to_string("history.json").expect("Error case 16");
    let list:Vec<History> = serde_json::from_str(&json).expect("Error code 17");

    for i in &list{
        println!("{} = {}",i.expression, i.result)
    }
    return list;
}


fn write_history(op: String, result:f64, date: DateTime<Local>){
    let config = read_config();
    let lenght = config.history_length;
    let json = fs::read_to_string("history.json").expect("Error case 16");
    let mut list: Vec<History> = serde_json::from_str(&json).unwrap_or_else(|_| Vec::new());
    let formatted_date = date.format("%Y-%M-%D %H:%M:%S");

    list.push(History { expression: (op), result: (result), time: (formatted_date.to_string()) });
    
    if list.len() > lenght{
        list = list[list.len()-lenght..].to_vec();
    }

    fs::write("history.json", serde_json::to_string_pretty(&list).unwrap()).unwrap();
}

fn edit_config(operation: &str){
    let json = fs::read_to_string("config.json").expect("error case 17");
    let mut config: Config = serde_json::from_str(&json).unwrap_or_else(|_| Config { history_length: (10), decimal_precision: (4) });
    let op: Vec<&str> = operation.split_whitespace().collect();

    if operation.to_ascii_lowercase().starts_with("edit decimal precision"){
            if let Some(value_str) = op.get(3) {
                if let Ok(value) = value_str.parse::<usize>(){
                    config.decimal_precision = value;
                }
                else{
                    println!("Invalid number!");
                }
            } 
            else{
                println!("Please provide a number after the command.");
            }
        }
    else if operation.to_ascii_lowercase().starts_with("edit history length") {
        if let Some(value_str) = op.get(3){
            if let Ok(value) = value_str.parse::<usize>(){
                config.history_length = value;
            }
            else{
                println!("Invalid number!")
            }
        }
        else{
            println!("Please provide a number after the command.")
        }
    }
    else{
        println!("Invalid edit command.")
    }
    
    fs::write("config.json", serde_json::to_string_pretty(&config).unwrap()).unwrap();
}

fn commands() {
    println!("Available commands:");
    println!("  edit history length <number>   - Set the history length (must provide a number)");
    println!("  edit decimal precision <number> - Set decimal precision (must provide a number)");
    println!("  history                        - Show history");
    println!("  clear history                  - Clear history");
    println!("  exit                           - Exit the program");
}


fn brackets(tokens: &mut Vec<Token>) -> Result<(), &'static str>{

        let mut i = 0;
        let mut modified = false;

        while i < tokens.len() {
            if let Token::Operator(')') = tokens[i]{
                let mut j = i;
                let mut found_open = false;

                while j > 0 {
                    j -= 1;
                    if let Token::Operator('(') = tokens[j]{
                        found_open = true;

                        let mut inner = tokens[j + 1..i].to_vec();

                        if inner.is_empty() {
                            return Err("Empty parentheses");
                        }

                        power_root(&mut inner);
                        multiplication_divison(&mut inner);
                        let result = addition_subtraction(&mut inner)?;
                        tokens.splice(j..=i, [Token::Number(result)]);
                        modified = true;

                        i = 0;
                        break;
                    }
                }

                if !found_open{
                    return Err("Unmatched closing parenthesis ')'");
                }
            } else{
                i += 1;
            }
        }

        if modified{
            Ok(())
        } else{
            Ok(())
        }
    }
