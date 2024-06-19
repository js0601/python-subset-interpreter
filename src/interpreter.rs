use std::{collections::HashMap, fmt};

use crate::common::{ast::*, py_error::*};

#[derive(Debug, Clone)]
pub enum Value {
    Int(i128),
    Float(f64),
    String(String),
    Bool(bool),
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{n}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::None => write!(f, "None"),
        }
    }
}

struct Environment {
    // TODO: when I get to functions, this needs some way to scope variables
    // maybe a Option<Box<Environment>> to point at Env that encloses this one
    // would need to change get method to first search the env on which it is called, then recursively the one enclosing it, until None in which case it errors
    vars: HashMap<String, Value>,
}

impl Environment {
    fn assign(&mut self, name: String, val: Value) {
        self.vars.insert(name, val);
    }

    fn get(&mut self, var: Name) -> Result<Value, PyError> {
        match self.vars.get(&var.name) {
            Some(v) => Ok(v.clone()),
            None => Err(PyError {
                msg: format!("NameError: name {} is not defined", var.name),
                line: var.line,
                column: var.column,
            }),
        }
    }
}

// entry point, goes through all statements and prints errors
pub fn interpret(stmts: Vec<Stmt>) {
    let mut int = Interpreter {
        env: Environment {
            vars: HashMap::new(),
        },
    };
    for st in stmts {
        match int.interpret_stmt(st) {
            Ok(_) => continue,
            Err(e) => {
                println!("{e}");
                break; // TODO: maybe continue?
            }
        }
    }
}

struct Interpreter {
    env: Environment,
}

