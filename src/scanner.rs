use crate::token::*;

pub fn scan(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    // TODO: does lex_start really need to exist?
    let mut lex_start = 0;
    let mut current_idx = 0;
    let mut line = 1;

    while current_idx < code.len() {
        lex_start = current_idx;
        match scan_token(&code, &mut lex_start, &mut current_idx, &mut line) {
            Ok(x) => match x {
                // add token
                Some(t) => {
                    current_idx += t.value.len();
                    tokens.push(t)
                }
                // ignore
                None => {
                    current_idx += 1;
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

    tokens.push(Token {
        token_type: TokenType::EndOfFile,
        value: "".to_string(),
        line,
    });
    tokens
}

// TODO: add all the tokens
fn scan_token(
    code: &str,
    lex_start: &mut usize,
    current_idx: &mut usize,
    line: &mut u64,
) -> Result<Option<Token>, String> {
    let current_char = code
        .chars()
        .nth(*current_idx)
        .expect("This should not fail, since current_idx should not be out of bounds here");
    match current_char {
        '+' => Ok(Some(Token::create(TokenType::Plus, *line))),
        '-' => Ok(Some(Token::create(TokenType::Minus, *line))),
        '*' => Ok(Some(Token::create(TokenType::Asterisk, *line))),
        '/' => Ok(Some(Token::create(TokenType::Slash, *line))),
        '\n' => {
            *line += 1;
            Ok(Some(Token::create(TokenType::EndOfLine, *line)))
        }
        // TODO: probably don't ignore all whitespace because of identation
        // TODO: check for all ignored chars if they work
        ' ' | '\r' => Ok(None),
        // TODO: instead of a String maybe return a custom PyError
        _ => Err("Unknown token".to_string()),
    }
}
