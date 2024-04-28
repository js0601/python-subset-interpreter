#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: u64,
}

impl Token {
    // exists so scanner doesn't have to do it all
    pub fn create(token_type: TokenType, line: u64) -> Self {
        match token_type {
            TokenType::Plus => Self {
                token_type,
                value: "+".to_string(),
                line,
            },
            TokenType::Minus => Self {
                token_type,
                value: "-".to_string(),
                line,
            },
            TokenType::Asterisk => Self {
                token_type,
                value: "*".to_string(),
                line,
            },
            TokenType::Slash => Self {
                token_type,
                value: "/".to_string(),
                line,
            },
            TokenType::Colon => Self {
                token_type,
                value: ":".to_string(),
                line,
            },
            TokenType::LeftParen => Self {
                token_type,
                value: "(".to_string(),
                line,
            },
            TokenType::RightParen => Self {
                token_type,
                value: ")".to_string(),
                line,
            },
            TokenType::Hashtag => Self {
                token_type,
                value: "#".to_string(),
                line,
            },
            TokenType::NotEqual => Self {
                token_type,
                value: "!=".to_string(),
                line,
            },
            TokenType::Equal => Self {
                token_type,
                value: "=".to_string(),
                line,
            },
            TokenType::DoubleEqual => Self {
                token_type,
                value: "==".to_string(),
                line,
            },
            TokenType::Greater => Self {
                token_type,
                value: ">".to_string(),
                line,
            },
            TokenType::GreaterEqual => Self {
                token_type,
                value: ">=".to_string(),
                line,
            },
            TokenType::Less => Self {
                token_type,
                value: "<".to_string(),
                line,
            },
            TokenType::LessEqual => Self {
                token_type,
                value: "<=".to_string(),
                line,
            },
            TokenType::True => Self {
                token_type,
                value: "True".to_string(),
                line,
            },
            TokenType::False => Self {
                token_type,
                value: "False".to_string(),
                line,
            },
            TokenType::Not => Self {
                token_type,
                value: "not".to_string(),
                line,
            },
            TokenType::And => Self {
                token_type,
                value: "and".to_string(),
                line,
            },
            TokenType::Or => Self {
                token_type,
                value: "or".to_string(),
                line,
            },
            TokenType::If => Self {
                token_type,
                value: "if".to_string(),
                line,
            },
            TokenType::Else => Self {
                token_type,
                value: "else".to_string(),
                line,
            },
            TokenType::While => Self {
                token_type,
                value: "while".to_string(),
                line,
            },
            TokenType::Def => Self {
                token_type,
                value: "def".to_string(),
                line,
            },
            TokenType::Return => Self {
                token_type,
                value: "return".to_string(),
                line,
            },
            TokenType::None => Self {
                token_type,
                value: "None".to_string(),
                line,
            },
            TokenType::Identifier(ref n) => Self {
                value: n.to_string(),
                token_type,
                line,
            },
            TokenType::String(ref s) => Self {
                value: s.to_string(),
                token_type,
                line,
            },
            TokenType::Int(x) => Self {
                token_type,
                value: x.to_string(),
                line,
            },
            TokenType::Float(x) => Self {
                token_type,
                value: x.to_string(),
                line,
            },
            TokenType::EndOfLine => Self {
                token_type,
                value: "\n".to_string(),
                line,
            },
            TokenType::EndOfFile => Self {
                token_type,
                value: "".to_string(),
                line,
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
