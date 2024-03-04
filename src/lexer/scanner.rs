use std::{collections::HashMap, num::ParseFloatError};

use super::token::{LiterialValue, Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(_source: String) -> Self {
        Self {
            source: _source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,

            keywords: Self::get_keyword_hashmap(),
        }
    }

    pub fn get_keyword_hashmap() -> HashMap<&'static str, TokenType> {
        HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::CLass),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ])
    }

    // brief: scan tokens from  self.source,
    // input:
    // output: Convert self.source into Vec<String>
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut scan_errors = vec![];

        // Scan the source String.
        while !self.is_at_end() {
            self.start = self.current;

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
            lexeme: "EOF".to_string(),
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

        // Notice: After advance(), self.current points to the new character.
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
                    self.add_token(TokenType::Bang);
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
            '=' => {
                if self.second_operator_match('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
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

            ' ' | '\r' | '\t' => {}

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
    // Notice: After advance(), self.current points to the new character.
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

    // brief: Do sth when find an string.
    // input:
    // output: Err or Ok
    fn find_a_string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            return Err(String::from("Unterminated String Error!"));
        }

        self.advance(); // consume the second ".

        let value = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token_with_literial(TokenType::String, Some(LiterialValue::StringValue(value)));
        Ok(())
    }

    // brief: Do sth when find an Number, and check whether legal.
    // input:
    // output: Err or Ok

    fn find_a_number(&mut self) -> Result<(), String> {
        // if is_at_end return Err.
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

    // brief: Do sth when find an Identifier, or keyword.
    // input:
    // output: Ok
    fn find_an_identifier(&mut self) -> Result<(), String> {
        // if is_at_end return Err.
        while self.is_alpha_and_digit(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        if self.keywords.contains_key(text) {
            if let Some(ty) = self.keywords.get(text) {
                self.add_token(ty.clone());
            }
        } else {
            self.add_token(TokenType::Identifier);
        }
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

    // brief: Return current char.
    // input:
    // output:
    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    // brief: Return next char.
    // input:
    // output:
    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let sources = "(())".to_string();
        let mut scan = Scanner::new(sources);

        let res = scan.scan_tokens().unwrap();

        //dbg!(res);
    }

    #[test]
    fn handle_two_char_tokens() {
        let sources = "! != > >= < <= == =".to_string();
        let mut scan = Scanner::new(sources);

        let res = scan.scan_tokens();

        match res {
            Ok(_) => {
                //dbg!(res);
            }
            Err(st) => {
                println!("print is -> {}", st);
            }
        }
    }

    #[test]
    fn handle_equal_char_tokens() {
        let sources = " qweqew \"123\" //aaa".to_string();
        let mut scan = Scanner::new(sources);

        let res = scan.scan_tokens();

        match res {
            Ok(_) => {
                //dbg!(res);
            }
            Err(st) => {
                println!("print is -> {}", st);
            }
        }
    }

    #[test]
    fn handle_equal_string_tokens() {
        let sources = ">= \"1234\" ==".to_string();
        let mut scan = Scanner::new(sources);

        let res = scan.scan_tokens();

        match res {
            Ok(_) => {
                //dbg!(res);
            }
            Err(st) => {
                println!("print is -> {}", st);
            }
        }
    }

    #[test]
    fn handle_equal_iden_tokens() {
        let sources = ">= _abc = 1.123 ==".to_string();
        let mut scan = Scanner::new(sources);

        let res = scan.scan_tokens();

        match res {
            Ok(_) => {
                //dbg!(res);
            }
            Err(st) => {
                println!("print is -> {}", st);
            }
        }
    }

    #[test]
    fn handle_equal_keyword_tokens() {
        let sources = ">= var nil iff if".to_string();
        let mut scan = Scanner::new(sources);

        let res = scan.scan_tokens();

        match res {
            Ok(_) => {
                //dbg!(res);
            }
            Err(st) => {
                println!("print is -> {}", st);
            }
        }
    }

    #[test]
    fn handle_real_sentence() {
        let sources = "var num = 10.0 \n while true { num >= 1.0 }".to_string();
        let mut scan = Scanner::new(sources);

        let res = scan.scan_tokens().unwrap();

        assert_eq!(res.len(), 12);
        assert_eq!(res[0].token_type, TokenType::Var);
        assert_eq!(res[1].token_type, TokenType::Identifier);
        assert_eq!(res[2].token_type, TokenType::Equal);
        assert_eq!(res[3].token_type, TokenType::Number);
        assert_eq!(res[4].token_type, TokenType::While);
        assert_eq!(res[5].token_type, TokenType::True);
        assert_eq!(res[6].token_type, TokenType::LeftBrace);
        assert_eq!(res[7].token_type, TokenType::Identifier);
        assert_eq!(res[8].token_type, TokenType::GreaterEqual);
        assert_eq!(res[9].token_type, TokenType::Number);
        assert_eq!(res[10].token_type, TokenType::RightBrace);
        assert_eq!(res[11].token_type, TokenType::Eof);
    }

    #[test]
    fn handle_two_sentence() {
        let sources = "var num = 10.0; \n while true { num >= 1.0 }".to_string();
        let mut scan = Scanner::new(sources);

        let res = scan.scan_tokens();
        match res {
            Ok(v) => {
                dbg!(v);
            }
            Err(st) => {
                println!("print is -> {}", st);
            }
        }
    }
    // cargo test <unique signature: keyword> --  --nocapture

    // #[test]
    // fn handle_parser_sentence() {
    //     let sources = "1.0 * 3.0 + 2.0 * 4.0 == 11.0".to_string();
    //     let mut scan = Scanner::new(sources);

    //     let res = scan.scan_tokens().unwrap();

    //     dbg!(res);
    // }
}
