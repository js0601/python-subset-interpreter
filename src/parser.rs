// see grammar.txt for grammar

use crate::common::{ast::*, py_error::*, token::*};

#[derive(Default)]
struct Parser {
    tokens: Vec<Token>,
    current_idx: usize,
}

impl Parser {
    // check if current token has one of the types
    fn check_type(&self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.tokens[self.current_idx].token_type == t {
                return true;
            }
        }
        false
    }

    // check_type but it moves the idx
    fn check_advance(&mut self, types: Vec<TokenType>) -> bool {
        if self.check_type(types) {
            self.current_idx += 1;
            return true;
        }
        false
    }
}

// TODO: only returns one expression for now
pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut p = Parser {
        tokens,
        ..Default::default()
    };

    expression(&mut p)
}

// expr -> equality
fn expression(p: &mut Parser) -> Expr {
    equality(p)
}

// equality -> comparison (("=="|"!=") comparison)*
fn equality(p: &mut Parser) -> Expr {
    let mut ex = comparison(p);
    while p.check_advance(vec![TokenType::DoubleEqual, TokenType::NotEqual]) {
        // turn the token into a BiOp
        let op = match p.tokens[p.current_idx - 1].token_type {
            TokenType::DoubleEqual => BiOp::DoubleEqual,
            TokenType::NotEqual => BiOp::NotEqual,
            _ => panic!("In equality(): op token_type was not == or !="), // TODO: better error handling
        };
        let right = comparison(p);
        ex = Expr::Binary(Box::new(ex), op, Box::new(right));
    }
    ex
}

// comparison -> term ((">"|">="|"<"|">=") term)*
fn comparison(p: &mut Parser) -> Expr {
    Expr::Literal(Lit::String("nothing".to_string()))
}
