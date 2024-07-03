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
                // NOTE: not harsh enough because some statements span more than one line
                // because of this parser breaks for now to prevent cascading errors
                // advance to next line on error
                // while !p.check_type(vec![TokenType::EndOfLine, TokenType::EndOfFile]) {
                //     p.current_idx += 1;
                // }
                error = true;
                println!("{e}");
                break;
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

    // stmt -> exprStmt | printStmt | assignStmt | ifStmt | whileStmt | funDecl
    fn statement(&mut self) -> Result<Stmt, PyError> {
        if self.check_advance(vec![TokenType::Print]) {
            return self.print_statement();
        }
        // only go to assignStmt if there is an id followed by =
        if self.check_advance(vec![TokenType::Identifier("".to_owned())]) {
            if self.check_advance(vec![TokenType::Equal]) {
                return self.assign_statement();
            } else {
                // needed so idx points back at identifier and doesn't skip it
                self.current_idx -= 1;
            }
        }
        if self.check_advance(vec![TokenType::If]) {
            return self.if_statement();
        }
        if self.check_advance(vec![TokenType::While]) {
            return self.while_statement();
        }
        if self.check_advance(vec![TokenType::Def]) {
            return self.function_declaration();
        }

        self.expression_statement()
    }

    // exprStmt -> expr "\n"
    fn expression_statement(&mut self) -> Result<Stmt, PyError> {
        let ex = self.expression()?;
        self.check_or_error(vec![TokenType::EndOfLine], "SyntaxError: unexpected or missing token after statement (expected newline)".to_owned())?;

        Ok(Stmt::Expr(ex))
    }

    // printStmt -> "print" "(" expr ")" "\n"
    fn print_statement(&mut self) -> Result<Stmt, PyError> {
        // print already consumed in statement
        self.check_or_error(vec![TokenType::LeftParen], "SyntaxError: missing ( in call to print".to_owned())?;
        let ex = self.expression()?;
        self.check_or_error(vec![TokenType::RightParen], "SyntaxError: missing ) in call to print".to_owned())?;
        self.check_or_error(vec![TokenType::EndOfLine], "SyntaxError: unexpected or missing token after statement (expected newline)".to_owned())?;

        Ok(Stmt::Print(ex))
    }

    // assignStmt -> IDENTIFIER "=" expr "\n"
    fn assign_statement(&mut self) -> Result<Stmt, PyError> {
        // guaranteed to be identifier because of ifs in statement()
        let id = &self.tokens[self.current_idx - 2];
        let name = Name {name: id.value.to_owned(), line: id.line, column: id.column };

        let ex = self.expression()?;
        self.check_or_error(vec![TokenType::EndOfLine], "SyntaxError: unexpected or missing token after statement (expected newline)".to_owned())?;

        Ok(Stmt::Assign(name, ex))
    }

    // ifStmt -> "if" expr ":" block ("else" ":" block)?
    fn if_statement(&mut self) -> Result<Stmt, PyError> {
        let cond = self.expression()?;
        self.check_or_error(vec![TokenType::Colon], "SyntaxError: missing colon or expression after if statement".to_owned())?;

        let then = self.block()?;
        let maybe_else = if self.check_advance(vec![TokenType::Else]) {
            self.check_or_error(vec![TokenType::Colon], "SyntaxError: missing colon after else statement".to_owned())?;
            Some(self.block()?)
        } else {
            None
        };

        Ok(Stmt::If(cond, then, maybe_else))
    }

    // whileStmt -> "while" expr ":" block
    fn while_statement(&mut self) -> Result<Stmt, PyError> {
        let cond = self.expression()?;
        self.check_or_error(vec![TokenType::Colon], "SyntaxError: missing colon or expression after while statement".to_owned())?;
        let block = self.block()?;
        Ok(Stmt::While(cond, block))
    }

    // funDecl -> "def" IDENTIFIER "(" parameters? ")" ":" block
    fn function_declaration(&mut self) -> Result<Stmt, PyError> {
        self.check_or_error(vec![TokenType::Identifier("".to_owned())], "SyntaxError: missing name in def statement".to_owned())?;
        let id_tok = &self.tokens[self.current_idx - 1];
        let name;
        if let TokenType::Identifier(n) = &id_tok.token_type {
            name = Name { name: n.to_owned(), line: id_tok.line, column: id_tok.column };
        } else {
            panic!("expected Identifier token here");
        }

        self.check_or_error(vec![TokenType::LeftParen], "SyntaxError: missing ( in def statement".to_owned())?;
        let params = self.parameters()?;
        self.check_or_error(vec![TokenType::RightParen], "SyntaxError: missing ) in def statement".to_owned())?;

        self.check_or_error(vec![TokenType::Colon], "SyntaxError: missing colon after def statement".to_owned())?;
        let body = self.block()?;

        Ok(Stmt::FunDecl(name, params, body))
    }

    // parameters -> IDENTIFIER ("," IDENTIFIER)*
    fn parameters(&mut self) -> Result<Vec<Name>, PyError> {
        
    }

    // block -> "\n" INDENT stmt* DEDENT
    fn block(&mut self) -> Result<Vec<Stmt>, PyError> {
        self.check_or_error(vec![TokenType::EndOfLine], "SyntaxError: missing newline before block".to_owned())?;
        self.check_or_error(vec![TokenType::Indent], "SyntaxError: missing indent before block".to_owned())?;
           
        let mut statements = Vec::new();
        while !self.check_advance(vec![TokenType::Dedent]) {
            // ignore end of lines that haven't been parsed in a rule in order to ignore blank lines
            if self.check_advance(vec![TokenType::EndOfLine]) {
                continue;
            }
            match self.statement() {
                Ok(s) => statements.push(s),
                Err(e) => {
                    return Err(e);
                },
            }
        }
        Ok(statements)
    }

    // expr -> equality
    fn expression(&mut self) -> Result<Expr, PyError> {
        self.disjunction()
    }

    // disjunction -> conjunction ("and" conjunction)*
    fn disjunction(&mut self) -> Result<Expr, PyError> {
        let mut ex = self.conjunction()?;
        while self.check_advance(vec![TokenType::Or]) {
            // turn the token into a BiOp
            let tok = &self.tokens[self.current_idx - 1];
            let op = match tok.token_type {
                TokenType::Or => BiOp {
                    ty: BiOpType::Or,
                    line: tok.line,
                    column: tok.column,
                },
                _ => panic!("In disjunction(): op token_type was not Or, error probably in check_advance() or disjunction()"),
            };
            let right = self.conjunction()?;
            ex = Expr::Binary(Box::new(ex), op, Box::new(right));
        }
        Ok(ex)
    }

    // conjunction -> equality ("and" equality)*
    fn conjunction(&mut self) -> Result<Expr, PyError> {
        let mut ex = self.equality()?;
        while self.check_advance(vec![TokenType::And]) {
            // turn the token into a BiOp
            let tok = &self.tokens[self.current_idx - 1];
            let op = match tok.token_type {
                TokenType::And => BiOp {
                    ty: BiOpType::And,
                    line: tok.line,
                    column: tok.column,
                },
                _ => panic!("In conjunction(): op token_type was not And, error probably in check_advance() or conjunction()"),
            };
            let right = self.equality()?;
            ex = Expr::Binary(Box::new(ex), op, Box::new(right));
        }
        Ok(ex)
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

    // primary -> NUMBER | STRING | "True" | "False" | "None" | "(" expr ")" | IDENTIFIER ("(" arguments? ")")?
    fn primary(&mut self) -> Result<Expr, PyError> {
        if self.check_advance(vec![TokenType::Identifier("".to_owned())]) {
            if self.check_advance(vec![TokenType::LeftParen]) {
                // save the id token to later get the line and column from it
                let id_tok = self.tokens[self.current_idx - 2].clone();
                let args = self.arguments()?;
                if let TokenType::Identifier(n) = id_tok.token_type {
                    return Ok(Expr::Call(Name { name: n , line: id_tok.line, column: id_tok.column }, args));
                } else {
                    panic!("expected Identifier token here");
                }
            } else {
                // go back if only id without parentheses found
                self.current_idx -= 1;
            }
        }
        
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

        // TODO: really necessary??
        if self.check_advance(vec![TokenType::Indent, TokenType::Dedent]) {
            return Err(PyError {
                msg: "IndentationError: unexpected indent/dedent".to_owned(),
                line: self.tokens[self.current_idx].line,
                column: self.tokens[self.current_idx].column,
            });
        }
        Err(PyError {
            msg: "SyntaxError: Unexpected or missing token".to_owned(),
            line: self.tokens[self.current_idx].line,
            column: self.tokens[self.current_idx].column, // TODO: sometimes points at wrong column
        })
    }

    // arguments -> expr ("," expr)*
    fn arguments(&mut self) -> Result<Vec<Expr>, PyError> {
        let mut args = Vec::new();
        while !self.check_advance(vec![TokenType::RightParen]) {
            args.push(self.expression()?);
            if self.check_advance(vec![TokenType::Comma]) {
                // NOTE: this allows e.g. f(1,), but python allows it too so no matter
                continue;
            }
        }
        Ok(args)
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
