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

    for (i, line) in file.lines().enumerate() {
        collected.append(&mut lex_line(&line.expect("Unable to read line"), i));
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

fn lex_line(line: &str, line_num: usize) -> Vec<String>{

    let mut iter = line.chars();
    let mut vals = Vec::new();
    let mut token;

    while let Some(ch) = iter.next() {

        if ch.is_whitespace() { continue };

        if ch.is_numeric() {
            token = lex_number(&ch, &mut iter)
        } else {
            token = match ch {
                '=' => Ok(String::from("=")),
                '[' => Ok(String::from("[")),
                ']' => Ok(String::from("]")),
                '{' => Ok(String::from("{")),
                '}' => Ok(String::from("}")),
                ':' => Ok(String::from(":")),
                ',' => Ok(String::from(",")),
                '"' => match lex_string(&mut iter) {
                    Ok(s) => Ok(s),
                    Err(e) => panic!("error: {} on line {}", e, line_num)
                },
                't' | 'f' | 'n' => lex_literal(&ch, &mut iter),
                _ => panic!("unknown character encountered on line {}", line_num)
            };
        }

        vals.push(token.unwrap());
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

fn lex_number(pre: &char, chars: &mut std::str::Chars) -> Result<String, &'static str> {
    let mut string = pre.to_string();

    for ch in chars.peeking_take_while(|&c| c.is_numeric() 
                                       || c == '.' 
                                       || c == 'E' 
                                       || c == 'e' 
                                       || c == '-' 
                                       || c == '+') {
        string.push(ch)
    }

    Ok(string)
}

fn lex_string(chars: &mut std::str::Chars) -> Result<String, &'static str> {
    let mut string = "\"".to_string();

    for ch in chars.by_ref() {
        string.push(ch);
        if ch == '\"' {
            break;
        }
    }

    if !string.ends_with('\"') || string.len() <= 1 {
        return Err("missing closing \" for string");
    }

    Ok(string)
}

fn lex_literal(pre: &char, chars: &mut std::str::Chars) -> Result<String, &'static str> {
    let mut string = pre.to_string();

    for ch in chars.peeking_take_while(|&c| c.is_alphabetic()) {
        string.push(ch);
    }

    return match string.as_str() {
        "true" => Ok(String::from("true")),
        "false" => Ok(String::from("false")),
        "null" => Ok(String::from("null")),
        _ => Err("unexpected literal")
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok_lex_literals() {
        let array = ["true", "false", "null"];
        let mut chars;
        let mut pre;

        for item in array.iter() {
            chars = item.chars();
            pre = chars.next().unwrap();

            assert_eq!(lex_literal(&pre, &mut chars), Ok(String::from(*item)));
        }
    }

    #[test]
    fn err_lex_literals() {
        let array = ["talse", "frue", "none"];
        let mut chars;
        let mut pre;

        for item in array.iter() {
            chars = item.chars();
            pre = chars.next().unwrap();

            assert_eq!(lex_literal(&pre, &mut chars), Err("unexpected literal"));
        }
    }

    #[test]
    fn ok_lex_strings() {
        let array = [
            "Woah this is a really good string\"", 
            "And here's another one\"", 
            "And the last string\""
        ];
        let mut chars;

        for item in array.iter() {
            chars = item.chars();

            assert_eq!(lex_string(&mut chars), Ok(format!("{}{}", "\"", *item)));
        }
    }

    #[test]
    fn err_lex_strings() {
        let array = [
            "Woah this is a really good string", 
            "And here's another one", 
            "And the last string"
        ];
        let mut chars;

        for item in array.iter() {
            chars = item.chars();

            assert_eq!(lex_string(&mut chars), Err("missing closing \" for string"));
        }
    }

    #[test]
    // reference: https://www.rfc-editor.org/rfc/rfc8259#section-6
    fn ok_lex_number() {
        let array = [
            "2.023123987491287389471", 
            "-19729.1010E+10",
            "2e234", 
            "2E234", 
            "+1000",
            "+1000E-10",
            "+1000E+10",
            "-1000E-10",
        ];
        let mut chars;
        let mut pre;

        for item in array.iter() {
            chars = item.chars();
            pre = chars.next().unwrap();

            assert_eq!(lex_number(&pre, &mut chars), Ok(String::from(*item)));
        }
    }

    #[test]
    fn err_lex_number() {
        let array = [
            "2.0231239 87491287389471", 
        ];
        let mut chars;
        let mut pre;

        for item in array.iter() {
            chars = item.chars();
            pre = chars.next().unwrap();

            assert_eq!(lex_number(&pre, &mut chars), Ok(String::from(*item)));

        }
    }
     

}
