use super::{
    expr::{Expr, ExprLiteral},
    token::{LiterialValue, Token, TokenType},
};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    // brief:
    // input:
    // output:
    fn new(tokens: Vec<Token>) -> Self {
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
    fn expression(&mut self) -> Result<Expr, String> {
        let parser_error: Vec<String> = vec![]; // Todo: Error Add together.
        self.equality()
    }

    // brief: equality -> comparision ( ("!=" | "==") comparision  ) * ;
    // 1 != 2 != 3 != 4
    // input:
    // output:
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparision()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right_expr = self.comparision()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right_expr),
            };
        }

        Ok(expr)
    }

    //comparision -> term ( ( ">" | ">=" | "<" | "<=") ) * ;
    fn comparision(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right_expr = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right_expr),
            };
        }
        Ok(expr)
    }

    // term -> factor ( ( "-" | "+" ) factor ) * ;
    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right_expr = self.factor()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right_expr),
            };
        }
        Ok(expr)
    }

    // factor -> unary ( ( "/" | "*") unary ) * ;
    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right_expr = self.unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right_expr),
            };
        }
        Ok(expr)
    }

    // unary -> ( ( "!" | "-" ) unary ) | primary ;
    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right_expr = self.unary()?;

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right_expr),
            });
        }
        self.primary()
    }

    // primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[TokenType::False]) {
            Ok(Expr::Literal {
                value: ExprLiteral::False,
            })
        } else if self.match_tokens(&[TokenType::True]) {
            Ok(Expr::Literal {
                value: ExprLiteral::True,
            })
        } else if self.match_tokens(&[TokenType::Nil]) {
            Ok(Expr::Literal {
                value: ExprLiteral::Nil,
            })
        } else if self.match_tokens(&[TokenType::String]) {
            if let Some(LiterialValue::StringValue(v)) = self.previous().literial {
                return Ok(Expr::Literal {
                    value: ExprLiteral::StringLiteral(v),
                });
            }
            Err(String::from("Error occur at parsering String."))
        } else if self.match_tokens(&[TokenType::Number]) {
            if let Some(LiterialValue::FloatValue(v)) = self.previous().literial {
                return Ok(Expr::Literal {
                    value: ExprLiteral::NumberLiteral(v),
                });
            }
            Err(String::from("Error occur at parsering Number."))
        } else if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume();
            Ok(Expr::Grouping {
                expression: Box::new(expr),
            })
        } else {
            Err(String::from(
                "Parsering error occurs for finding nothing to match with.",
            ))
        }
    }

    // brief: Tihs is the second version of primary(),clear and short superficially，
    // which increase the degree of code coupling.So i still use if to match.
    // input:
    // output:
    // fn primary2(&mut self) -> Result<Expr, String> {
    //     if self.match_tokens(&[TokenType::LeftParen]) {
    //         let expr = self.expression();
    //         self.consume();
    //         Ok(Expr::Grouping {
    //             expression: Box::new(expr),
    //         })
    //     } else {
    //         let current_token = self.peek();
    //         self.advance();
    //         Ok(Expr::Literal {
    //             value: ExprLiteral::from_token(current_token)?,
    //         })
    //     }
    // }

    fn consume(&mut self) -> Token {
        todo!()
    }
    // brief: Check tempToken and self.current ++ if matched really.
    // input:
    // output:
    fn match_tokens(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    // brief: Peek the next char.
    // input:
    // output:
    // Attention : Make sure self.current is available before call self.peek().
    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    // brief:
    // input:
    // output:
    // Attention : if is_at_end() return will be the last one, and current do not increase.
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    // brief:
    // input:
    // output:
    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    // brief:
    // input:
    // output:
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Scanner;
    #[test]
    fn parser_test_one() {
        let sources = "1.0 * 3.0 * 2.0 + 2.0 * 4.0 == 11.0".to_string();
        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        let pas = Parser::new(tok).expression().unwrap().two_string();

        dbg!(pas);
    }
}
// cargo test <unique signature: keyword> --  --nocapture
// Todo: Add a Error Address.
