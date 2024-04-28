use crate::common::token::*;

pub fn scan(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    // TODO: does lex_start really need to exist?
    let mut lex_start = 0;
    let mut current_idx = 0;
    let mut line = 1;
    let mut column = 1;

    while current_idx < code.len() {
        lex_start = current_idx;
        match scan_token(&code, &mut lex_start, &mut current_idx, line, column) {
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
                    continue;
                }
            },
            // syntax error (unknown token)
            Err(e) => {
                println!("Error: {e}");
                break;
            }
        }
    }

    tokens.push(Token::create(TokenType::EndOfFile, line, column));
    tokens
}

// TODO: add all the tokens
fn scan_token(
    code: &str,
    lex_start: &mut usize,
    current_idx: &mut usize,
    line: u64,
    column: u64,
) -> Result<Option<Token>, String> {
    let current_char = code
        .chars()
        .nth(*current_idx)
        .expect("This should not fail, since current_idx should not be out of bounds here");
    match current_char {
        '+' => Ok(Some(Token::create(TokenType::Plus, line, column))),
        '-' => Ok(Some(Token::create(TokenType::Minus, line, column))),
        '*' => Ok(Some(Token::create(TokenType::Asterisk, line, column))),
        '/' => Ok(Some(Token::create(TokenType::Slash, line, column))),
        '\n' => Ok(Some(Token::create(TokenType::EndOfLine, line, column))),
        // TODO: probably don't ignore all whitespace because of identation
        // TODO: check for all ignored chars if they work
        ' ' | '\r' => Ok(None),
        // TODO: instead of a String maybe return a custom PyError
        _ => Err("Unknown token".to_string()),
    }
}
