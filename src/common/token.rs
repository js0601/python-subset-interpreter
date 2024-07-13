#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    // single-character
    Plus,
    Minus,
    Asterisk,
    Slash,
    Colon,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    // Point,
    // Percent,
    EndOfLine,

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
    Else,
    While,
    Def,
    Return,
    Print,
    None,

    // literals
    Identifier(String),
    String(String),
    Int(u64), // this is only ever positive, bc negative numbers are built by the parser
    Float(f64),

    Indent,
    Dedent,
    EndOfFile,
}

#[derive(Debug, Clone)]
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
                value: "+".to_owned(),
                line,
                column,
            },
            TokenType::Minus => Self {
                token_type,
                value: "-".to_owned(),
                line,
                column,
            },
            TokenType::Asterisk => Self {
                token_type,
                value: "*".to_owned(),
                line,
                column,
            },
            TokenType::Slash => Self {
                token_type,
                value: "/".to_owned(),
                line,
                column,
            },
            TokenType::Colon => Self {
                token_type,
                value: ":".to_owned(),
                line,
                column,
            },
            TokenType::LeftParen => Self {
                token_type,
                value: "(".to_owned(),
                line,
                column,
            },
            TokenType::RightParen => Self {
                token_type,
                value: ")".to_owned(),
                line,
                column,
            },
            TokenType::LeftBracket => Self {
                token_type,
                value: "[".to_owned(),
                line,
                column,
            },
            TokenType::RightBracket => Self {
                token_type,
                value: "]".to_owned(),
                line,
                column,
            },
            TokenType::Comma => Self {
                token_type,
                value: ",".to_owned(),
                line,
                column,
            },
            TokenType::NotEqual => Self {
                token_type,
                value: "!=".to_owned(),
                line,
                column,
            },
            TokenType::Equal => Self {
                token_type,
                value: "=".to_owned(),
                line,
                column,
            },
            TokenType::DoubleEqual => Self {
                token_type,
                value: "==".to_owned(),
                line,
                column,
            },
            TokenType::Greater => Self {
                token_type,
                value: ">".to_owned(),
                line,
                column,
            },
            TokenType::GreaterEqual => Self {
                token_type,
                value: ">=".to_owned(),
                line,
                column,
            },
            TokenType::Less => Self {
                token_type,
                value: "<".to_owned(),
                line,
                column,
            },
            TokenType::LessEqual => Self {
                token_type,
                value: "<=".to_owned(),
                line,
                column,
            },
            TokenType::True => Self {
                token_type,
                value: "True".to_owned(),
                line,
                column,
            },
            TokenType::False => Self {
                token_type,
                value: "False".to_owned(),
                line,
                column,
            },
            TokenType::Not => Self {
                token_type,
                value: "not".to_owned(),
                line,
                column,
            },
            TokenType::And => Self {
                token_type,
                value: "and".to_owned(),
                line,
                column,
            },
            TokenType::Or => Self {
                token_type,
                value: "or".to_owned(),
                line,
                column,
            },
            TokenType::If => Self {
                token_type,
                value: "if".to_owned(),
                line,
                column,
            },
            TokenType::Else => Self {
                token_type,
                value: "else".to_owned(),
                line,
                column,
            },
            TokenType::While => Self {
                token_type,
                value: "while".to_owned(),
                line,
                column,
            },
            TokenType::Def => Self {
                token_type,
                value: "def".to_owned(),
                line,
                column,
            },
            TokenType::Return => Self {
                token_type,
                value: "return".to_owned(),
                line,
                column,
            },
            TokenType::Print => Self {
                token_type,
                value: "print".to_owned(),
                line,
                column,
            },
            TokenType::None => Self {
                token_type,
                value: "None".to_owned(),
                line,
                column,
            },
            TokenType::Identifier(ref n) => Self {
                value: n.to_owned(),
                token_type,
                line,
                column,
            },
            TokenType::String(ref s) => Self {
                value: format!("\"{s}\""),
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
                value: "\n".to_owned(),
                line,
                column,
            },
            TokenType::Indent => Self {
                token_type,
                value: "".to_owned(),
                line,
                column,
            },
            TokenType::Dedent => Self {
                token_type,
                value: "".to_owned(),
                line,
                column,
            },
            TokenType::EndOfFile => Self {
                token_type,
                value: "".to_owned(),
                line,
                column,
            },
        }
    }
}
