#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literial: Option<LiterialValue>,
    pub line_number: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literial: Option<LiterialValue>,
        line_number: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literial,
            line_number,
        }
    }

    pub fn two_string(&self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literial)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiterialValue {
    IntValue(i64),
    FloatValue(f64),
    StringValue(String),
    IdentifierVal(String),
}

#[derive(Debug, Clone, PartialEq)] // Todo: Partialeq
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literials.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    CLass,
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Let,
    While,
    // Eof.
    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
