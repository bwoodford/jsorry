pub mod tokens {
    
    use std::str::Chars;
    use std::iter::Peekable;
    use std::fs;

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

    pub struct FileContents {
        contents: String,
    }

    impl FileContents {
        pub fn new(filename: String) -> Self {
            let contents = fs::read_to_string(&filename).unwrap();
            FileContents { contents }
        }
        pub fn token_iter(&self) -> TokenIter<'_> {
            TokenIter {
                chars: self.contents.chars().peekable(),
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

    pub struct TokenIter<'a> {
        chars: Peekable<Chars<'a>>,
        curr_line: usize
    }

    impl<'a> Iterator for TokenIter<'a> {
        type Item = Token;
        fn next(&mut self) -> Option<Self::Item> {
            match self.chars.next() {
                Some(val) => {
                    match val {
                        ' ' => {
                            Some(Token {kind: JsonKind::Space, line: self.curr_line, text: val.to_string()})
                        },
                        '\n' => {
                            self.curr_line += 1;
                            Some(Token {kind: JsonKind::LineFeed, line: self.curr_line, text: val.to_string()})
                        },
                        '\r' => {
                            Some(Token {kind: JsonKind::CarriageReturn, line: self.curr_line,  text: val.to_string()})
                        },
                        '\t' => {
                            Some(Token {kind: JsonKind::HorizontalTab, line: self.curr_line, text: val.to_string()})
                        },
                        '{' => {
                            Some(Token {kind: JsonKind::BeginObject, line: self.curr_line, text: val.to_string()})
                        },
                        '}' => {
                            Some(Token {kind: JsonKind::EndObject, line: self.curr_line, text: val.to_string()})
                        },
                        // Begin Array
                        '[' => {
                            Some(Token {kind: JsonKind::BeginArray, line: self.curr_line, text: val.to_string()})
                        },
                        // End Array
                        ']' => {
                            Some(Token {kind: JsonKind::EndArray, line: self.curr_line, text: val.to_string()})
                        },
                        // Name Seperator
                        ':' => {
                            Some(Token {kind: JsonKind::NameSeperator, line: self.curr_line, text: val.to_string()})
                        },
                        // Value Seperator
                        ',' => {
                            Some(Token {kind: JsonKind::ValueSeperator, line: self.curr_line, text: val.to_string()})
                        },
                        // Number stuff
                        '-' => {
                            Some(Token {kind: JsonKind::Minus, line: self.curr_line, text: val.to_string()})
                        },
                        '+' => {
                            Some(Token {kind: JsonKind::Plus, line: self.curr_line, text: val.to_string()})
                        },
                        'E' | 'e' => {
                            Some(Token {kind: JsonKind::E, line: self.curr_line, text: val.to_string()})
                        },
                        '.' => {
                            Some(Token {kind: JsonKind::DecimalPoint, line: self.curr_line, text: val.to_string()})
                        },
                        '0' => {
                            Some(Token {kind: JsonKind::Zero, line: self.curr_line, text: val.to_string()})
                        },
                        // Strings
                        '"' => {

                            let mut tmp = val.to_string();

                            while let Some(next) = self.chars.next() {
                                tmp.push(next);
                                if next == '"' {
                                    return Some(Token {kind: JsonKind::StringVal, line: self.curr_line, text: tmp});
                                } else if next == '\n' {
                                    break;
                                }
                            }

                            panic!("Non terminating string detected");

                        },
                        // Digits and Keywords
                        _ => {
                            if val.is_numeric() {

                                return Some(Token {kind: JsonKind::Digit, line: self.curr_line, text: val.to_string()});

                            } else if val.is_alphabetic() {

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

                                return Some(Token {kind, line: self.curr_line, text: tmp});
                            } 

                            panic!("Unexpected token: {}", val);

                        }
                    }
                },
                None => None
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

    // TODO: Painstakingly make more test cases
    #[test]
    fn test_tokens() {
        let data = "\"testObj\":{\"inner\": 14289, \"huh\": false}";

        let test_data = vec![
            Token { kind: JsonKind::StringVal, line: 1, text: "\"testObj\"".to_string() },
            Token { kind: JsonKind::NameSeperator, line: 1, text: ":".to_string() },
            Token { kind: JsonKind::BeginObject, line: 1, text: "{".to_string() },

            Token { kind: JsonKind::StringVal, line: 1, text: "\"inner\"".to_string() },

            Token { kind: JsonKind::NameSeperator, line: 1, text: ":".to_string() },

            Token { kind: JsonKind::Space, line: 1, text: " ".to_string() },

            Token { kind: JsonKind::Digit, line: 1, text: "1".to_string() },
            Token { kind: JsonKind::Digit, line: 1, text: "4".to_string() },
            Token { kind: JsonKind::Digit, line: 1, text: "2".to_string() },
            Token { kind: JsonKind::Digit, line: 1, text: "8".to_string() },
            Token { kind: JsonKind::Digit, line: 1, text: "9".to_string() },

            Token { kind: JsonKind::ValueSeperator, line: 1, text: ",".to_string() },
            Token { kind: JsonKind::Space, line: 1, text: " ".to_string() },

            Token { kind: JsonKind::StringVal, line: 1, text: "\"huh\"".to_string() },
            Token { kind: JsonKind::NameSeperator, line: 1, text: ":".to_string() },
            Token { kind: JsonKind::Space, line: 1, text: " ".to_string() },
            Token { kind: JsonKind::False, line: 1, text: "false".to_string() },

            Token { kind: JsonKind::EndObject, line: 1, text: "}".to_string() },
        ];

        let mut iter = TokenIter { chars: data.chars().peekable(), curr_line: 1 };
        let mut collect: Vec<Token> = Vec::new();

        while let Some(val) = iter.next() {
            collect.push(val);
        }

        assert_eq!(collect.len(), test_data.len(), "generated token data length does not match static test token data length");

        for (d, td) in collect.iter().zip(test_data.iter()) {
            assert_eq!(d.kind, td.kind, "token kind is incorrect");
            assert_eq!(d.text, td.text, "token text is incorrect");
            assert_eq!(d.line, td.line, "token line number is incorrect");
        }
    }

    #[test]
    fn test_keywords() {
        let data = "false";

        let test_data = vec![
            Token { kind: JsonKind::False, line: 1, text: "false".to_string() },
        ];

        let mut iter = TokenIter { chars: data.chars().peekable(), curr_line: 1 };
        let mut collect: Vec<Token> = Vec::new();

        while let Some(val) = iter.next() {
            collect.push(val);
        }

        println!("{:#?}", collect.first());

        assert_eq!(collect.len(), test_data.len(), "generated token data length does not match static test token data length");

        for (d, td) in collect.iter().zip(test_data.iter()) {
            assert_eq!(d.kind, td.kind, "token kind is incorrect");
            assert_eq!(d.text, td.text, "");
        }
    }
}
