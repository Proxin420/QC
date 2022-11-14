use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Tokens {
    Pop,
    Add,
    Sub,
    Mul,
    Dup,
    Swap,
    Over,
    Equals,
    Bigger,
    Smaller,
    Print,
    Int(usize),
    String(String),
    Assembly(String),
}


pub fn lexer(program_path: String) -> (Vec<Tokens>, HashMap<usize, usize>, Vec<String>) {
    println!("[INFO]: Lexing program");
    let keywords = vec![
        "dup",
        "drop",
        "swap",
        "over",
        "if",
        "end",
        "print",
    ];
    let file = File::open(program_path.clone());
    if file.is_err() {
        println!("Q compiler: Invalid path: {}", program_path);
        std::process::exit(1);
    }

    let buffer = BufReader::new(file.unwrap());
    let mut program: Vec<Tokens> = Vec::new();
    let mut token = String::new();
    let mut flag = "";
    let mut scope: HashMap<usize, usize> = HashMap::new();
    let mut current_scope: Vec<usize> = Vec::new();
    let mut strings: Vec<String> = Vec::new();

    for line in buffer.lines() {
        if line.is_err() {
            println!("[ERR]: Failed to read line");
        }
        for character in line.unwrap().split("") {
            if flag == "String" {
                if character == "\"" {
                    program.push(Tokens::String(token.clone()));
                    strings.push(token.clone());
                    token = String::new();
                    flag = "";
                } else {
                    token = token + character;
                }
                continue;
            } else if flag == "asm" {
                if character == ")" {
                    program.push(Tokens::Assembly(token.clone()));
                    token = String::new();
                    flag = "";
                } else {
                    if character == "" {
                        token = token + "\n";
                    }
                    token = token + character;
                }
                continue;
            }
            match character {
                "#" => { break; },
                "\"" => { flag = "String"; },
                "(" => { flag = "asm"; },
                "-" => { program.push(Tokens::Sub); },
                "+" => { program.push(Tokens::Add); },
                "*" => { program.push(Tokens::Mul); },
                "=" => { program.push(Tokens::Equals); },
                ">" => { program.push(Tokens::Bigger); },
                "<" => { program.push(Tokens::Smaller); },
                " " | "" => {
                    if token != "" {
                        let value = token.parse::<usize>();
                        if value.is_err() {
                            if keywords.contains(&token.as_str()) {
                                match token.as_str() {
                                    "dup" => { program.push(Tokens::Dup); },
                                    "swap" => { program.push(Tokens::Swap); },
                                    "over" => { program.push(Tokens::Over); },
                                    "drop" => { program.push(Tokens::Pop); },
                                    "if" => { current_scope.push(program.len()); },
                                    "end" => { scope.insert(current_scope.pop().unwrap(), program.len()); },
                                    "print" => { program.push(Tokens::Print); },
                                    _ => { /* Should not occur */ }
                                }
                                token = String::new();
                            } else {
                                println!("[ERR]: Unknown keyword: {}", token);
                                std::process::exit(1);
                            }
                        } else {
                            program.push(Tokens::Int(value.unwrap()));
                            token = String::new();
                        }
                    }
                },
                _ => { token = token + character; },
            }
        }
    }
    return (program, scope, strings);
}


