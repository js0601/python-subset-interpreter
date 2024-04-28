#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: u64,
    pub column: u64,
}

impl Token {
    // exists so scanner doesn't have to do it all
    pub fn create(token_type: TokenType, line: u64, column: u64) -> Self {
        match token_type {
            TokenType::Plus => Self {
                token_type,
                value: "+".to_string(),
                line,
                column,
            },
            TokenType::Minus => Self {
                token_type,
                value: "-".to_string(),
                line,
                column,
            },
            TokenType::Asterisk => Self {
                token_type,
                value: "*".to_string(),
                line,
                column,
            },
            TokenType::Slash => Self {
                token_type,
                value: "/".to_string(),
                line,
                column,
            },
            TokenType::Colon => Self {
                token_type,
                value: ":".to_string(),
                line,
                column,
            },
            TokenType::LeftParen => Self {
                token_type,
                value: "(".to_string(),
                line,
                column,
            },
            TokenType::RightParen => Self {
                token_type,
                value: ")".to_string(),
                line,
                column,
            },
            TokenType::Hashtag => Self {
                token_type,
                value: "#".to_string(),
                line,
                column,
            },
            TokenType::NotEqual => Self {
                token_type,
                value: "!=".to_string(),
                line,
                column,
            },
            TokenType::Equal => Self {
                token_type,
                value: "=".to_string(),
                line,
                column,
            },
            TokenType::DoubleEqual => Self {
                token_type,
                value: "==".to_string(),
                line,
                column,
            },
            TokenType::Greater => Self {
                token_type,
                value: ">".to_string(),
                line,
                column,
            },
            TokenType::GreaterEqual => Self {
                token_type,
                value: ">=".to_string(),
                line,
                column,
            },
            TokenType::Less => Self {
                token_type,
                value: "<".to_string(),
                line,
                column,
            },
            TokenType::LessEqual => Self {
                token_type,
                value: "<=".to_string(),
                line,
                column,
            },
            TokenType::True => Self {
                token_type,
                value: "True".to_string(),
                line,
                column,
            },
            TokenType::False => Self {
                token_type,
                value: "False".to_string(),
                line,
                column,
            },
            TokenType::Not => Self {
                token_type,
                value: "not".to_string(),
                line,
                column,
            },
            TokenType::And => Self {
                token_type,
                value: "and".to_string(),
                line,
                column,
            },
            TokenType::Or => Self {
                token_type,
                value: "or".to_string(),
                line,
                column,
            },
            TokenType::If => Self {
                token_type,
                value: "if".to_string(),
                line,
                column,
            },
            TokenType::Else => Self {
                token_type,
                value: "else".to_string(),
                line,
                column,
            },
            TokenType::While => Self {
                token_type,
                value: "while".to_string(),
                line,
                column,
            },
            TokenType::Def => Self {
                token_type,
                value: "def".to_string(),
                line,
                column,
            },
            TokenType::Return => Self {
                token_type,
                value: "return".to_string(),
                line,
                column,
            },
            TokenType::None => Self {
                token_type,
                value: "None".to_string(),
                line,
                column,
            },
            TokenType::Identifier(ref n) => Self {
                value: n.to_string(),
                token_type,
                line,
                column,
            },
            TokenType::String(ref s) => Self {
                value: s.to_string(),
                token_type,
                line,
                column,
            },
            TokenType::Int(x) => Self {
                token_type,
                value: x.to_string(),
                line,
                column,
            },
            TokenType::Float(x) => Self {
                token_type,
                value: x.to_string(),
                line,
                column,
            },
            TokenType::EndOfLine => Self {
                token_type,
                value: "\n".to_string(),
                line,
                column,
            },
            TokenType::EndOfFile => Self {
                token_type,
                value: "".to_string(),
                line,
                column,
            },
        }
    }
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
