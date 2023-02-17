
mod tokenizer;
mod parsizer;

use std::env;
use std::process;
use crate::tokenizer::tokens::FileContents;
use crate::parsizer::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let contents = FileContents::new(config.file_path);
    let tokens = contents.token_iter();
    let mut parser = Parser::new(tokens);
    parser.start();
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
