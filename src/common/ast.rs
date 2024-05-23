use std::fmt;

pub enum Expr {
    Unary(UnOp, Box<Expr>),
    Binary(Box<Expr>, BiOp, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Lit),
}

pub enum UnOp {
    Minus,
    Not,
}

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

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Unary(op, ex) => write!(f, "({op:?} {ex:?})"),
            Expr::Binary(ex1, op, ex2) => write!(f, "({op:?} {ex1:?} {ex2:?})"),
            Expr::Grouping(ex) => write!(f, "(group {ex:?})"),
            Expr::Literal(l) => write!(f, "{l:?}"),
        }
    }
}

impl fmt::Debug for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnOp::Minus => write!(f, "-"),
            UnOp::Not => write!(f, "not"),
        }
    }
}

impl fmt::Debug for BiOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BiOp::Plus => write!(f, "+"),
            BiOp::Minus => write!(f, "-"),
            BiOp::Times => write!(f, "*"),
            BiOp::Divided => write!(f, "/"),
            BiOp::DoubleEqual => write!(f, "=="),
            BiOp::NotEqual => write!(f, "!="),
            BiOp::Greater => write!(f, ">"),
            BiOp::GreatEqual => write!(f, ">="),
            BiOp::Less => write!(f, "<"),
            BiOp::LessEqual => write!(f, "<="),
        }
    }
}

impl fmt::Debug for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lit::Int(x) => write!(f, "{x}"),
            Lit::Float(x) => write!(f, "{x}"),
            Lit::String(s) => write!(f, "{s}"),
            Lit::True => write!(f, "True"),
            Lit::False => write!(f, "False"),
            Lit::None => write!(f, "None"),
        }
    }
}
