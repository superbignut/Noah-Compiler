use super::{
    expr::{Expr, ExprLiteral},
    token::{Token, TokenType},
};

struct Interpreter {}

impl Interpreter {
    fn match_expr(&self, expr: Expr) -> ExprLiteral {
        match expr {
            Expr::Literal { value } => value,
            Expr::Grouping { expression } => self.evaluate(&expression),
            Expr::Unary { operator, right } => {
                if operator.token_type == TokenType::Minus {
                    if let ExprLiteral::NumberLiteral(v) = self.evaluate(&right) {
                        return ExprLiteral::NumberLiteral(-v);
                    }
                    todo!() // may be an error.
                } else if operator.token_type == TokenType::Bang {
                    return self.is_truthy(self.evaluate(&right));
                }
                ExprLiteral::Nil // may be an error
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_operand = self.evaluate(&left);
                let right_operand = self.evaluate(&right);

                match operator.token_type {
                    TokenType::Minus => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(operator, left_operand, right_operand)
                        {
                            return ExprLiteral::NumberLiteral(l_number - r_number);
                        }
                        todo!() //Error
                    }

                    TokenType::Slash => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(operator, left_operand, right_operand)
                        {
                            return ExprLiteral::NumberLiteral(l_number / r_number);
                        }
                        todo!() //Error
                    }

                    TokenType::Star => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(operator, left_operand, right_operand)
                        {
                            return ExprLiteral::NumberLiteral(l_number * r_number);
                        }
                        todo!() //Error
                    }

                    TokenType::Plus => match (left_operand, right_operand) {
                        (
                            ExprLiteral::NumberLiteral(l_number),
                            ExprLiteral::NumberLiteral(r_number),
                        ) => {
                            return ExprLiteral::NumberLiteral(l_number + r_number);
                        }

                        (
                            ExprLiteral::StringLiteral(l_string),
                            ExprLiteral::StringLiteral(r_string),
                        ) => {
                            return ExprLiteral::StringLiteral(format!("{}{}", l_string, r_string));
                        }

                        _ => todo!(), // Error
                    },

                    TokenType::Greater => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(operator, left_operand, right_operand)
                        {
                            if l_number > r_number {
                                return ExprLiteral::True;
                            } else {
                                return ExprLiteral::False;
                            }
                        }
                        todo!() //Error
                    }

                    TokenType::GreaterEqual => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(operator, left_operand, right_operand)
                        {
                            if l_number >= r_number {
                                return ExprLiteral::True;
                            } else {
                                return ExprLiteral::False;
                            }
                        }
                        todo!() //Error
                    }
                    TokenType::Less => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(operator, left_operand, right_operand)
                        {
                            if l_number < r_number {
                                return ExprLiteral::True;
                            } else {
                                return ExprLiteral::False;
                            }
                        }
                        todo!() //Error
                    }
                    TokenType::LessEqual => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(operator, left_operand, right_operand)
                        {
                            if l_number <= r_number {
                                return ExprLiteral::True;
                            } else {
                                return ExprLiteral::False;
                            }
                        }
                        todo!() //Error
                    }
                    TokenType::EqualEqual => {
                        if left_operand.is_equal(&right_operand) {
                            ExprLiteral::True
                        } else {
                            ExprLiteral::False
                        }
                    }

                    TokenType::BangEqual => {
                        if !left_operand.is_equal(&right_operand) {
                            ExprLiteral::True
                        } else {
                            ExprLiteral::False
                        }
                    }

                    _ => {
                        todo!() // Error.
                    }
                }
            }
        }
    }

    fn check_number_operands(
        &self,
        operator: Token,
        l_operand: ExprLiteral,
        r_operand: ExprLiteral,
    ) -> (bool, f64, f64) {
        todo!()
    }
    fn check_number_operand(&self, operator: Token, operand: ExprLiteral) -> (bool, f64) {
        todo!()
    }
    fn is_truthy(&self, expr: ExprLiteral) -> ExprLiteral {
        match expr {
            ExprLiteral::False | ExprLiteral::Nil => ExprLiteral::False,
            _ => ExprLiteral::True,
        }
    }
    fn evaluate(&self, expr: &Expr) -> ExprLiteral {
        todo!() // recursively.
    }
}
