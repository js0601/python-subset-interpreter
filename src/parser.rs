use crate::common::{ast::*, py_error::*, token::*};

// TODO: only returns one expression for now
pub fn parse(tokens: Vec<Token>) -> Expr {
    // TODO: remove placeholder
    Expr::Literal(Lit::String("nothing".to_string()))
}
