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
                    // move the index by the lexeme length
                    current_idx += t.value.len();
                    // increase column counter by same amount
                    column += t.value.len() as u64;
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

        // ignored
        // TODO: probably don't ignore all whitespace because of identation
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
            msg: format!("Syntax Error: Unknown Token: \"{current_char}\""),
            line,
            column: *column,
        }),
    }
}