impl Interpreter {
    fn interpret_stmt(&mut self, stmt: Stmt) -> Result<(), PyError> {
        match stmt {
            Stmt::Expr(e) => {
                self.eval_expr(e)?;
                Ok(())
            }
            Stmt::Print(e) => {
                let val = self.eval_expr(e)?;
                println!("{val}");
                Ok(())
            }
            Stmt::Assign(n, e) => {
                let val = self.eval_expr(e)?;
                self.env.assign(n.name, val);
                Ok(())
            }
            Stmt::If(_, _, _) => todo!(),
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Result<Value, PyError> {
        match expr {
            Expr::Unary(op, e) => self.eval_unary(op, *e),
            Expr::Binary(e1, op, e2) => self.eval_binary(*e1, op, *e2),
            Expr::Grouping(e) => self.eval_expr(*e),
            Expr::Literal(l) => Ok(self.eval_literal(l)),
            Expr::Variable(n) => self.env.get(n),
        }
    }

    fn eval_unary(&mut self, op: UnOp, expr: Expr) -> Result<Value, PyError> {
        let right = self.eval_expr(expr)?;

        match (op.ty, right) {
            (UnOpType::Minus, Value::Int(n)) => Ok(Value::Int(-n)),
            (UnOpType::Minus, Value::Float(n)) => Ok(Value::Float(-n)),
            // TODO: python doesn't throw an error for this, maybe change it
            (UnOpType::Minus, Value::Bool(_)) => Err(PyError {
                msg: "TypeError: Can't apply unary operator - to Boolean".to_owned(),
                line: op.line,
                column: op.column,
            }),
            (UnOpType::Not, Value::Int(n)) => Ok(Value::Bool(n == 0)),
            (UnOpType::Not, Value::Float(n)) => Ok(Value::Bool(n == 0.0)),
            (UnOpType::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
            (_, Value::String(_)) => Err(PyError {
                msg: "TypeError: Can't apply unary operator to String".to_owned(),
                line: op.line,
                column: op.column,
            }),
            (_, Value::None) => Err(PyError {
                msg: "TypeError: Can't apply unary operator to None".to_owned(),
                line: op.line,
                column: op.column,
            }),
        }
    }

    fn eval_binary(&mut self, ex1: Expr, op: BiOp, ex2: Expr) -> Result<Value, PyError> {
        let left = self.eval_expr(ex1)?;
        let right = self.eval_expr(ex2)?;

        // TODO: maybe add arithmetic for Booleans (true=1, false=0)
        match op.ty {
            BiOpType::Plus => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f64)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{a}{b}"))),
                _ => Err(PyError {
                    msg: "TypeError: Can't apply binary operator + here".to_owned(),
                    line: op.line,
                    column: op.column,
                }),
            },
            BiOpType::Minus => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - b as f64)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                _ => Err(PyError {
                    msg: "TypeError: Can't apply binary operator - here".to_owned(),
                    line: op.line,
                    column: op.column,
                }),
            },
            BiOpType::Times => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * b as f64)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                _ => Err(PyError {
                    msg: "TypeError: Can't apply binary operator * here".to_owned(),
                    line: op.line,
                    column: op.column,
                }),
            },
            BiOpType::Divided => match (left, right) {
                (_, Value::Int(0)) | (_, Value::Float(0.0)) => Err(PyError {
                    msg: "ZeroDivisionError: division by zero".to_owned(),
                    line: op.line,
                    column: op.column,
                }),
                (Value::Int(a), Value::Int(b)) => {
                    if a % b == 0 {
                        Ok(Value::Int(a / b))
                    } else {
                        Ok(Value::Float(a as f64 / b as f64))
                    }
                }
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 / b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / b as f64)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
                _ => Err(PyError {
                    msg: "TypeError: Can't apply binary operator / here".to_owned(),
                    line: op.line,
                    column: op.column,
                }),
            },
            BiOpType::DoubleEqual => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a == b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 == b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a == b as f64)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a == b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Bool(a == b)),
                (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a == b)),
                (Value::None, Value::None) => Ok(Value::Bool(true)),
                (Value::None, _) => Ok(Value::Bool(false)),
                (_, Value::None) => Ok(Value::Bool(false)),
                _ => Err(PyError {
                    msg: "TypeError: Can't apply binary operator == here".to_owned(),
                    line: op.line,
                    column: op.column,
                }),
            },
            BiOpType::NotEqual => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 != b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a != b as f64)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a != b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Bool(a != b)),
                (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),
                (Value::None, Value::None) => Ok(Value::Bool(false)),
                (Value::None, _) => Ok(Value::Bool(true)),
                (_, Value::None) => Ok(Value::Bool(true)),
                _ => Err(PyError {
                    msg: "TypeError: Can't apply binary operator != here".to_owned(),
                    line: op.line,
                    column: op.column,
                }),
            },
            BiOpType::Greater => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 > b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a > b as f64)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
                _ => Err(PyError {
                    msg: "TypeError: Can't apply binary operator > here".to_owned(),
                    line: op.line,
                    column: op.column,
                }),
            },
            BiOpType::GreaterEqual => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 >= b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a >= b as f64)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
                _ => Err(PyError {
                    msg: "TypeError: Can't apply binary operator >= here".to_owned(),
                    line: op.line,
                    column: op.column,
                }),
            },
            BiOpType::Less => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) < b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a < b as f64)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
                _ => Err(PyError {
                    msg: "TypeError: Can't apply binary operator < here".to_owned(),
                    line: op.line,
                    column: op.column,
                }),
            },
            BiOpType::LessEqual => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 <= b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a <= b as f64)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
                _ => Err(PyError {
                    msg: "TypeError: Can't apply binary operator <= here".to_owned(),
                    line: op.line,
                    column: op.column,
                }),
            },
        }
    }

    fn eval_literal(&mut self, lit: Lit) -> Value {
        match lit {
            Lit::Int(n) => Value::Int(n.into()),
            Lit::Float(n) => Value::Float(n),
            Lit::String(s) => Value::String(s),
            Lit::True => Value::Bool(true),
            Lit::False => Value::Bool(false),
            Lit::None => Value::None,
        }
    }
}
