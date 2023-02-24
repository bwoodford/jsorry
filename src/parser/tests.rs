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

#[test]
fn test_element() {
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
