use super::token::{LiterialValue, Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(_source: String) -> Self {
        Self {
            source: _source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }
    // brief: scan tokens from  self.source,
    // input:
    // output: Convert self.source into Vec<String>
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literial: None,
            line_number: self.line,
        });

        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // brief: match token, used by scan_tokens.
    // input:
    // output:
    fn scan_token(&mut self) -> Result<(), String> {
        let temp_char: char = self.advance();
        match temp_char {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            _ => {
                return Err(String::from("unexpected character!"));
            }
        };
        Ok(())
    }

    // brief:  consumes the next character in the source file and returns it.
    // input:
    // output: next char.
    fn advance(&mut self) -> char {
        let current_char = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        current_char
    }

    // brief: grabs the text of the current lexeme and creates a new token for it.
    // input:
    // output:
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literial(token_type, None);
    }

    // brief: add_token with literial.
    // input:
    // output:
    fn add_token_with_literial(&mut self, token_type: TokenType, literial: Option<LiterialValue>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            literial,
            line_number: self.line,
        });
    }
}
