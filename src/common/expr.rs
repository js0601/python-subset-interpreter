#[derive(Debug)]
pub enum Expr {
    Unary(UnOp, Box<Expr>),
    Binary(Box<Expr>, BiOp, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Lit),
}

#[derive(Debug)]
pub enum UnOp {
    Minus,
    Not,
}

#[derive(Debug)]
pub enum BiOp {
    Plus,
    Minus,
    Times,
    Divided,
    DoubleEqual,
    NotEqual,
    Greater,
    GreatEqual,
    Less,
    LessEqual,
}

#[derive(Debug)]
pub enum Lit {
    Int(u64),
    Float(f64),
    String(String),
    True,
    False,
    None,
}
