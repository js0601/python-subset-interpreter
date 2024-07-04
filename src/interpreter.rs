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

impl Value {
    fn to_bool(&self) -> bool {
        match self {
            Value::Int(n) if *n == 0 => false,
            Value::Float(n) if *n == 0.0 => false,
            Value::String(s) if s.is_empty() => false,
            Value::Bool(b) => *b,
            Value::None => false,
            _ => true,
        }
    }
}

#[derive(Clone)]
struct Function {
    name: Name,
    parameters: Vec<Name>,
    body: Vec<Stmt>,
}

impl Function {
    fn arity(&self) -> usize {
        self.parameters.len()
    }

    fn call(&self, args: Vec<Value>, encl: Environment) -> Result<Value, PyError> {
        let vars: HashMap<String, Value> = self
            .parameters
            .iter()
            .zip(args.iter())
            .map(|(param, arg)| (param.name.clone(), arg.clone()))
            .collect();

        let mut fun_int = Interpreter {
            env: Environment {
                enclosed_by: Some(Box::new(encl)),
                funcs: HashMap::new(),
                vars,
            },
        };

        for st in self.body.clone() {
            match fun_int.interpret_stmt(st) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(Value::None)
    }
}

#[derive(Clone)]
struct Environment {
    enclosed_by: Option<Box<Environment>>,
    funcs: HashMap<String, Function>,
    vars: HashMap<String, Value>,
}

impl Environment {
    fn assign_var(&mut self, name: String, val: Value) {
        self.vars.insert(name, val);
    }

    fn get_var(&self, var: Name) -> Result<Value, PyError> {
        match self.vars.get(&var.name) {
            Some(v) => Ok(v.clone()),
            None => {
                if let Some(e) = &self.enclosed_by {
                    e.get_var(var)
                } else {
                    Err(PyError {
                        msg: format!("NameError: name {} is not defined", var.name),
                        line: var.line,
                        column: var.column,
                    })
                }
            }
        }
    }

    fn assign_fun(&mut self, name: Name, parameters: Vec<Name>, body: Vec<Stmt>) {
        let n = name.name.clone();
        let f = Function {
            name,
            parameters,
            body,
        };
        self.funcs.insert(n, f);
    }

