use super::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
}
