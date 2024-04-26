use crate::token::*;

fn scan(code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lex_start = 0;
    let mut current_idx = 0;
    let mut line = 1;

    while current_idx < code.len() {
        // println!("{current_idx}");
        // println!("{tokens:?}");
        lex_start = current_idx;
        match scan_token(&code, &mut lex_start, &mut current_idx, &mut line) {
            Ok(x) => match x {
                Some(t) => {
                    current_idx += t.value.len();
                    tokens.push(t)
                }
                None => {
                    current_idx += 1;
                    continue;
                }
            },
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
        '+' => Ok(Some(Token {
            token_type: TokenType::Plus,
            value: "+".to_string(),
            line: *line,
        })),
        '-' => Ok(Some(Token {
            token_type: TokenType::Minus,
            value: "-".to_string(),
            line: *line,
        })),
        '*' => Ok(Some(Token {
            token_type: TokenType::Asterisk,
            value: "*".to_string(),
            line: *line,
        })),
        '/' => Ok(Some(Token {
            token_type: TokenType::Slash,
            value: "/".to_string(),
            line: *line,
        })),
        '\n' => {
            *line += 1;
            Ok(Some(Token {
                token_type: TokenType::EndOfLine,
                value: "\n".to_string(),
                line: *line - 1,
            }))
        }
        ' ' | '\r' => Ok(None),
        _ => Err("Unknown token".to_string()),
    }
}
