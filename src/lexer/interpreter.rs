use super::{
    expr::{Expr, ExprLiteral},
    token::{Token, TokenType},
};

struct Interpreter {}

impl Interpreter {
    // brief:
    // input:
    // output:
    fn evaluate(&self, expr: &Expr) -> Result<ExprLiteral, String> {
        self.match_expr(expr.clone())
    }
    // brief:
    // input:
    // output:
    fn match_expr(&self, expr: Expr) -> Result<ExprLiteral, String> {
        match expr {
            Expr::Literal { value } => Ok(value),
            Expr::Grouping { expression } => self.evaluate(&expression),
            Expr::Unary { operator, right } => {
                if operator.token_type == TokenType::Minus {
                    if let ExprLiteral::NumberLiteral(v) = self.evaluate(&right)? {
                        return Ok(ExprLiteral::NumberLiteral(-v));
                    }
                    return Err(format!(
                        "Error occur when interpreter number at line {} at {}.",
                        operator.line_number, operator.lexeme
                    ));
                } else if operator.token_type == TokenType::Bang {
                    return Ok(self.is_truthy(self.evaluate(&right)?));
                }
                Err(format!(
                    "Error occur when interpreter at line {} at {} for no matching unary operator.",
                    operator.line_number, operator.lexeme
                ))
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_operand = self.evaluate(&left)?; // recursively.
                let right_operand = self.evaluate(&right)?;

                match operator.token_type {
                    TokenType::Minus => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(left_operand, right_operand)
                        {
                            return Ok(ExprLiteral::NumberLiteral(l_number - r_number));
                        }
                        Err(format!(
                            "Error occur when interpreter at line {} at {} for some wrong operand.",
                            operator.line_number, operator.lexeme
                        ))
                    }

                    TokenType::Slash => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(left_operand, right_operand)
                        {
                            return Ok(ExprLiteral::NumberLiteral(l_number / r_number));
                        }
                        Err(format!(
                            "Error occur when interpreter at line {} at {} for some wrong operand.",
                            operator.line_number, operator.lexeme
                        ))
                    }

                    TokenType::Star => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(left_operand, right_operand)
                        {
                            return Ok(ExprLiteral::NumberLiteral(l_number * r_number));
                        }
                        Err(format!(
                            "Error occur when interpreter at line {} at {} for some wrong operand.",
                            operator.line_number, operator.lexeme
                        ))
                    }

                    TokenType::Plus => match (left_operand, right_operand) {
                        (
                            ExprLiteral::NumberLiteral(l_number),
                            ExprLiteral::NumberLiteral(r_number),
                        ) => Ok(ExprLiteral::NumberLiteral(l_number + r_number)),

                        (
                            ExprLiteral::StringLiteral(l_string),
                            ExprLiteral::StringLiteral(r_string),
                        ) => Ok(ExprLiteral::StringLiteral(format!(
                            "{}{}",
                            l_string, r_string
                        ))),

                        _ => {
                            Err(format!(
                            "Error occur when interpreter at line {} at {} for some wrong operand.",
                            operator.line_number, operator.lexeme
                        ))
                        }
                    },

                    TokenType::Greater => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(left_operand, right_operand)
                        {
                            if l_number > r_number {
                                return Ok(ExprLiteral::True);
                            } else {
                                return Ok(ExprLiteral::False);
                            }
                        }
                         Err(format!(
                            "Error occur when interpreter at line {} at {} for some wrong operand.",
                            operator.line_number, operator.lexeme
                        ))
                    }

                    TokenType::GreaterEqual => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(left_operand, right_operand)
                        {
                            if l_number >= r_number {
                                return Ok(ExprLiteral::True);
                            } else {
                                return Ok(ExprLiteral::False);
                            }
                        }
                         Err(format!(
                            "Error occur when interpreter at line {} at {} for some wrong operand.",
                            operator.line_number, operator.lexeme
                        ))
                    }
                    TokenType::Less => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(left_operand, right_operand)
                        {
                            if l_number < r_number {
                                return Ok(ExprLiteral::True);
                            } else {
                                return Ok(ExprLiteral::False);
                            }
                        }
                         Err(format!(
                            "Error occur when interpreter at line {} at {} for some wrong operand.",
                            operator.line_number, operator.lexeme
                        ))
                    }
                    TokenType::LessEqual => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(left_operand, right_operand)
                        {
                            if l_number <= r_number {
                                return Ok(ExprLiteral::True);
                            } else {
                                return Ok(ExprLiteral::False);
                            }
                        }
                         Err(format!(
                            "Error occur when interpreter at line {} at {} for some wrong operand.",
                            operator.line_number, operator.lexeme
                        ))
                    }
                    TokenType::EqualEqual => {
                        if left_operand.is_equal(&right_operand) {
                            Ok(ExprLiteral::True)
                        } else {
                            Ok(ExprLiteral::False)
                        }
                    }

                    TokenType::BangEqual => {
                        if !left_operand.is_equal(&right_operand) {
                            Ok(ExprLiteral::True)
                        } else {
                            Ok(ExprLiteral::False)
                        }
                    }

                    _ => {
                         Err(format!(
                            "Error occur when interpreter at line {} at {} for no matchine Binary operator.",
                            operator.line_number, operator.lexeme
                        ))
                    }
                }
            }
        }
    }
    // brief:
    // input:
    // output:
    fn check_number_operands(
        &self,
        l_operand: ExprLiteral,
        r_operand: ExprLiteral,
    ) -> (bool, f64, f64) {
        if let (true, v1) = self.check_number_operand(l_operand) {
            if let (true, v2) = self.check_number_operand(r_operand) {
                return (true, v1, v2);
            }
        }
        (false, 0.0, 0.0)
    }
    // brief:
    // input:
    // output:
    fn check_number_operand(&self, operand: ExprLiteral) -> (bool, f64) {
        if let ExprLiteral::NumberLiteral(v) = operand {
            return (true, v);
        }
        (false, 0.0)
    }
    // brief:
    // input:
    // output:
    fn is_truthy(&self, expr: ExprLiteral) -> ExprLiteral {
        match expr {
            ExprLiteral::False | ExprLiteral::Nil => ExprLiteral::False,
            _ => ExprLiteral::True,
        }
    }
}

// Todo: add a test.
