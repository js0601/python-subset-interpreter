#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Assign(Name, Expr),
}

pub enum Expr {
    Unary(UnOp, Box<Expr>),
    Binary(Box<Expr>, BiOp, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Lit),
    Variable(Name),
}

pub struct Name {
    pub name: String,
    pub line: u64,
    pub column: u64,
}

pub struct UnOp {
    pub ty: UnOpType,
    pub line: u64,
    pub column: u64,
}

pub enum UnOpType {
    Minus,
    Not,
}

pub struct BiOp {
    pub ty: BiOpType,
    pub line: u64,
    pub column: u64,
}

pub enum BiOpType {
    Plus,
    Minus,
    Times,
    Divided,
    DoubleEqual,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

pub enum Lit {
    Int(u64),
    Float(f64),
    String(String),
    True,
    False,
    None,
}

////////////////////////////////////////////////////
// debug trait implementations for nicer printing //
////////////////////////////////////////////////////

use std::fmt;

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Unary(op, ex) => write!(f, "({op:?} {ex:?})"),
            Expr::Binary(ex1, op, ex2) => write!(f, "({op:?} {ex1:?} {ex2:?})"),
            Expr::Grouping(ex) => write!(f, "(group {ex:?})"),
            Expr::Literal(l) => write!(f, "{l:?}"),
            Expr::Variable(n) => write!(f, "{n:?}"),
        }
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Debug for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.ty {
            UnOpType::Minus => write!(f, "-"),
            UnOpType::Not => write!(f, "not"),
        }
    }
}

impl fmt::Debug for BiOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.ty {
            BiOpType::Plus => write!(f, "+"),
            BiOpType::Minus => write!(f, "-"),
            BiOpType::Times => write!(f, "*"),
            BiOpType::Divided => write!(f, "/"),
            BiOpType::DoubleEqual => write!(f, "=="),
            BiOpType::NotEqual => write!(f, "!="),
            BiOpType::Greater => write!(f, ">"),
            BiOpType::GreaterEqual => write!(f, ">="),
            BiOpType::Less => write!(f, "<"),
            BiOpType::LessEqual => write!(f, "<="),
        }
    }
}

impl fmt::Debug for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lit::Int(x) => write!(f, "{x}"),
            Lit::Float(x) => write!(f, "{x}"),
            Lit::String(s) => write!(f, "\"{s}\""),
            Lit::True => write!(f, "True"),
            Lit::False => write!(f, "False"),
            Lit::None => write!(f, "None"),
        }
    }
}
