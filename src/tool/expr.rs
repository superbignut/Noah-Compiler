use crate::lexer::token::{Token, TokenType};

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
    // brief: Convert an Expression to String, which mainly forcus on print/debug.
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
                todo!()
            }
            Expr::Grouping { expression } => {
                todo!()
            }
            Expr::Literal { value } => {
                todo!()
            }
        }
    }
    pub fn print(&self) {
        println!("{}", self.two_string());
    }
}

pub enum ExprLiteral {
    Number,
    String,
    True,
    False,
    Nil,
}

// pub struct BinaryExpr {
//     left: Box<Expr>,
//     operator: Token,
//     right: Box<Expr>,
// }
