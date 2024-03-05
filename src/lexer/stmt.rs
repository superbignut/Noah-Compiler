use super::{expr::Expr, token::Token};

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var { name: Token, initializer: Expr },
    Block { statements: Vec<Stmt> },
}
