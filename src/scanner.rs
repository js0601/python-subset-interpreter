use crate::common::{py_error::*, token::*};

pub fn scan(code: String) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    // error flag to decide whether to return Some or None
    let mut error = false;
    // TODO: does lex_start really need to exist?
    let mut lex_start = 0;
    let mut current_idx = 0;
    let mut line = 1;
    let mut column = 1;

    while current_idx < code.len() {
        lex_start = current_idx;
        match scan_token(&code, &mut lex_start, &mut current_idx, line, &mut column) {
            Ok(x) => match x {
                // add token
                Some(t) => {
                    // special case because idx and col get updated inside function (to circumvent weird trimming of trailing zeros by rust)
                    if let TokenType::Float(_) = t.token_type {
                    } else {
                        // move the index by the lexeme length
                        current_idx += t.value.len();
                        // increase column counter by same amount
                        column += t.value.len() as u64;
                    }
                    // if it's a EoL token increase the line counter and reset column counter
                    if let TokenType::EndOfLine = t.token_type {
                        line += 1;
                        column = 1;
                    }
                    tokens.push(t)
                }
                // ignore
                None => {
                    // ignored characters are always of length 1
                    current_idx += 1;
                    column += 1;
                }
            },
            // syntax error (unknown token)
            // TODO: handle error better by returning it from scan and handling it in main
            Err(e) => {
                error = true;
                current_idx += 1;
                column += 1;
                println!("{e}");
            }
        }
    }

    tokens.push(Token::create(TokenType::EndOfFile, line, column));
    if !error {
        Some(tokens)
    } else {
        None
    }
}

// TODO: add all the tokens
fn scan_token(
    code: &str,
    lex_start: &mut usize,
    current_idx: &mut usize,
    line: u64,
    column: &mut u64,
) -> Result<Option<Token>, PyError> {
    let mut code = code.chars();
    let current_char = code
        .nth(*current_idx)
        .expect("This should not fail, since current_idx should not be out of bounds here");
    match current_char {
        // single character
        '+' => Ok(Some(Token::create(TokenType::Plus, line, *column))),
        '-' => Ok(Some(Token::create(TokenType::Minus, line, *column))),
        '*' => Ok(Some(Token::create(TokenType::Asterisk, line, *column))),
        '/' => Ok(Some(Token::create(TokenType::Slash, line, *column))),
        ':' => Ok(Some(Token::create(TokenType::Colon, line, *column))),
        '(' => Ok(Some(Token::create(TokenType::LeftParen, line, *column))),
        ')' => Ok(Some(Token::create(TokenType::RightParen, line, *column))),
        '\n' => Ok(Some(Token::create(TokenType::EndOfLine, line, *column))),

        // double character
        '!' => match code.next() {
            Some('=') => Ok(Some(Token::create(TokenType::NotEqual, line, *column))),
            _ => Err(PyError {
                msg: format!("Syntax Error: Unknown Token: \"{current_char}\""),
                line,
                column: *column,
            }),
        },

        // single or double character
        '=' => match code.next() {
            Some('=') => Ok(Some(Token::create(TokenType::DoubleEqual, line, *column))),
            _ => Ok(Some(Token::create(TokenType::Equal, line, *column))),
        },
        '>' => match code.next() {
            Some('=') => Ok(Some(Token::create(TokenType::GreaterEqual, line, *column))),
            _ => Ok(Some(Token::create(TokenType::Greater, line, *column))),
        },
        '<' => match code.next() {
            Some('=') => Ok(Some(Token::create(TokenType::LessEqual, line, *column))),
            _ => Ok(Some(Token::create(TokenType::Less, line, *column))),
        },

        // literals
        // TODO: extract these into seperate functions for readability
        // Strings
        '"' => {
            // used for still moving the index in case of error
            let mut err_idx = *current_idx;
            let mut text = String::new();
            for c in code {
                err_idx += 1;
                match c {
                    // end string
                    '"' => break,
                    // missing " at end
                    '\n' => {
                        // -1 so it points at the newline
                        *current_idx = err_idx - 1;
                        return Err(PyError {
                            msg: format!("SyntaxError: Unterminated String: \"{text}"),
                            line,
                            column: *column,
                        });
                    }
                    _ => text.push(c),
                }
            }
            Ok(Some(Token::create(TokenType::String(text), line, *column)))
        }
        // Numbers
        '0'..='9' => {
            // used for still moving the idx in case of error
            let mut err_idx = *current_idx;
            // used so error points at the invalid literal
            let mut err_col = *column;
            // used so there can only be one . in the number
            let mut is_float = false;
            let mut number = current_char.to_string();
            while let Some(c) = code.next() {
                err_idx += 1;
                err_col += 1;
                match c {
                    ' ' | '\n' | '+' | '-' | '*' | '/' => break,
                    '0'..='9' => number.push(c),
                    '.' => {
                        if is_float {
                            *current_idx = err_idx - 1;
                            return Err(PyError {
                                msg: format!(
                                    "SyntaxError: Float has more than one point: {number}{c}"
                                ),
                                line,
                                column: err_col,
                            });
                        } else {
                            // see if there is actually a number after the floating point
                            let char_after_dot = code.next();
                            match char_after_dot {
                                Some('0'..='9') => {
                                    is_float = true;
                                    number.push('.');
                                    // TODO: don't just unwrap here
                                    number.push(char_after_dot.unwrap());
                                }
                                _ => {
                                    *current_idx = err_idx;
                                    // TODO: error msg could show what follows instead
                                    return Err(PyError {
                                        msg: "SyntaxError: Floating Point not followed by number"
                                            .to_string(),
                                        line,
                                        column: err_col,
                                    });
                                }
                            }
                        }
                    }
                    // not a valid number
                    _ => {
                        *current_idx = err_idx - 1;
                        return Err(PyError {
                            msg: format!("SyntaxError: Invalid Decimal Literal: {c}"),
                            line,
                            column: err_col,
                        });
                    }
                }
            }

            if is_float {
                *current_idx += number.len();
                *column += number.len() as u64;
                Ok(Some(Token::create(
                    TokenType::Float(number.parse::<f64>().expect(
                        "This should never fail, because number should only contain numbers",
                    )),
                    line,
                    // still use old column for token start
                    *column - number.len() as u64,
                )))
            } else {
                Ok(Some(Token::create(
                    TokenType::Int(number.parse::<u64>().expect(
                        "This should never fail, because number should only contain numbers",
                    )),
                    line,
                    *column,
                )))
            }
        }

        // ignored
        // TODO: probably don't ignore all whitespace because of identation
        // TODO: \t not to be ignored (probably need a "block" token)
        ' ' => Ok(None),
        '\r' => Ok(None),
        '#' => {
            while code
                .next()
                .expect("There should always be a newline between the start of a comment and the end of the iterator/file")
                != '\n'
            {
                *current_idx += 1;
                *column += 1;
            }
            Ok(None)
        }

        // unknown
        _ => Err(PyError {
            msg: format!("SyntaxError: Unknown Token: {current_char}"),
            line,
            column: *column,
        }),
    }
}

fn build_string(
    code: impl Iterator<Item = char>,
    current_idx: &mut usize,
    line: u64,
    column: &mut u64,
) {
}

fn build_number(
    code: impl Iterator<Item = char>,
    current_idx: &mut usize,
    line: u64,
    column: &mut u64,
) {
}
