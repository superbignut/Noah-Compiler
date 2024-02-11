use std::num::ParseFloatError;

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
        let mut scan_errors = vec![];

        // Scan the source String.
        while !self.is_at_end() {
            self.start = self.current; // ?????

            match self.scan_token() {
                Ok(_) => {}
                Err(msg) => {
                    scan_errors.push(msg);
                }
            }
        }

        // Add an EOF.
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literial: None,
            line_number: self.line,
        });

        // Return all thErrors.
        if !scan_errors.is_empty() {
            let mut joined = "".to_string();

            for msg in scan_errors {
                joined.push_str(&msg);
            }
            joined.push('\n');
            Err(joined)
        } else {
            Ok(self.tokens.clone())
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // brief: match token and add token to self, used by scan_tokens.
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
            '!' => {
                if self.second_operator_match('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '>' => {
                if self.second_operator_match('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '<' => {
                if self.second_operator_match('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '/' => {
                if self.second_operator_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.find_a_string()?;
            }

            _ => {
                if self.is_digit(temp_char) {
                    self.find_a_number()?;
                } else if self.is_alpha(temp_char) {
                    self.find_an_identifier()?;
                } else {
                    return Err(format!("Unexpected character at line: {}", self.line));
                }
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
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literial,
            line_number: self.line,
        });
    }

    // brief: wether the second character matched.(conditional advanced.)
    // input:
    // output:
    fn second_operator_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap_or('\0') != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn find_a_string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            return Err(String::from("Unterminated String Error!"));
        }

        self.advance();

        let value = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token_with_literial(TokenType::String, Some(LiterialValue::StringValue(value)));
        Ok(())
    }

    fn find_a_number(&mut self) -> Result<(), String> {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // consume '.'
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        } else {
            return Err(String::from("digit Error!"));
        }
        let value: Result<f64, ParseFloatError> = self.source[self.start..self.current].parse();
        match value {
            Ok(v) => {
                self.add_token_with_literial(TokenType::Number, Some(LiterialValue::FloatValue(v)));
                Ok(())
            }

            Err(_) => Err(String::from("Parse Error!")),
        }
    }

    fn find_an_identifier(&mut self) -> Result<(), String> {
        while self.is_alpha_and_digit(self.peek()) {
            self.advance();
        }
        self.add_token(TokenType::Identifier);
        Ok(())
    }

    fn is_alpha_and_digit(&self, c: char) -> bool {
        self.is_digit(c) || self.is_alpha(c)
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }
}