    fn get_fun(&self, fun: Name) -> Result<Function, PyError> {
        match self.funcs.get(&fun.name) {
            Some(f) => Ok(f.clone()),
            None => {
                if let Some(e) = &self.enclosed_by {
                    e.get_fun(fun)
                } else {
                    Err(PyError {
                        msg: format!("NameError: name {} is not defined", fun.name),
                        line: fun.line,
                        column: fun.column,
                    })
                }
            }
        }
    }
}

// entry point, goes through all statements and prints errors
pub fn interpret(stmts: Vec<Stmt>) {
    let mut int = Interpreter {
        env: Environment {
            enclosed_by: None,
            funcs: HashMap::new(),
            vars: HashMap::new(),
        },
    };
    for st in stmts {
        match int.interpret_stmt(st) {
            Ok(_) => continue,
            Err(e) => {
                println!("{e}");
                break;
            }
        }
    }
}

struct Interpreter {
    env: Environment,
}

impl Interpreter {
    // TODO: return values: return Result<Option<Value>, PyError> here
    // at return statement return Ok(Some(Value))
    // at every other statement return Ok(None)
    // in interpret() if it receives Ok(Some(...)) it errors with return outside function, else it continues
    // in function.call() it breaks and returns received value
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
                self.env.assign_var(n.name, val);
                Ok(())
            }
            Stmt::If(c, t, e) => {
                let cond = self.eval_expr(c)?.to_bool();
                if cond {
                    for st in t {
                        self.interpret_stmt(st)?;
                    }
                } else if let Some(stmts) = e {
                    for st in stmts {
                        self.interpret_stmt(st)?;
                    }
                }
                Ok(())
            }
            Stmt::While(c, b) => {
                while self.eval_expr(c.clone())?.to_bool() {
                    for st in b.clone() {
                        self.interpret_stmt(st)?;
                    }
                }
                Ok(())
            }
            Stmt::FunDecl(n, p, b) => {
                self.env.assign_fun(n, p, b);
                Ok(())
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Result<Value, PyError> {
        match expr {
            Expr::Unary(op, e) => self.eval_unary(op, *e),
            Expr::Binary(e1, op, e2) => self.eval_binary(*e1, op, *e2),
            Expr::Grouping(e) => self.eval_expr(*e),
            Expr::Literal(l) => Ok(self.eval_literal(l)),
            Expr::Variable(n) => self.env.get_var(n),
            Expr::Call(n, a) => self.eval_call(n, a),
        }
    }

    fn eval_unary(&mut self, op: UnOp, expr: Expr) -> Result<Value, PyError> {
        let right = self.eval_expr(expr)?;

        match (op.ty, right) {
            (UnOpType::Minus, Value::Int(n)) => Ok(Value::Int(-n)),
            (UnOpType::Minus, Value::Float(n)) => Ok(Value::Float(-n)),
            (UnOpType::Minus, Value::Bool(b)) => Ok(Value::Int(-(b as i128))),
            (UnOpType::Not, a) => Ok(Value::Bool(!a.to_bool())),
            (UnOpType::Minus, Value::String(_)) => Err(PyError {
                msg: "TypeError: Can't apply unary operator - to String".to_owned(),
                line: op.line,
                column: op.column,
            }),
            (UnOpType::Minus, Value::None) => Err(PyError {
                msg: "TypeError: Can't apply unary operator - to None".to_owned(),
                line: op.line,
                column: op.column,
            }),
        }
    }

    fn eval_binary(&mut self, ex1: Expr, op: BiOp, ex2: Expr) -> Result<Value, PyError> {
        let left = self.eval_expr(ex1)?;
        let right = self.eval_expr(ex2)?;

        match op.ty {
            BiOpType::Plus => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f64)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Int(a), Value::Bool(b)) => Ok(Value::Int(a + b as i128)),
                (Value::Bool(a), Value::Int(b)) => Ok(Value::Int(a as i128 + b)),
                (Value::Float(a), Value::Bool(b)) => Ok(Value::Float(a + b as i8 as f64)),
                (Value::Bool(a), Value::Float(b)) => Ok(Value::Float(a as i8 as f64 + b)),
                (Value::Bool(a), Value::Bool(b)) => Ok(Value::Int(a as i128 + b as i128)),
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
                (Value::Int(a), Value::Bool(b)) => Ok(Value::Int(a - b as i128)),
                (Value::Bool(a), Value::Int(b)) => Ok(Value::Int(a as i128 - b)),
                (Value::Float(a), Value::Bool(b)) => Ok(Value::Float(a - b as i8 as f64)),
                (Value::Bool(a), Value::Float(b)) => Ok(Value::Float(a as i8 as f64 - b)),
                (Value::Bool(a), Value::Bool(b)) => Ok(Value::Int(a as i128 - b as i128)),
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
                (Value::Int(a), Value::Bool(b)) => Ok(Value::Int(a * b as i128)),
                (Value::Bool(a), Value::Int(b)) => Ok(Value::Int(a as i128 * b)),
                (Value::Float(a), Value::Bool(b)) => Ok(Value::Float(a * b as i8 as f64)),
                (Value::Bool(a), Value::Float(b)) => Ok(Value::Float(a as i8 as f64 * b)),
                (Value::Bool(a), Value::Bool(b)) => Ok(Value::Int(a as i128 * b as i128)),
                _ => Err(PyError {
                    msg: "TypeError: Can't apply binary operator * here".to_owned(),
                    line: op.line,
                    column: op.column,
                }),
            },
            BiOpType::Divided => match (left, right) {
                (_, Value::Int(0)) | (_, Value::Float(0.0)) | (_, Value::Bool(false)) => {
                    Err(PyError {
                        msg: "ZeroDivisionError: division by zero".to_owned(),
                        line: op.line,
                        column: op.column,
                    })
                }
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
                (Value::Int(a), Value::Bool(b)) => Ok(Value::Int(a / b as i128)),
                (Value::Bool(a), Value::Int(b)) => Ok(Value::Int(a as i128 / b)),
                (Value::Float(a), Value::Bool(b)) => Ok(Value::Float(a / b as i8 as f64)),
                (Value::Bool(a), Value::Float(b)) => Ok(Value::Float(a as i8 as f64 / b)),
                (Value::Bool(a), Value::Bool(b)) => Ok(Value::Int(a as i128 / b as i128)),
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
                _ => Ok(Value::Bool(false)),
                // TODO: in python 1 == True and 0 == False, but other numbers are not equal to either, maybe implement
            },
            BiOpType::NotEqual => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 != b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a != b as f64)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a != b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Bool(a != b)),
                (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),
                (Value::None, Value::None) => Ok(Value::Bool(false)),
                _ => Ok(Value::Bool(true)),
                // TODO: see double equal above
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
            BiOpType::And => {
                if !left.to_bool() {
                    Ok(left)
                } else {
                    Ok(right)
                }
            }
            BiOpType::Or => {
                if left.to_bool() {
                    Ok(left)
                } else {
                    Ok(right)
                }
            }
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

    fn eval_call(&mut self, name: Name, arguments: Vec<Expr>) -> Result<Value, PyError> {
        let f = self.env.get_fun(name.clone())?;
        let mut args = Vec::new();
        for arg in arguments {
            args.push(self.eval_expr(arg)?);
        }

        if f.arity() != args.len() {
            return Err(PyError {
                msg: format!(
                    "TypeError: {} takes {} positional arguments but {} were given",
                    f.name.name,
                    f.arity(),
                    args.len()
                ),
                line: name.line,
                column: name.column,
            });
        }

        f.call(args, self.env.clone())
    }
}
