#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    AssignVar(Name, Expr),
    AssignList(Name, Expr, Expr),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    While(Expr, Vec<Stmt>),
    FunDecl(Name, Vec<Name>, Vec<Stmt>),
    Return(Location, Option<Expr>),
}

#[derive(Clone)]
pub enum Expr {
    Unary(UnOp, Box<Expr>),
    Binary(Box<Expr>, BiOp, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Lit),
    Variable(Name),
    Call(Name, Vec<Expr>),
    ListAccess(Name, Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct Location {
    pub line: u64,
    pub column: u64,
}

#[derive(Clone)]
pub struct Name {
    pub name: String,
    pub line: u64,
    pub column: u64,
}

#[derive(Clone)]
pub struct UnOp {
    pub ty: UnOpType,
    pub line: u64,
    pub column: u64,
}

#[derive(Clone)]
pub enum UnOpType {
    Minus,
    Not,
}

#[derive(Clone)]
pub struct BiOp {
    pub ty: BiOpType,
    pub line: u64,
    pub column: u64,
}

#[derive(Clone)]
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
    And,
    Or,
}

#[derive(Clone)]
pub enum Lit {
    Int(u64),
    Float(f64),
    String(String),
    List(Vec<Expr>),
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
            Expr::Call(n, p) => write!(f, "{n:?}({p:?})"),
            Expr::ListAccess(n, i) => write!(f, "{n:?}[{i:?}]"),
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
            BiOpType::And => write!(f, "and"),
            BiOpType::Or => write!(f, "or"),
        }
    }
}

impl fmt::Debug for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lit::Int(x) => write!(f, "{x}"),
            Lit::Float(x) => write!(f, "{x}"),
            Lit::String(s) => write!(f, "\"{s}\""),
            Lit::List(l) => write!(f, "{l:?}"),
            Lit::True => write!(f, "True"),
            Lit::False => write!(f, "False"),
            Lit::None => write!(f, "None"),
        }
    }
}
