use std::cmp::Ordering;

use crate::common::{py_error::*, token::*};

pub fn scan(code: String) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    // error flag to decide whether to return Some or None
    let mut error = false;
    let mut indent_stack = vec![0];
    let mut current_idx = 0;
    let mut line = 1;
    let mut column = 1;

    while current_idx < code.chars().count() {
        // calc the indent at every line start
        if column == 1 {
            match scan_indent(
                &code,
                &mut tokens,
                &mut indent_stack,
                &mut current_idx,
                line,
                &mut column,
            ) {
                Ok(_) => (),
                // indentation error
                Err(e) => {
                    error = true;
                    println!("{e}");
                    break; // after indentation error, scanner should stop, otherwise every new correct dedent is an error since the stack is gone
                }
            }
        }
        match scan_token(&code, &mut current_idx, line, &mut column) {
            Ok(x) => match x {
                // add token
                Some(t) => {
                    // special case because idx and col get updated inside function (to circumvent weird trimming of trailing zeros by rust)
                    if let TokenType::Float(_) = t.token_type {
                    } else {
                        // move the index by the lexeme length
                        current_idx += t.value.chars().count();
                        // increase column counter by same amount
                        column += t.value.chars().count() as u64;
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
            Err(e) => {
                error = true;
                current_idx += 1;
                column += 1;
                println!("{e}");
            }
        }
    }

    // push Dedent on stack for every Indent above 0
    while let Some(i) = indent_stack.pop() {
        if i > 0 {
            tokens.push(Token::create(TokenType::Dedent, line, column));
        }
    }
    tokens.push(Token::create(TokenType::EndOfFile, line, column));
    if !error {
        Some(tokens)
    } else {
        None
    }
}

fn scan_indent(
    code: &str,
    tokens: &mut Vec<Token>,
    indent_stack: &mut Vec<u64>,
    current_idx: &mut usize,
    line: u64,
    column: &mut u64,
) -> Result<(), PyError> {
    // calc the current line's indentation by counting whitespaces and tabs until first non-blank char shows up
    let mut line_indent = 0;
    for c in code.chars().skip(*current_idx) {
        match c {
            '\t' => {
                let next_8 = 8 - (line_indent % 8);
                line_indent += next_8;
                *current_idx += 1;
                *column += next_8;
            }
            ' ' => {
                line_indent += 1;
                *current_idx += 1;
                *column += 1;
            }
            '#' | '\n' => return Ok(()), // ignore indent on comment and blank lines
            _ => break,
        }
    }

    // check for matching indent in indent stack
    let mut current_indent = indent_stack.pop().expect(
        "This should never fail, because there should always be a number on this stack atp.",
    );
    match line_indent.cmp(&current_indent) {
        // keep looking for equal indent, if not found throw IndentationError
        Ordering::Less => loop {
            if line_indent == current_indent {
                indent_stack.push(current_indent);
                break;
            }
            if let Some(i) = indent_stack.pop() {
                tokens.push(Token::create(TokenType::Dedent, line, *column));
                current_indent = i;
            } else {
                // if there was no matching indent found, return an error
                return Err(PyError {
                    msg: "IndentationError: inconsistent dedent".to_string(),
                    line,
                    column: *column,
                });
            }
        },
        // do nothing
        Ordering::Equal => {
            indent_stack.push(current_indent);
        }
        // push line indent on stack and generate Indent
        Ordering::Greater => {
            indent_stack.push(current_indent);
            indent_stack.push(line_indent);
            tokens.push(Token::create(TokenType::Indent, line, *column));
        }
    }

    Ok(())
}

fn scan_token(
    code: &str,
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
        '[' => Ok(Some(Token::create(TokenType::LeftBracket, line, *column))),
        ']' => Ok(Some(Token::create(TokenType::RightBracket, line, *column))),
        ',' => Ok(Some(Token::create(TokenType::Comma, line, *column))),
        '\n' => Ok(Some(Token::create(TokenType::EndOfLine, line, *column))),

        // double character
        '!' => match code.next() {
            Some('=') => Ok(Some(Token::create(TokenType::NotEqual, line, *column))),
            _ => Err(PyError {
                msg: format!("SyntaxError: Unknown Token: \"{current_char}\""),
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
        '"' => build_string(code, current_idx, line, column),
        '0'..='9' => build_number(code, current_char, current_idx, line, column),
        '_' | 'a'..='z' | 'A'..='Z' => Ok(Some(build_identifier(code, current_char, line, column))),

        // ignored
        '\r' | '\t' | ' ' => Ok(None),
        // comments
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
    mut code: impl Iterator<Item = char>,
    current_idx: &mut usize,
    line: u64,
    column: &mut u64,
) -> Result<Option<Token>, PyError> {
    // used for still moving the index in case of error
    let mut err_idx = *current_idx;
    let mut text = String::new();
    while let Some(c) = code.next() {
        err_idx += 1;
        match c {
            '\\' => {
                *current_idx += 1;
                err_idx += 1;
                match code.next() {
                    Some(c) => match c {
                        '"' | '\\' => text.push(c),
                        'n' => text.push('\n'),
                        _ => {
                            *current_idx += 1;
                            continue;
                        }
                    },
                    None => {
                        *current_idx = err_idx - 1;
                        return Err(PyError {
                            msg: format!("SyntaxError: Unterminated String: \"{text}"),
                            line,
                            column: *column,
                        });
                    }
                }
            }
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

fn build_number(
    mut code: impl Iterator<Item = char>,
    current_char: char,
    current_idx: &mut usize,
    line: u64,
    column: &mut u64,
) -> Result<Option<Token>, PyError> {
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
            ' ' | '\n' | '+' | '-' | '*' | '/' | ':' | '<' | '>' | '=' | '!' | '(' | ')' | '['
            | ']' | '{' | '}' | ',' => break,
            '0'..='9' => number.push(c),
            '.' => {
                // was there already a floating point?
                if is_float {
                    *current_idx = err_idx - 1;
                    return Err(PyError {
                        msg: format!("SyntaxError: Float has more than one point: {number}{c}"),
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
                            number.push(char_after_dot.expect("This should never fail, because char_after_dot can only be a number here"));
                        }
                        _ => {
                            *current_idx = err_idx;
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

    // parse number into f64 or u64 depending on is_float
    if is_float {
        // update the idx here because Rust trims trailing zeros, e.g. 2.0 becomes 2
        *current_idx += number.len();
        *column += number.len() as u64;
        Ok(Some(Token::create(
            TokenType::Float(
                number
                    .parse::<f64>()
                    .expect("This should never fail, because number should only contain numbers"),
            ),
            line,
            // still use old column for token start
            *column - number.len() as u64,
        )))
    } else {
        Ok(Some(Token::create(
            TokenType::Int(
                number
                    .parse::<u64>()
                    .expect("This should never fail, because number should only contain numbers"), // TODO: what if number big
            ),
            line,
            *column,
        )))
    }
}

fn build_identifier(
    code: impl Iterator<Item = char>,
    current_char: char,
    line: u64,
    column: &mut u64,
) -> Token {
    let mut text = current_char.to_string();
    for c in code {
        if c.is_alphanumeric() || c == '_' {
            text.push(c)
        } else {
            break;
        }
    }
    let token_type = check_keywords(&text).unwrap_or(TokenType::Identifier(text));
    Token::create(token_type, line, *column)
}

fn check_keywords(text: &str) -> Option<TokenType> {
    match text {
        "True" => Some(TokenType::True),
        "False" => Some(TokenType::False),
        "not" => Some(TokenType::Not),
        "and" => Some(TokenType::And),
        "or" => Some(TokenType::Or),
        "if" => Some(TokenType::If),
        "else" => Some(TokenType::Else),
        "while" => Some(TokenType::While),
        "def" => Some(TokenType::Def),
        "return" => Some(TokenType::Return),
        "print" => Some(TokenType::Print),
        "None" => Some(TokenType::None),
        _ => None,
    }
}
