use core::f64;
use std:: char;
use chrono::{ Local};
use crate:: configuration;
use crate:: user_interaction;
use configuration::*;
use user_interaction::*;

#[derive(Clone)]
enum Token  {
    Number(f64),
    Operator(char),
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
                    _ => return Err("Error"),
                }
            }
        }
    }

    if tokens.is_empty(){
        return Err("Enter an operation");
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
        return Err("Error");
    }
    else{
        write_history(input, format_history(result), date);
        return Ok(result);
    }
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
        _ => return Err("The operation must start with a number!"),
    };
    let mut i = 1;
    while i < tokens.len() {
            if let(Token::Operator(op), Some(Token::Number(n))) = (&tokens[i], tokens.get(i + 1)){
                match op {
                    '+' => result += *n,
                    '-' => result -= *n,
                    _ => return Err("Idk Something went wrong!"),
                }
            }
            else{
                return Err("This shouldn't happen idk what u did wrong!");
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