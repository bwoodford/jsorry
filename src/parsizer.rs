pub mod parser {
    
    use crate::tokenizer::tokens::Token;
    use crate::tokenizer::tokens::JsonKind;
    use crate::tokenizer::tokens::TokenIter;

    pub struct Parser<'a> {
        lexer: TokenIter<'a>,
        curr: Option<Token>,
        peek: Option<Token>
    }

    impl<'a> Parser<'a> {
        pub fn new(lexer: TokenIter) -> Parser {
            let mut tmp = Parser {
                lexer,
                curr: None,
                peek: None
            };

            // Init curr and peek tokens
            tmp.curr = tmp.lexer.next();
            tmp.peek = tmp.lexer.next();

            tmp
        }

        pub fn start(&mut self) {
            while self.curr.is_some() {
                self.elements();
            }
        }

        fn next_token(&mut self) {
          println!("Next");
            self.curr = self.peek.clone();
            self.peek = self.lexer.next();
        }


        fn is_kind(&self, kind: &JsonKind) -> bool {
            match &self.curr {
                None => false,
                Some(val) => (val.kind == *kind)
            }
        }

        fn illegal_leading_zero(&self) {
            if self.is_kind(&JsonKind::Zero) && 
              (self.is_next(&JsonKind::Digit) || self.is_next(&JsonKind::Zero)) {
                panic!("Illegal leading zero found for number | line {}", 
                       self.curr.as_ref().unwrap().line);
            }
        }

        fn is_next(&self, kind: &JsonKind) -> bool {
            match &self.peek {
                None => false,
                Some(val) => (val.kind == *kind)
            }
        }

        fn must_match(&mut self, kind: &JsonKind) {
            if !self.is_kind(kind) {
                panic!("Expected token {:#?} did not match {:#?} | line: {}", 
                       kind, self.curr.as_ref().unwrap().kind, self.curr.as_ref().unwrap().line);
            }
            println!("{:#?} | line: {} | next: {:#?}", kind, self.curr.as_ref().unwrap().line, self.peek.as_ref().unwrap().kind);
            self.next_token();
        }

        fn must_match_either(&mut self, kind1: &JsonKind, kind2: &JsonKind) {
            if !self.is_kind(kind1) && !self.is_kind(kind2) {
                panic!("Expected token {:#?} did not match {:#?}, or {:#?} | line: {}", 
                       kind1, kind2, self.curr.as_ref().unwrap().kind, self.curr.as_ref().unwrap().line);
            }
            self.next_token();
        }

        fn elements(&mut self) {
            println!("Elements");
            self.element();

            if self.is_kind(&JsonKind::ValueSeperator) {
              println!("Value Seperator ,");
              self.next_token();
              self.elements();
            }
        }

        fn element(&mut self) {
            println!("Element");
            self.whitespace();

            if self.is_kind(&JsonKind::BeginObject) {

                println!("Begin object {{");
                self.next_token();
                self.whitespace();

                self.members();

                self.must_match(&JsonKind::EndObject);

            } else if self.is_kind(&JsonKind::BeginArray) {
                
                println!("Begin Array [");

                self.next_token();
                self.elements();

                self.must_match(&JsonKind::EndArray);

            } else if self.is_kind(&JsonKind::StringVal) ||  
                      self.is_kind(&JsonKind::True) ||  
                      self.is_kind(&JsonKind::False) ||
                      self.is_kind(&JsonKind::Null) {

                println!("String | True | False | Null");
                self.next_token();

            } else if self.is_kind(&JsonKind::Minus) || 
                      self.is_kind(&JsonKind::Digit) || 
                      self.is_kind(&JsonKind::Zero) {

                println!("Number");
                self.number();

            } else {
                panic!("Unexpected token {:#?} on line {}", 
                       self.curr.as_ref().unwrap().kind, self.curr.as_ref().unwrap().line);
            }

            self.whitespace();
        }

        fn members(&mut self) {
            println!("Members");
            self.member();

            if self.is_kind(&JsonKind::ValueSeperator) {
              println!("Value Seperator ,");
              self.next_token();
              self.members();
            }
        }

        fn member(&mut self) {
            println!("Member");
            self.whitespace();
            self.must_match(&JsonKind::StringVal);
            self.whitespace();
            self.must_match(&JsonKind::NameSeperator);
            self.element();
        }

        fn number(&mut self) {
          println!("Number");
          self.integer();
          self.fraction();
          self.exponent();
        }

        fn integer(&mut self) {
            println!("Integer");
            self.illegal_leading_zero();

            if self.is_kind(&JsonKind::Minus) {
              self.next_token();
              self.illegal_leading_zero();
              self.must_match_either(&JsonKind::Digit, &JsonKind::Zero);
            } 
            self.digits();

        }
        
        fn fraction(&mut self) {
          if self.is_kind(&JsonKind::DecimalPoint) {
            self.next_token();
            self.digits();
          }
        }

        fn exponent(&mut self) {
          if self.is_kind(&JsonKind::E) {
            self.next_token();
            if self.is_kind(&JsonKind::Plus) || self.is_kind(&JsonKind::Minus) {
              self.next_token();
            }
          }
        }

        fn digits(&mut self) {
          while let Some(val) = &self.curr {
            match val.kind {
              JsonKind::Digit | JsonKind::Zero => self.next_token(),
              _ => break,
            }
          }
        }

        fn whitespace(&mut self) {
            while let Some(val) = &self.curr {
                match val.kind { 
                    JsonKind::Space | 
                      JsonKind::HorizontalTab | 
                      JsonKind::LineFeed | 
                      JsonKind::CarriageReturn => self.next_token(),
                    _ => break,
                }
            }
        }
    }
}
