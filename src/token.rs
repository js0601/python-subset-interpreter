#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: u64,
}

// TODO: implement commented types
#[derive(Debug)]
pub enum TokenType {
    // single-character
    Plus,
    Minus,
    Asterisk,
    Slash,
    Colon,
    LeftParen,
    RightParen,
    Hashtag,
    // LeftBracket,
    // RightBracket,
    // Comma,
    // Point,
    // Percent,

    // double-character
    NotEqual,

    // single- or double-character
    Equal,
    DoubleEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // keywords
    True,
    False,
    Not,
    And,
    Or,
    If,
    // Elif,
    Else,
    While,
    // For,
    Def,
    Return,
    None,

    // literals
    Identifier(String),
    String(String),
    Int(i64),
    Float(f64),

    EndOfLine,
    EndOfFile,
}
