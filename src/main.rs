use std::env;
use std::fs::File;
use std::process;
use std::io::{BufRead, BufReader};
use itertools::Itertools;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let file = BufReader::new(File::open(config.file_path).expect("Unable to open file"));

    let mut collected = Vec::new();

    for line in file.lines() {
        collected.append(&mut lex_line(&line.expect("Unable to read line")));
    }

    let mut level = 0;
    let mut prev_key = false;

    for item in collected {
        match item.as_str() {
            "{" => {
                if !prev_key {
                    print_tab(level);
                } else {
                    prev_key = false;
                }
                println!("{}", item);
                level += 1;
            },
            "}" => {
                println!();
                level -= 1;
                print_tab(level);
                print!("{}", item);
            },
            "[" => {
                if !prev_key {
                    print_tab(level);
                } else {
                    prev_key = false;
                }
                println!("{}", item);
                level += 1;
            }, 
            "]" => {
                println!();
                level -= 1;
                print_tab(level);
                print!("{}", item);
            },
            ":" => {
                print!("{} ", item);
                prev_key = true
            },
            "," => {
                println!("{}", item);
            },
            _ => {
                if !prev_key {
                    print_tab(level);
                } else {
                    prev_key = false;
                }
                print!("{}", item);
            } 
        }
    }
}

fn print_tab(num: i32) {
    for _ in 0..num {
        print!("\t");
    }
}


fn lex_line(line: &str) -> Vec<String>{

    let mut iter = line.chars();
    let mut vals = Vec::new();
    let mut token;

    while let Some(ch) = iter.next() {

        if ch.is_whitespace() { continue };

        if ch.is_numeric() {
            token = lex_number(&ch, &mut iter)
        } else {
            token = match ch {
                '=' => String::from("="),
                '[' => String::from("["),
                ']' => String::from("]"),
                '{' => String::from("{"),
                '}' => String::from("}"),
                ':' => String::from(":"),
                ',' => String::from(","),
                '"' => lex_string(&mut iter),
                't' | 'f' | 'n' => lex_literal(&ch, &mut iter).expect("Unable to parse string literal"),
                _ => String::from(" ") 
            };
        }

        vals.push(token);
    }

    vals
}

struct Config {
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

fn lex_number(pre: &char, chars: &mut std::str::Chars) -> String {
    let mut string = pre.to_string();

    for ch in chars.peeking_take_while(|&c| c.is_numeric() 
                                       || c == '.' 
                                       || c == 'E' 
                                       || c == 'e' 
                                       || c == '-' 
                                       || c == '+') {
        string.push(ch)
    }

    string
}

fn lex_string(chars: &mut std::str::Chars) -> String {
    let mut string = "\"".to_string();

    for ch in chars.by_ref() {
        string.push(ch);
        if ch == '\"' {
            break;
        }
    }

    string
}

fn lex_literal(pre: &char, chars: &mut std::str::Chars) -> Result<String, &'static str> {
    let mut string = pre.to_string();

    for ch in chars.peeking_take_while(|&c| c.is_alphabetic()) {
        string.push(ch);
    }

    return match string.as_str() {
        "true" => Ok(String::from("true")),
        "false" => Ok(String::from("true")),
        "null" => Ok(String::from("null")),
        _ => Err("unexpected literal")
    };
}
