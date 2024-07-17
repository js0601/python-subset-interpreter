use std::{collections::HashMap, fmt};

use crate::common::{ast::*, py_error::*};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i128),
    Float(f64),
    String(String),
    Bool(bool),
    List(Vec<Value>),
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{n}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::List(e) => {
                let elems: Vec<String> = e.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", elems.join(", "))
            }
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
            Value::List(l) if l.is_empty() => false,
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
                Ok(None) => continue,
                Ok(Some((_, v))) => return Ok(v),
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
            Ok(None) => continue,
            Ok(Some((l, _))) => {
                let e = PyError {
                    msg: "SyntaxError: return statement outside of function".to_owned(),
                    line: l.line,
                    column: l.column,
                };
                println!("{e}");
                break;
            }
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
    fn interpret_stmt(&mut self, stmt: Stmt) -> Result<Option<(Location, Value)>, PyError> {
        match stmt {
            Stmt::Expr(e) => {
                self.eval_expr(e)?;
                Ok(None)
            }
            Stmt::Print(e) => {
                let val = self.eval_expr(e)?;
                println!("{val}");
                Ok(None)
            }
            Stmt::Assign(n, e) => {
                let val = self.eval_expr(e)?;
                self.env.assign_var(n.name, val);
                Ok(None)
            }
            Stmt::If(c, t, e) => {
                let cond = self.eval_expr(c)?.to_bool();
                if cond {
                    for st in t {
                        let res = self.interpret_stmt(st);
                        match res {
                            Ok(None) => continue,
                            _ => return res,
                        }
                    }
                } else if let Some(stmts) = e {
                    for st in stmts {
                        let res = self.interpret_stmt(st);
                        match res {
                            Ok(None) => continue,
                            _ => return res,
                        }
                    }
                }
                Ok(None)
            }
            Stmt::While(c, b) => {
                while self.eval_expr(c.clone())?.to_bool() {
                    for st in b.clone() {
                        let res = self.interpret_stmt(st);
                        match res {
                            Ok(None) => continue,
                            _ => return res,
                        }
                    }
                }
                Ok(None)
            }
            Stmt::FunDecl(n, p, b) => {
                self.env.assign_fun(n, p, b);
                Ok(None)
            }
            Stmt::Return(l, e) => {
                if let Some(ex) = e {
                    let val = self.eval_expr(ex)?;
                    Ok(Some((l, val)))
                } else {
                    Ok(Some((l, Value::None)))
                }
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Result<Value, PyError> {
        match expr {
            Expr::Unary(op, e) => self.eval_unary(op, *e),
            Expr::Binary(e1, op, e2) => self.eval_binary(*e1, op, *e2),
            Expr::Grouping(e) => self.eval_expr(*e),
            Expr::Literal(l) => self.eval_literal(l),
            Expr::Variable(n) => self.env.get_var(n),
            Expr::Call(n, a) => self.eval_call(n, a),
            Expr::ListAccess(n, i) => self.eval_access(n, *i),
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
            (UnOpType::Minus, Value::List(_)) => Err(PyError {
                msg: "TypeError: Can't apply unary operator - to List".to_owned(),
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
                (Value::List(a), Value::List(b)) => {
                    // TODO: better way???
                    let mut a = a.clone();
                    let mut b = b.clone();
                    a.append(&mut b);
                    Ok(Value::List(a))
                }
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
                (Value::List(a), Value::List(b)) => Ok(Value::Bool(a == b)),
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
                (Value::List(a), Value::List(b)) => Ok(Value::Bool(a != b)),
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

    fn eval_literal(&mut self, lit: Lit) -> Result<Value, PyError> {
        match lit {
            Lit::Int(n) => Ok(Value::Int(n.into())),
            Lit::Float(n) => Ok(Value::Float(n)),
            Lit::String(s) => Ok(Value::String(s)),
            Lit::True => Ok(Value::Bool(true)),
            Lit::False => Ok(Value::Bool(false)),
            Lit::List(elems) => {
                let mut list = vec![];
                for e in elems {
                    let el = self.eval_expr(e)?;
                    list.push(el);
                }
                Ok(Value::List(list))
            }
            Lit::None => Ok(Value::None),
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

    fn eval_access(&mut self, name: Name, idx_ex: Expr) -> Result<Value, PyError> {
        let idx_val = self.eval_expr(idx_ex)?;
        let idx;
        if let Value::Int(i) = idx_val {
            idx = i;
        } else {
            return Err(PyError {
                msg: "TypeError: List index must be an integer value".to_owned(),
                line: name.line,
                column: name.column,
            });
        }

        let list_val = self.env.get_var(name.clone())?;
        let list;
        if let Value::List(l) = list_val {
            list = l;
        } else {
            return Err(PyError {
                msg: format!(
                    "TypeError: {} is not indexable, because it is not a list",
                    name.name
                ),
                line: name.line,
                column: name.column,
            });
        }

        if idx as usize >= list.len() {
            return Err(PyError {
                msg: "IndexError: Index out of bounds".to_owned(),
                line: name.line,
                column: name.column,
            });
        }
        if idx < 0 {
            return Err(PyError {
                msg: "IndexError: Index below zero".to_owned(),
                line: name.line,
                column: name.column,
            });
        }

        Ok(list[idx as usize].clone())
    }
}
