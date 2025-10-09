use core::f64;
use std::fs::{self};
use std::{char, io};
use serde::{Deserialize, Serialize};
use chrono::{ DateTime, Local};
use std::time::SystemTime;


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

#[derive(Serialize, Deserialize)]
pub struct Config {
    history_length: usize,
    pub decimal_precision: usize
}


pub fn ask_for_operation() -> String{
    loop {
        println!("Enter an operation: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error case 1");
        let operation = input.trim();
        if operation == "exit"{
            exit();
        }
        if operation == "history"{
            read_history();
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
    let date = Local::now();
    let config : Config = read_config();
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
        let data = format!("{} = {}\n", input, format_history(result, config.decimal_precision)).trim().to_string();
        write_history(input, result,  date,config.history_length);
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
    return config
}

pub fn format_output(m: &str, n: f64, precision: usize) -> String {
    let s = n.to_string();
    let digits = s.split(".").nth(1).map(|part|part.len()).unwrap_or(0);

    if digits > precision{
        format!("{} = {:.prec$}", m, n, prec = precision)
    }
    else{
        format!("{} = {}", m, n)
    }
}

fn format_history(n: f64 , precision: usize) -> f64{
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


fn write_history(op: String, result:f64, date: DateTime<Local>, lenght: usize){
    let json = fs::read_to_string("history.json").expect("Error case 16");
    let mut list: Vec<History> = serde_json::from_str(&json).unwrap_or_else(|_| Vec::new());
    let formatted_date = date.format("%Y-%M-%D %H:%M:%S");

    list.push(History { expression: (op), result: (result), time: (formatted_date.to_string()) });
    
    if list.len() > lenght{
        list = list[list.len()-lenght..].to_vec();
    }

    fs::write("history.json", serde_json::to_string_pretty(&list).unwrap()).unwrap();
}