use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JsonKind {
    StringVal,
    False,
    True,
    Null,
    // Structural
    BeginArray,
    BeginObject,
    EndArray,
    EndObject,
    NameSeperator,
    ValueSeperator,
    // Space
    Space,
    HorizontalTab,
    LineFeed,
    CarriageReturn,
    // Numbers
    Plus,
    Minus,
    Digit,
    Zero,
    DecimalPoint,
    E
}

impl<'a> TokenIter<'a> {
    pub fn new(contents: &'a str) -> Self {
        TokenIter {
            chars: contents.chars().peekable(),
            curr_line: 1
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: JsonKind,
    pub text: String,
    pub line: usize 
}

impl Token {
    pub fn new(kind: JsonKind, line: usize, text: String) -> Token {
        Token {
            kind,
            line,
            text
        }
    }
}

pub struct TokenIter<'a> {
    chars: Peekable<Chars<'a>>,
    curr_line: usize
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            None => None,
            Some(val) => {
                let token = match val {
                    ' ' => {
                        Token::new(JsonKind::Space, self.curr_line, val.to_string())
                    },
                    '\n' => {
                        self.curr_line += 1;
                        Token::new(JsonKind::LineFeed, self.curr_line-1, val.to_string())
                    },
                    '\r' => {
                        Token::new(JsonKind::CarriageReturn, self.curr_line, val.to_string())
                    },
                    '\t' => {
                        Token::new(JsonKind::HorizontalTab, self.curr_line, val.to_string())
                    },
                    '{' => {
                        Token::new(JsonKind::BeginObject, self.curr_line, val.to_string())
                    },
                    '}' => {
                        Token::new(JsonKind::EndObject, self.curr_line, val.to_string())
                    },
                    '[' => {
                        Token::new(JsonKind::BeginArray, self.curr_line, val.to_string())
                    },
                    ']' => {
                        Token::new(JsonKind::EndArray, self.curr_line, val.to_string())
                    },
                    ':' => {
                        Token::new(JsonKind::NameSeperator, self.curr_line, val.to_string())
                    },
                    ',' => {
                        Token::new(JsonKind::ValueSeperator, self.curr_line, val.to_string())
                    },
                    '-' => {
                        Token::new(JsonKind::Minus, self.curr_line, val.to_string())
                    },
                    '+' => {
                        Token::new(JsonKind::Plus, self.curr_line, val.to_string())
                    },
                    'E' | 'e' => {
                        Token::new(JsonKind::E, self.curr_line, val.to_string())
                    },
                    '.' => {
                        Token::new(JsonKind::DecimalPoint, self.curr_line, val.to_string())
                    },
                    '0' => {
                        Token::new(JsonKind::Zero, self.curr_line, val.to_string())
                    },
                    '"' => {
                        let mut tmp = val.to_string();
                        let mut next: Option<&char>;

                        while let Some(curr) = self.chars.next() {
                            tmp.push(curr);
                            next = self.chars.peek();

                            if curr == '\\' && next.is_some() && *next.unwrap() == '"' {
                                tmp.push(self.chars.next().unwrap());
                            } else if curr == '"' {
                                break;
                            } else if curr == '\n' {
                                panic!("Multiline string detected");
                            } else if next.is_none(){
                                panic!("Nonterminating string found");
                            }
                        }

                        Token::new(JsonKind::StringVal, self.curr_line, tmp)
                    },
                    val if val.is_numeric() => {
                        Token::new(JsonKind::Digit, self.curr_line, val.to_string())
                    },
                    val if val.is_alphabetic() => {
                        let mut tmp = val.to_string();

                        while let Some(next) = self.chars.peek() {
                            if next.is_alphabetic() {
                                tmp.push(self.chars.next().unwrap());
                            } else {
                                break;
                            }
                        }

                        let kind = match check_keyword(&tmp) {
                            Ok(v) => v,
                            Err(e) => panic!("{}", e)
                        };

                        Token::new(kind, self.curr_line, tmp)
                    },
                    _ => {
                        panic!("Unexpected token: {}", val);
                    }
                };

                Some(token)
            }
        }
    }
}

fn check_keyword(keyword: &String) -> Result<JsonKind, String> {
    match keyword.as_str() {
        "null" => Ok(JsonKind::Null),
        "true" => Ok(JsonKind::True),
        "false" => Ok(JsonKind::False),
        _ => Err(format!("Unknown keyword detected: {}", keyword))
    }
}

#[cfg(test)]
mod tests;
