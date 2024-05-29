use crate::common::{ast::*, token::*};

// TODO: only returns one expression for now
pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut p = Parser {
        tokens,
        current_idx: 0,
    };

    p.expression()
}

struct Parser {
    tokens: Vec<Token>,
    current_idx: usize,
}

impl Parser {
    //////////////////////
    // helper functions //
    //////////////////////

    // checks if current token has one of the types
    fn check_type(&self, types: Vec<TokenType>) -> bool {
        // not very pretty, but needed some way to eliminate value inside literal types
        match &self.tokens[self.current_idx].token_type {
            TokenType::Identifier(_) => types.iter().any(|t| matches!(t, TokenType::Identifier(_))),
            TokenType::String(_) => types.iter().any(|t| matches!(t, TokenType::String(_))),
            TokenType::Int(_) => types.iter().any(|t| matches!(t, TokenType::Int(_))),
            TokenType::Float(_) => types.iter().any(|t| matches!(t, TokenType::Float(_))),
            other => types.contains(other),
        }
    }

    // check_type but it moves the idx
    fn check_advance(&mut self, types: Vec<TokenType>) -> bool {
        if self.check_type(types) {
            self.current_idx += 1;
            return true;
        }
        false
    }

    /////////////
    // grammar //
    /////////////
    // see grammar.txt

    // expr -> equality
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    // equality -> comparison (("=="|"!=") comparison)*
    fn equality(&mut self) -> Expr {
        let mut ex = self.comparison();
        while self.check_advance(vec![TokenType::DoubleEqual, TokenType::NotEqual]) {
            // turn the token into a BiOp
            let op = match self.tokens[self.current_idx - 1].token_type {
                TokenType::DoubleEqual => BiOp::DoubleEqual,
                TokenType::NotEqual => BiOp::NotEqual,
                _ => panic!("In equality(): op token_type was not == or !=, error probably in check_advance() or equality()"),
            };
            let right = self.comparison();
            ex = Expr::Binary(Box::new(ex), op, Box::new(right));
        }
        ex
    }

    // comparison -> term ((">"|">="|"<"|">=") term)*
    fn comparison(&mut self) -> Expr {
        let mut ex = self.term();
        while self.check_advance(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            // turn the token into a BiOp
            let op = match self.tokens[self.current_idx - 1].token_type {
                TokenType::Greater => BiOp::Greater,
                TokenType::GreaterEqual => BiOp::GreaterEqual,
                TokenType::Less => BiOp::Less,
                TokenType::LessEqual => BiOp::LessEqual,
                _ => panic!("In comparison(): op token_type was not <,<=,> or >=, error probably in check_advance() or comparison()"),
            };
            let right = self.term();
            ex = Expr::Binary(Box::new(ex), op, Box::new(right));
        }
        ex
    }

    // term -> factor (("+"|"-") factor)*
    fn term(&mut self) -> Expr {
        let mut ex = self.factor();
        while self.check_advance(vec![TokenType::Plus, TokenType::Minus]) {
            // turn the token into a BiOp
            let op = match self.tokens[self.current_idx - 1].token_type {
                TokenType::Plus => BiOp::Plus,
                TokenType::Minus => BiOp::Minus,
                _ => panic!("In term(): op token_type was not + or -, error probably in check_advance() or term()"),
            };
            let right = self.factor();
            ex = Expr::Binary(Box::new(ex), op, Box::new(right));
        }
        ex
    }

    // factor -> unary (("*"|"/") unary)*
    fn factor(&mut self) -> Expr {
        let mut ex = self.unary();
        while self.check_advance(vec![TokenType::Asterisk, TokenType::Slash]) {
            // turn the token into a BiOp
            let op = match self.tokens[self.current_idx - 1].token_type {
                TokenType::Asterisk => BiOp::Times,
                TokenType::Slash => BiOp::Divided,
                _ => panic!("In factor(): op token_type was not * or /, error probably in check_advance() or factor()"),
            };
            let right = self.unary();
            ex = Expr::Binary(Box::new(ex), op, Box::new(right));
        }
        ex
    }

    // unary -> ("-"|"not") unary | primary
    fn unary(&mut self) -> Expr {
        if self.check_advance(vec![TokenType::Minus, TokenType::Not]) {
            // turn the token into a UnOp
            let op = match self.tokens[self.current_idx - 1].token_type {
                TokenType::Minus => UnOp::Minus,
                TokenType::Not => UnOp::Not,
                _ => panic!("In unary(): op token_type was not - or not, error probably in check_advance() or unary()"),
            };
            let right = self.unary();
            return Expr::Unary(op, Box::new(right));
        }
        self.primary()
    }

    // primary -> NUMBER | STRING | "True" | "False" | "None" | "(" expr ")"
    fn primary(&mut self) -> Expr {
        if self.check_advance(vec![
            TokenType::String("".to_string()),
            TokenType::Int(0),
            TokenType::Float(0.0),
        ]) {
            match &self.tokens[self.current_idx - 1].token_type {
                TokenType::String(s) => return Expr::Literal(Lit::String(s.to_string())),
                TokenType::Int(n) => return Expr::Literal(Lit::Int(*n)),
                TokenType::Float(n) => return Expr::Literal(Lit::Float(*n)),
                _ => panic!("In primary(): op token_type was not String or Int or Float, error probably in check_literal_advance() or primary()"),
            }
        }

        if self.check_advance(vec![TokenType::True]) {
            return Expr::Literal(Lit::True);
        }
        if self.check_advance(vec![TokenType::False]) {
            return Expr::Literal(Lit::False);
        }
        if self.check_advance(vec![TokenType::None]) {
            return Expr::Literal(Lit::None);
        }

        // TODO: add grouping and error if no matching token was found
        Expr::Literal(Lit::String("hehe".to_string()))
    }
}
