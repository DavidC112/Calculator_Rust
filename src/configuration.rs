use core::f64;
use std::{ fs};
use serde::{Deserialize, Serialize};
use chrono::{ Local, DateTime};




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


pub fn write_history(op: String, result:f64, date: DateTime<Local>){
    let config = read_config();
    let lenght = config.history_length;
    let json = fs::read_to_string("history.json").expect("json error code 1");
    let mut list: Vec<History> = serde_json::from_str(&json).unwrap_or_else(|_| Vec::new());
    let formatted_date = date.format("%Y-%M-%D %H:%M:%S");

    list.push(History { expression: (op), result: (result), time: (formatted_date.to_string()) });
    
    if list.len() > lenght{
        list = list[list.len()-lenght..].to_vec();
    }

    fs::write("history.json", serde_json::to_string_pretty(&list).unwrap()).unwrap();
}


pub fn read_history() -> Vec<History>{
    let json = fs::read_to_string("history.json").expect("json error code 2");
    let list:Vec<History> = serde_json::from_str(&json).expect("json error code 3");

    for i in &list{
        println!("{} = {}",i.expression, i.result)
    }
    return list;
}


pub fn format_history(n: f64 ) -> f64{
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


pub fn clear_history(){
    fs::write("history.json", "").expect("Failed to clear history!");
}


pub fn read_config() -> Config {
    let json = fs::read_to_string("config.json").expect("json error code 4");
    let config: Config = serde_json::from_str(&json).expect("json error code 5"); 
    return config;
}


pub fn edit_config(operation: &str){
    let json = fs::read_to_string("config.json").expect("json error code 5");
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


pub fn exit(){
    std::process::exit(0)
}
