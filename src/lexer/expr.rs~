use super::token::{Token, TokenType};

pub enum Expr {
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: ExprLiteral,
    },
}

impl Expr {
    // brief: Convert an Expression to String recursively, which mainly forcus on print/debug.
    // input:
    // output: String.
    pub fn two_string(&self) -> String {
        match self {
            Expr::Unary { operator, right } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).two_string(); // recursion occurs!
                format!("( {} {} )", operator_str, right_str)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_str = (*left).two_string();
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).two_string();
                format!("( {} {} {} )", operator_str, left_str, right_str)
            }
            Expr::Grouping { expression } => {
                format!("( {} )", (*expression).two_string())
            }
            Expr::Literal { value } => {
                format!("{}", value.two_string())
            }
        }
    }
    pub fn print(&self) {
        println!("{}", self.two_string());
    }
}

pub enum ExprLiteral {
    NumberLiteral(f64),
    StringLiteral(String),
    True,
    False,
    Nil,
}

impl ExprLiteral {
    fn two_string(&self) -> String {
        match self {
            Self::NumberLiteral(v) => v.to_string(),
            Self::StringLiteral(v) => v.clone(),
            Self::True => "True".to_string(),
            Self::False => "False".to_string(),
            Self::Nil => "Nil".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Binary;

    use super::{Expr, ExprLiteral};
    use crate::lexer::token::{LiterialValue, Token, TokenType};

    #[test]
    fn test_expr() {
        let left1 = Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new(Expr::Literal {
                value: ExprLiteral::NumberLiteral(239.0),
            }),
        });
        let right1 = Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: ExprLiteral::NumberLiteral(45.67),
            }),
        });

        let test = Expr::Binary {
            left: left1,
            operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
            right: right1,
        };

        test.print();
    }
    // cargo test expr -- --nocapture
}
