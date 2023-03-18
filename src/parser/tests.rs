use std::collections::VecDeque;
use crate::parser::Parser;
use crate::lexer::{Token, JsonKind};

struct TestLexer {
    items: VecDeque<Token>
}

impl Iterator for TestLexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.items.pop_front()
    }
}

// START: ELEMENT TESTING
#[test]
fn test_empty_array_element() {
    let items = VecDeque::from([
       Token::new(JsonKind::Space, 1, " ".to_string()),
       Token::new(JsonKind::BeginArray, 1, " ".to_string()),
       Token::new(JsonKind::EndArray, 1, " ".to_string()),
       Token::new(JsonKind::Space, 1, " ".to_string())
    ]);

    let lexer = TestLexer {
        items
    };

    let mut parser = Parser::new(Box::new(lexer));

    parser.start();
}

#[test]
fn test_array_element() {
    let items = VecDeque::from([
       Token::new(JsonKind::BeginArray, 1, " ".to_string()),
       Token::new(JsonKind::BeginObject, 1, " ".to_string()),
       Token::new(JsonKind::EndObject, 1, " ".to_string()),
       Token::new(JsonKind::ValueSeperator, 1, " ".to_string()),
       Token::new(JsonKind::BeginObject, 1, " ".to_string()),
       Token::new(JsonKind::EndObject, 1, " ".to_string()),
       Token::new(JsonKind::EndArray, 1, " ".to_string()),
    ]);

    let lexer = TestLexer {
        items
    };

    let mut parser = Parser::new(Box::new(lexer));

    parser.start();
}

#[test]
fn test_empty_object_element() {
    let items = VecDeque::from([
       Token::new(JsonKind::Space, 1, " ".to_string()),
       Token::new(JsonKind::BeginObject, 1, " ".to_string()),
       Token::new(JsonKind::EndObject, 1, " ".to_string()),
       Token::new(JsonKind::Space, 1, " ".to_string())
    ]);

    let lexer = TestLexer {
        items
    };

    let mut parser = Parser::new(Box::new(lexer));

    parser.start();
}

#[test]
fn test_number_element() {
    let items = VecDeque::from([
       Token::new(JsonKind::Minus, 1, " ".to_string()),
       Token::new(JsonKind::Digit, 1, " ".to_string()),
       Token::new(JsonKind::Digit, 1, " ".to_string()),
       Token::new(JsonKind::Digit, 1, " ".to_string()),
       Token::new(JsonKind::DecimalPoint, 1, " ".to_string()),
       Token::new(JsonKind::Digit, 1, " ".to_string()),
       Token::new(JsonKind::Digit, 1, " ".to_string()),
       Token::new(JsonKind::E, 1, " ".to_string()),
       Token::new(JsonKind::Plus, 1, " ".to_string()),
       Token::new(JsonKind::Digit, 1, " ".to_string()),
       Token::new(JsonKind::Digit, 1, " ".to_string()),
    ]);

    let lexer = TestLexer {
        items
    };

    let mut parser = Parser::new(Box::new(lexer));

    parser.start();
}


#[test]
fn test_string_element() {
    let items = VecDeque::from([
       Token::new(JsonKind::StringVal, 1, " ".to_string()),
    ]);

    let lexer = TestLexer {
        items
    };

    let mut parser = Parser::new(Box::new(lexer));

    parser.start();
}

#[test]
fn test_true_element() {
    let items = VecDeque::from([
       Token::new(JsonKind::True, 1, " ".to_string()),
    ]);

    let lexer = TestLexer {
        items
    };

    let mut parser = Parser::new(Box::new(lexer));

    parser.start();
}

#[test]
fn test_false_element() {
    let items = VecDeque::from([
       Token::new(JsonKind::False, 1, " ".to_string()),
    ]);

    let lexer = TestLexer {
        items
    };

    let mut parser = Parser::new(Box::new(lexer));

    parser.start();
}

#[test]
fn test_null_element() {
    let items = VecDeque::from([
       Token::new(JsonKind::Null, 1, " ".to_string()),
    ]);

    let lexer = TestLexer {
        items
    };

    let mut parser = Parser::new(Box::new(lexer));

    parser.start();
}

// END: ELEMENT TESTING
