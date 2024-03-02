use super::token::{LiterialValue, Token, TokenType};

#[derive(Clone, Debug)]
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
    Variable {
        name: Token,
    },
}

impl Expr {
    // brief: Convert an Expression to String recursively, which mainly forcus on print/debug.
    // input:
    // output: String.
    pub fn two_string(&self) -> String {
        match self {
            Expr::Literal { value } => value.two_string().to_string(),
            Expr::Unary { operator, right } => {
                let operator_str = operator.lexeme.clone();
                let right_str = right.two_string(); // recursion occurs!
                format!("( {} {} )", operator_str, right_str)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_str = left.two_string();
                let operator_str = operator.lexeme.clone();
                let right_str = right.two_string();
                format!("( {} {} {} )", left_str, operator_str, right_str)
            }
            Expr::Grouping { expression } => {
                format!("( {} )", expression.two_string())
            }
            Expr::Variable { name } => name.lexeme.clone(), // Todo: Check.
        }
    }
    pub fn print(&self) {
        println!("{}", self.two_string());
    }
}

#[derive(Clone, Debug)]
pub enum ExprLiteral {
    NumberLiteral(f64),
    StringLiteral(String),
    True,
    False,
    Nil,
}

impl ExprLiteral {
    pub fn two_string(&self) -> String {
        match self {
            Self::NumberLiteral(v) => v.to_string(),
            Self::StringLiteral(v) => v.clone(),
            Self::True => "True".to_string(),
            Self::False => "False".to_string(),
            Self::Nil => "Nil".to_string(),
        }
    }

    pub fn is_equal(&self, other: &ExprLiteral) -> bool {
        match (self, other) {
            (ExprLiteral::NumberLiteral(v1), ExprLiteral::NumberLiteral(v2)) => *v1 == *v2,
            (ExprLiteral::StringLiteral(s1), ExprLiteral::StringLiteral(s2)) => *s1 == *s2,
            (ExprLiteral::True, ExprLiteral::True) => true,
            (ExprLiteral::False, ExprLiteral::False) => true,
            (ExprLiteral::Nil, ExprLiteral::Nil) => true,
            _ => false,
        }
    }
    // brief: Increase the degree of code coupling.
    // input:
    // output:
    // pub fn from_token(token: Token) -> Result<ExprLiteral, String> {
    //     match token.token_type {
    //         TokenType::False => Ok(ExprLiteral::False),
    //         TokenType::True => Ok(ExprLiteral::True),
    //         TokenType::Nil => Ok(ExprLiteral::Nil),
    //         TokenType::String => {
    //             if let Some(LiterialValue::StringValue(v)) = token.literial {
    //                 return Ok(ExprLiteral::StringLiteral(v));
    //             }
    //             Err(String::from("Error occur!"))
    //         }
    //         TokenType::Number => {
    //             if let Some(LiterialValue::FloatValue(v)) = token.literial {
    //                 return Ok(ExprLiteral::NumberLiteral(v));
    //             }
    //             Err(String::from("Error occur!"))
    //         }
    //         _ => Err(String::from("Error occur!")),
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use std::fmt::Binary;

    use super::{Expr, ExprLiteral};
    use super::{LiterialValue, Token, TokenType};

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
