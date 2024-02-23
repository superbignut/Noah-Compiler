use super::{
    expr::Expr,
    token::{Token, TokenType},
};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    // brief:
    // input:
    // output:
    fn new(&self, tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /*
    expression -> equality
    equality -> comparision ( ("!=" | "==") comparision  ) * ;
    comparision -> term ( ( ">" | ">=" | "<" | "<=") ) * ;
    term -> factor ( ( "-" | "+" ) factor ) * ;
    factor -> unary ( ( "/" | "*") unary ) * ;
    unary -> ( ( "!" | "-" ) unary ) | primary ;
    primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    */

    // brief: expression -> equality
    // input:
    // output:
    fn expression(&self) -> Expr {
        self.equality()
    }

    // brief:
    // input:
    // output:

    fn equality(&self) -> Expr {
        let expr = self.comparision();

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparision();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            };
        }

        expr
    }

    fn comparision(&self) -> Expr {
        todo!()
    }

    fn match_tokens(&self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        }
        self.peek()
    }
    // brief: Peek the next char.
    // input:
    // output:
    // Attention : Make sure self.current is available before call self.peek().
    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }
}
