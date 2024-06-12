use crate::common::{ast::*, py_error::PyError, token::*};

// TODO: only returns one expression for now
pub fn parse(tokens: Vec<Token>) -> Option<Vec<Stmt>> {
    let mut p = Parser {
        tokens,
        current_idx: 0,
    };
    let mut statements = Vec::new();
    let mut error = false;

    // TODO: error at end infinitely loops here for e.g. print 1)
    while !p.check_type(vec![TokenType::EndOfFile]) {
        match p.statement() {
            Ok(s) => statements.push(s),
            Err(e) => {
                // TODO: here probably needs to advance to next line or smth, else I think EoL gets parsed on repeat
                p.current_idx += 1; // stops loop eventually
                error = true;
                println!("{e}");
            },
        }
    }

    if error {
        None
    } else {
        Some(statements)
    }
}

struct Parser {
    tokens: Vec<Token>,
    current_idx: usize,
}

impl Parser {
    //////////////////////
    // helper functions //
    //////////////////////

    // checks if current token has one of the types
    fn check_type(&self, types: Vec<TokenType>) -> bool {
        // not very pretty, but needed some way to eliminate value inside literal types
        match &self.tokens[self.current_idx].token_type {
            TokenType::Identifier(_) => types.iter().any(|t| matches!(t, TokenType::Identifier(_))),
            TokenType::String(_) => types.iter().any(|t| matches!(t, TokenType::String(_))),
            TokenType::Int(_) => types.iter().any(|t| matches!(t, TokenType::Int(_))),
            TokenType::Float(_) => types.iter().any(|t| matches!(t, TokenType::Float(_))),
            other => types.contains(other),
        }
    }

    // check_type but it moves the idx
    fn check_advance(&mut self, types: Vec<TokenType>) -> bool {
        if self.check_type(types) {
            self.current_idx += 1;
            return true;
        }
        false
    }

    // check_advance but outputs error on False
    fn check_or_error(&mut self, types: Vec<TokenType>, msg: String) -> Result<(), PyError> {
        if !self.check_advance(types) {
            return Err(PyError {
                msg,
                line: self.tokens[self.current_idx].line,
                column: self.tokens[self.current_idx].column,
            })
        }
        Ok(())
    }

    /////////////
    // grammar //
    /////////////
    // see grammar.txt

