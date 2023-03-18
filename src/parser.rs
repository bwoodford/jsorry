use crate::lexer::Token;
use crate::lexer::JsonKind;

pub struct Parser<'a> {
    lexer: Box<dyn Iterator<Item=Token> + 'a>,
    curr: Option<Token>,
    peek: Option<Token>
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Box<dyn Iterator<Item=Token> + 'a>) -> Parser {
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
            self.element();
        }
    }

    fn next_token(&mut self) {
        self.curr = self.peek.clone();
        self.peek = self.lexer.next();
    }


    fn is_kind(&self, kind: &JsonKind) -> bool {
        match &self.curr {
            None => false,
            Some(val) => val.kind == *kind
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
            Some(val) => val.kind == *kind
        }
    }

    fn must_match(&mut self, kind: &JsonKind) {
        if !self.is_kind(kind) {
            panic!("Expected token {:#?} did not match {:#?} | line: {}", 
                   kind, self.curr.as_ref().unwrap().kind, self.curr.as_ref().unwrap().line);
        }

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
        self.element();

        if self.is_kind(&JsonKind::ValueSeperator) {
            self.next_token();
            self.elements();
        }
    }

    fn element(&mut self) {

        self.whitespace();

        if self.is_kind(&JsonKind::BeginObject) {

            self.next_token();
            self.whitespace();

            if self.is_kind(&JsonKind::EndObject) {
                self.next_token();
            } else {
                self.members();
                self.must_match(&JsonKind::EndObject);
            }

        } else if self.is_kind(&JsonKind::BeginArray) {

            self.next_token();
            self.whitespace();

            if self.is_kind(&JsonKind::EndArray) {
                self.next_token();
            } else {
                self.elements();
                self.must_match(&JsonKind::EndArray);
            }

        } else if self.is_kind(&JsonKind::StringVal) ||  
            self.is_kind(&JsonKind::True) ||  
                self.is_kind(&JsonKind::False) ||
                self.is_kind(&JsonKind::Null) {

                    self.next_token();

                } else if self.is_kind(&JsonKind::Minus) || 
                    self.is_kind(&JsonKind::Digit) || 
                        self.is_kind(&JsonKind::Zero) {

                            self.number();

                        } else {
                            panic!("Unexpected token {:#?} on line {}", 
                                   self.curr.as_ref().unwrap().kind, self.curr.as_ref().unwrap().line);
                        }

                self.whitespace();
    }

    fn members(&mut self) {
        self.member();
        if self.is_kind(&JsonKind::ValueSeperator) {
            self.next_token();
            self.members();
        }
    }

    fn member(&mut self) {
        self.whitespace();
        self.must_match(&JsonKind::StringVal);
        self.whitespace();
        self.must_match(&JsonKind::NameSeperator);
        self.element();
    }

    fn number(&mut self) {
        self.integer();
        self.fraction();
        self.exponent();
    }

    fn integer(&mut self) {
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

#[cfg(test)]
mod tests;
