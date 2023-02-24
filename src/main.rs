mod lexer;
mod parser;

use std::{env, fs, process};
use crate::lexer::TokenIter;
use crate::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Starting to process {} file", config.file_path);
    let contents = fs::read_to_string(&config.file_path).unwrap();
    let tokens = TokenIter::new(&contents);
    let mut parser = Parser::new(Box::new(tokens));
    parser.start();
    println!("File {} looks good 👍", config.file_path);
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