    // stmt -> exprStmt | printStmt
    fn statement(&mut self) -> Result<Stmt, PyError> {
        if self.check_advance(vec![TokenType::Print]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    // exprStmt -> expr "\n"
    fn expression_statement(&mut self) -> Result<Stmt, PyError> {
        let ex = self.expression()?;
        self.check_or_error(vec![TokenType::EndOfLine], "SyntaxError: no newline after statement found".to_owned())?;
        Ok(Stmt::Expr(ex))
    }

    // printStmt -> "print" "(" expr ")" "\n"
    fn print_statement(&mut self) -> Result<Stmt, PyError> {
        // print already consumed in statement
        self.check_or_error(vec![TokenType::LeftParen], "SyntaxError: missing ( in call to print".to_owned())?;
        let ex = self.expression()?;
        self.check_or_error(vec![TokenType::RightParen], "SyntaxError: missing ) in call to print".to_owned())?;
        self.check_or_error(vec![TokenType::EndOfLine], "SyntaxError: no newline after statement found".to_owned())?;
        Ok(Stmt::Print(ex))
    }

    // expr -> equality
    fn expression(&mut self) -> Result<Expr, PyError> {
        self.equality()
    }

    // equality -> comparison (("=="|"!=") comparison)*
    fn equality(&mut self) -> Result<Expr, PyError> {
        let mut ex = self.comparison()?;
        while self.check_advance(vec![TokenType::DoubleEqual, TokenType::NotEqual]) {
            // turn the token into a BiOp
            let tok = &self.tokens[self.current_idx - 1];
            let op = match tok.token_type {
                TokenType::DoubleEqual => BiOp {
                    ty: BiOpType::DoubleEqual,
                    line: tok.line,
                    column: tok.column,
                },
                TokenType::NotEqual => BiOp {
                    ty: BiOpType::NotEqual,
                    line: tok.line,
                    column: tok.column,
                },
                _ => panic!("In equality(): op token_type was not == or !=, error probably in check_advance() or equality()"),
            };
            let right = self.comparison()?;
            ex = Expr::Binary(Box::new(ex), op, Box::new(right));
        }
        Ok(ex)
    }

    // comparison -> term ((">"|">="|"<"|">=") term)*
    fn comparison(&mut self) -> Result<Expr, PyError> {
        let mut ex = self.term()?;
        while self.check_advance(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            // turn the token into a BiOp
            let tok = &self.tokens[self.current_idx - 1];
            let op = match tok.token_type {
                TokenType::Greater => BiOp {
                    ty: BiOpType::Greater,
                    line: tok.line,
                    column: tok.column,
                },
                TokenType::GreaterEqual => BiOp {
                    ty: BiOpType::GreaterEqual,
                    line: tok.line,
                    column: tok.column,
                },
                TokenType::Less => BiOp {
                    ty: BiOpType::Less,
                    line: tok.line,
                    column: tok.column,
                },
                TokenType::LessEqual => BiOp {
                    ty: BiOpType::LessEqual,
                    line: tok.line,
                    column: tok.column,
                },
                _ => panic!("In comparison(): op token_type was not <,<=,> or >=, error probably in check_advance() or comparison()"),
            };
            let right = self.term()?;
            ex = Expr::Binary(Box::new(ex), op, Box::new(right));
        }
        Ok(ex)
    }

    // term -> factor (("+"|"-") factor)*
    fn term(&mut self) -> Result<Expr, PyError> {
        let mut ex = self.factor()?;
        while self.check_advance(vec![TokenType::Plus, TokenType::Minus]) {
            // turn the token into a BiOp
            let tok = &self.tokens[self.current_idx - 1];
            let op = match tok.token_type {
                TokenType::Plus => BiOp {
                    ty: BiOpType::Plus,
                    line: tok.line,
                    column: tok.column,
                },
                TokenType::Minus => BiOp {
                    ty: BiOpType::Minus,
                    line: tok.line,
                    column: tok.column,
                },
                _ => panic!("In term(): op token_type was not + or -, error probably in check_advance() or term()"),
            };
            let right = self.factor()?;
            ex = Expr::Binary(Box::new(ex), op, Box::new(right));
        }
        Ok(ex)
    }

    // factor -> unary (("*"|"/") unary)*
    fn factor(&mut self) -> Result<Expr, PyError> {
        let mut ex = self.unary()?;
        while self.check_advance(vec![TokenType::Asterisk, TokenType::Slash]) {
            // turn the token into a BiOp
            let tok = &self.tokens[self.current_idx - 1];
            let op = match tok.token_type {
                TokenType::Asterisk => BiOp {
                    ty: BiOpType::Times,
                    line: tok.line,
                    column: tok.column,
                },
                TokenType::Slash => BiOp {
                    ty: BiOpType::Divided,
                    line: tok.line,
                    column: tok.column,
                },
                _ => panic!("In factor(): op token_type was not * or /, error probably in check_advance() or factor()"),
            };
            let right = self.unary()?;
            ex = Expr::Binary(Box::new(ex), op, Box::new(right));
        }
        Ok(ex)
    }

    // unary -> ("-"|"not") unary | primary
    fn unary(&mut self) -> Result<Expr, PyError> {
        if self.check_advance(vec![TokenType::Minus, TokenType::Not]) {
            // turn the token into a UnOp
            let tok = &self.tokens[self.current_idx - 1];
            let op = match tok.token_type {
                TokenType::Minus => UnOp { 
                    ty: UnOpType::Minus,
                    line: tok.line,
                    column: tok.column,
                },
                TokenType::Not => UnOp {
                    ty: UnOpType::Not,
                    line: tok.line,
                    column: tok.column,
                },
                _ => panic!("In unary(): op token_type was not - or not, error probably in check_advance() or unary()"),
            };
            let right = self.unary()?;
            return Ok(Expr::Unary(op, Box::new(right)));
        }
        self.primary()
    }

    // primary -> NUMBER | STRING | "True" | "False" | "None" | "(" expr ")"
    fn primary(&mut self) -> Result<Expr, PyError> {
        if self.check_advance(vec![
            TokenType::String("".to_string()),
            TokenType::Int(0),
            TokenType::Float(0.0),
        ]) {
            match &self.tokens[self.current_idx - 1].token_type {
                TokenType::String(s) => return Ok(Expr::Literal(Lit::String(s.to_string()))),
                TokenType::Int(n) => return Ok(Expr::Literal(Lit::Int(*n))),
                TokenType::Float(n) => return Ok(Expr::Literal(Lit::Float(*n))),
                _ => panic!("In primary(): op token_type was not String or Int or Float, error probably in check_advance() or primary()"),
            }
        }

        if self.check_advance(vec![TokenType::True]) {
            return Ok(Expr::Literal(Lit::True));
        }
        if self.check_advance(vec![TokenType::False]) {
            return Ok(Expr::Literal(Lit::False));
        }
        if self.check_advance(vec![TokenType::None]) {
            return Ok(Expr::Literal(Lit::None));
        }

        if self.check_advance(vec![TokenType::LeftParen]) {
            let ex = self.expression()?;
            self.check_or_error(vec![TokenType::RightParen], "SyntaxError: Missing closing parentheses".to_owned())?;
            return Ok(Expr::Grouping(Box::new(ex)));
        }

        Err(PyError {
            msg: "SyntaxError: Unexpected or missing token".to_owned(),
            line: self.tokens[self.current_idx].line,
            column: self.tokens[self.current_idx].column, // TODO: sometimes points at wrong column
        })
    }
}
