use crate::common::{ast::*, py_error::PyError, token::*};

pub fn parse(tokens: Vec<Token>) -> Option<Vec<Stmt>> {
    let mut p = Parser {
        tokens,
        current_idx: 0,
    };
    let mut statements = Vec::new();
    let mut error = false;

    while !p.check_type(vec![TokenType::EndOfFile]) {
        // ignore end of lines that haven't been parsed in a rule in order to ignore blank lines
        if p.check_advance(vec![TokenType::EndOfLine]) {
            continue;
        }
        match p.statement() {
            Ok(s) => statements.push(s),
            Err(e) => {
                // advance to next line on error
                while !p.check_type(vec![TokenType::EndOfLine, TokenType::EndOfFile]) {
                    p.current_idx += 1;
                }
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
    /////////////
    // grammar //
    /////////////
    // see grammar.txt

    // stmt -> exprStmt | printStmt | assignStmt
    fn statement(&mut self) -> Result<Stmt, PyError> {
        if self.check_advance(vec![TokenType::Print]) {
            return self.print_statement();
        }

        // only go to assignStmt if there is an id followed by =, else go to exprStmt
        if self.check_advance(vec![TokenType::Identifier("".to_owned())]) {
            if self.check_advance(vec![TokenType::Equal]) {
                return self.assign_statement();
            } else {
                // needed to idx points back at identifier and doesn't skip it
                self.current_idx -= 1;
            }
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

    // assignStmt -> IDENTIFIER "=" expr "\n"
    fn assign_statement(&mut self) -> Result<Stmt, PyError> {
        // guaranteed to be identifier because of ifs in statement()
        let id = &self.tokens[self.current_idx - 2];
        let name = Name {name: id.value.to_owned(), line: id.line, column: id.column };

        let ex = self.expression()?;
        self.check_or_error(vec![TokenType::EndOfLine], "SyntaxError: no newline after statement found".to_owned())?;

        Ok(Stmt::Assign(name, ex))
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

    // primary -> NUMBER | STRING | "True" | "False" | "None" | "(" expr ")" | IDENTIFIER
    fn primary(&mut self) -> Result<Expr, PyError> {
        if self.check_advance(vec![
            TokenType::Identifier("".to_owned()),
            TokenType::String("".to_owned()),
            TokenType::Int(0),
            TokenType::Float(0.0),
        ]) {
            let previous_tok = &self.tokens[self.current_idx - 1];
            match &previous_tok.token_type {
                TokenType::Identifier(n) => return Ok(Expr::Variable(Name { name: n.to_owned(), line: previous_tok.line, column: previous_tok.column})),
                TokenType::String(s) => return Ok(Expr::Literal(Lit::String(s.to_owned()))),
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
}
