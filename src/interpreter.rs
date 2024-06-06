use crate::common::{ast::*, py_error::*};

#[derive(Debug)]
pub enum Value {
    Int(i128),
    Float(f64),
    String(String),
    Bool(bool),
    None,
}

pub fn interpret(expr: Expr) -> Result<Value, PyError> {
    match expr {
        Expr::Unary(op, e) => eval_unary(op, *e),
        Expr::Binary(e1, op, e2) => eval_binary(*e1, op, *e2),
        Expr::Grouping(e) => interpret(*e),
        Expr::Literal(l) => Ok(eval_literal(l)),
    }
}

fn eval_unary(op: UnOp, expr: Expr) -> Result<Value, PyError> {
    let right = interpret(expr)?;

    match (op, right) {
        (UnOp::Minus, Value::Int(n)) => Ok(Value::Int(-n)),
        (UnOp::Minus, Value::Float(n)) => Ok(Value::Float(-n)),
        // TODO: python doesn't throw an error for this, maybe change it
        (UnOp::Minus, Value::Bool(_)) => Err(PyError {
            msg: "SyntaxError: Can't apply unary operator {op:?} to Boolean".to_owned(),
            line: 1, // TODO: need to carry line/column through somehow, lost it inside parser
            column: 1,
        }),
        (UnOp::Not, Value::Int(n)) => Ok(Value::Bool(n == 0)),
        (UnOp::Not, Value::Float(n)) => Ok(Value::Bool(n == 0.0)),
        (UnOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
        (_, Value::String(_)) => Err(PyError {
            msg: "SyntaxError: Can't apply unary operator {op:?} to String".to_owned(),
            line: 1, // TODO: fix
            column: 1,
        }),
        (_, Value::None) => Err(PyError {
            msg: "SyntaxError: Can't apply unary operator {op:?} to None".to_owned(),
            line: 1, // TODO: fix
            column: 1,
        }),
    }
}

fn eval_binary(ex1: Expr, op: BiOp, ex2: Expr) -> Result<Value, PyError> {
    let left = interpret(ex1)?;
    let right = interpret(ex2)?;

    // TODO: needs to handle e.g. 3 / 2 and 3 / 0
    // TODO: also float loses precision as soon as it does anything
    // TODO: maybe add arithmetic for Booleans (true=1, false=0)
    match op {
        BiOp::Plus => match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)), // TODO: might fail?
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f64)), // TODO: might fail?
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{a}{b}"))),
            _ => Err(PyError {
                msg: "SyntaxError: Can't apply binary operator + here".to_owned(),
                line: 1, // TODO: fix
                column: 1,
            }),
        },
        BiOp::Minus => match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 - b)), // TODO: might fail?
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - b as f64)), // TODO: might fail?
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            _ => Err(PyError {
                msg: "SyntaxError: Can't apply binary operator - here".to_owned(),
                line: 1, // TODO: fix
                column: 1,
            }),
        },
        BiOp::Times => match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 * b)), // TODO: might fail?
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * b as f64)), // TODO: might fail?
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            _ => Err(PyError {
                msg: "SyntaxError: Can't apply binary operator * here".to_owned(),
                line: 1, // TODO: fix
                column: 1,
            }),
        },
        BiOp::Divided => match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 / b)), // TODO: might fail?
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / b as f64)), // TODO: might fail?
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            _ => Err(PyError {
                msg: "SyntaxError: Can't apply binary operator / here".to_owned(),
                line: 1, // TODO: fix
                column: 1,
            }),
        },
        BiOp::DoubleEqual => match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a == b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 == b)), // TODO: might fail?
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a == b as f64)), // TODO: might fail?
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a == b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a == b)),
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a == b)),
            (Value::None, Value::None) => Ok(Value::Bool(true)),
            (Value::None, _) => Ok(Value::Bool(false)),
            (_, Value::None) => Ok(Value::Bool(false)),
            _ => Err(PyError {
                msg: "SyntaxError: Can't apply binary operator == here".to_owned(),
                line: 1, // TODO: fix
                column: 1,
            }),
        },
        BiOp::NotEqual => match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 != b)), // TODO: might fail?
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a != b as f64)), // TODO: might fail?
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a != b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a != b)),
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),
            (Value::None, Value::None) => Ok(Value::Bool(false)),
            (Value::None, _) => Ok(Value::Bool(true)),
            (_, Value::None) => Ok(Value::Bool(true)),
            _ => Err(PyError {
                msg: "SyntaxError: Can't apply binary operator != here".to_owned(),
                line: 1, // TODO: fix
                column: 1,
            }),
        },
        BiOp::Greater => match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 > b)), // TODO: might fail?
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a > b as f64)), // TODO: might fail?
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
            _ => Err(PyError {
                msg: "SyntaxError: Can't apply binary operator > here".to_owned(),
                line: 1, // TODO: fix
                column: 1,
            }),
        },
        BiOp::GreaterEqual => match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 >= b)), // TODO: might fail?
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a >= b as f64)), // TODO: might fail?
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
            _ => Err(PyError {
                msg: "SyntaxError: Can't apply binary operator >= here".to_owned(),
                line: 1, // TODO: fix
                column: 1,
            }),
        },
        BiOp::Less => match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) < b)), // TODO: might fail?
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a < b as f64)), // TODO: might fail?
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
            _ => Err(PyError {
                msg: "SyntaxError: Can't apply binary operator < here".to_owned(),
                line: 1, // TODO: fix
                column: 1,
            }),
        },
        BiOp::LessEqual => match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 <= b)), // TODO: might fail?
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a <= b as f64)), // TODO: might fail?
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
            _ => Err(PyError {
                msg: "SyntaxError: Can't apply binary operator <= here".to_owned(),
                line: 1, // TODO: fix
                column: 1,
            }),
        },
    }
}

fn eval_literal(lit: Lit) -> Value {
    match lit {
        Lit::Int(n) => Value::Int(n.into()),
        Lit::Float(n) => Value::Float(n),
        Lit::String(s) => Value::String(s),
        Lit::True => Value::Bool(true),
        Lit::False => Value::Bool(false),
        Lit::None => Value::None,
    }
}
