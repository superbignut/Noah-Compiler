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

    fn scan_token(&mut self) -> Result<Vec<Token>, String> {
        let temp_char: char = self.advance();
        match temp_char {
            '(' => self.add_token(TokenType::LeftParen),
            _ => {}
        };
        todo!()
    }
    fn advance(&mut self) -> char {
        let current_char = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        current_char
    }

    fn add_token(&mut self, token_type: TokenType) {
        todo!()
    }
    fn add_token_with_literial(&mut self, token_type: TokenType, literial: Option<LiterialValue>) {
        todo!()
    }
}
