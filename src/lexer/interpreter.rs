use super::{
    environment::Environment,
    expr::{Expr, ExprLiteral},
    parser::Parser,
    stmt::Stmt,
    token::{Token, TokenType},
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    // brief:
    // input:
    // output:
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    // brief:
    // input:
    // output:
    pub fn interpreter(&mut self, statements: &Vec<Stmt>) -> Result<(), String> {
        for statement in statements {
            match statement {
                // If just an expression.
                Stmt::Expression(v) => {
                    let _ = self.evaluate(v)?;
                }
                // If a print statement.
                Stmt::Print(v) => {
                    println!("{}", (self.evaluate(v)?).two_string());
                }
                // If a Var defination.
                Stmt::Var { name, initializer } => {
                    let value;
                    if *initializer
                        != (Expr::Literal {
                            value: ExprLiteral::Nil,
                        })
                    {
                        value = self.evaluate(initializer)?;
                        self.environment.define(name.lexeme.clone(), value);
                    }
                }
            }
        }
        Ok(())
    }

    // brief:
    // input:
    // output:
    pub fn evaluate(&mut self, expr: &Expr) -> Result<ExprLiteral, String> {
        self.match_expr(expr)
    }
    // brief:
    // input:
    // output:
    fn match_expr(&mut self, expr: &Expr) -> Result<ExprLiteral, String> {
        match expr {
            // 1 Literal
            Expr::Literal { value } => Ok(value.clone()),

            // 2 Grouping
            Expr::Grouping { expression } => self.evaluate(expression),

            // 3 Unary
            Expr::Unary { operator, right } => {
                if operator.token_type == TokenType::Minus {
                    if let ExprLiteral::NumberLiteral(v) = self.evaluate(right)? {
                        return Ok(ExprLiteral::NumberLiteral(-v));
                    }
                    return Err(format!(
                        "Error occur when interpreter number at line {} at {}.",
                        operator.line_number, operator.lexeme
                    ));
                } else if operator.token_type == TokenType::Bang {
                    let evaluated = self.evaluate(right)?;
                    return Ok(self.is_truthy(evaluated));
                }
                Err(format!(
                    "Error occur when interpreter at line {} at {} for no matching unary operator.",
                    operator.line_number, operator.lexeme
                ))
            }
            // 4 Variable
            Expr::Variable { name } => Ok(self.environment.get(name)?),

            Expr::Assign { name, value } => {
                let real_value = self.evaluate(value)?;
                self.environment.assign(name, real_value.clone())?;
                Ok(real_value)
            }

            // 5 Binary
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_operand = self.evaluate(left)?; // recursnively.
                let right_operand = self.evaluate(right)?; // recursively.

                match operator.token_type {
                    TokenType::Minus => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(&left_operand, &right_operand)
                        {
                            return Ok(ExprLiteral::NumberLiteral(l_number - r_number));
                        }
                        Err(format!(
                            "Error occur when interpreter at line {} at {} for some wrong operand.",
                            operator.line_number, operator.lexeme
                        ))
                    },
                    TokenType::Slash => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(&left_operand, &right_operand)
                        {
                            return Ok(ExprLiteral::NumberLiteral(l_number / r_number));
                        }
                        Err(format!(
                            "Error occur when interpreter at line {} at {} for some wrong operand.",
                            operator.line_number, operator.lexeme
                        ))
                    },
                    TokenType::Star => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(&left_operand, &right_operand)
                        {
                            return Ok(ExprLiteral::NumberLiteral(l_number * r_number));
                        }
                        Err(format!(
                            "Error occur when interpreter at line {} at {} for some wrong operand.",
                            operator.line_number, operator.lexeme
                        ))
                    },
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
                            self.check_number_operands(&left_operand, &right_operand)
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
                    },
                    TokenType::GreaterEqual => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(&left_operand, &right_operand)
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
                    },
                    TokenType::Less => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(&left_operand, &right_operand)
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
                    },
                    TokenType::LessEqual => {
                        if let (true, l_number, r_number) =
                            self.check_number_operands(&left_operand, &right_operand)
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
                    },
                    TokenType::EqualEqual => {
                        if left_operand.is_equal(&right_operand) {
                            Ok(ExprLiteral::True)
                        } else {
                            Ok(ExprLiteral::False)
                        }
                    },
                    TokenType::BangEqual => {
                        if !left_operand.is_equal(&right_operand) {
                            Ok(ExprLiteral::True)
                        } else {
                            Ok(ExprLiteral::False)
                        }
                    },
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
        l_operand: &ExprLiteral,
        r_operand: &ExprLiteral,
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
    fn check_number_operand(&self, operand: &ExprLiteral) -> (bool, f64) {
        if let ExprLiteral::NumberLiteral(v) = operand {
            return (true, *v);
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

#[cfg(test)]
mod tests {

    use super::Interpreter;
    use super::Parser;
    use crate::Scanner;

    #[test]
    fn test_inter_one() {
        let sources = "1.0 * 3.0 * 2.0 + 2.0 * 4.1 = 14.0".to_string();
        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        let pas = Parser::new(tok).parse().unwrap();

        // match Interpreter::new().evaluate(&pas) {
        //     Ok(v) => {
        //         println!("[    PASS!     ] ---> {}", v.two_string());
        //     }
        //     Err(v) => {
        //         println!("[    Error!    ] ---> {}", v);
        //     }
        // }
        //dbg!(pas);
    }

    #[test]
    fn test_inter_two() {
        let sources =
            "1.0 * 3.0 * ( 2.0 + 14.0 ) * 4.0 / 8.0 ; \n print \" Successfully!! \"; ".to_string();

        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        let pas = Parser::new(tok).parse().unwrap();

        match Interpreter::new().interpreter(&pas) {
            Ok(()) => {
                println!("[    PASS!     ] ---> Compile Successfully.");
            }
            Err(v) => {
                println!("[    Error!    ] ---> {}", v);
            }
        }
        //        dbg!(pas);
    }
    #[test]
    fn test_inter_three() {
        let sources = "var a = 10.0; var b = 2.0; print a + b + 12.0; ".to_string();

        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        let pas = Parser::new(tok).parse().unwrap();

        match Interpreter::new().interpreter(&pas) {
            Ok(()) => {
                println!("[    PASS!     ] ---> Compile Successfully.");
            }
            Err(v) => {
                println!("[    Error!    ] ---> {}", v);
            }
        }
        //        dbg!(pas);
    }
    #[test]
    fn test_inter_four() {
        let sources = "var a = 10.0; var b = 2.0; print a + b + 12.0 >= 25.0 == true; ".to_string();

        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        let pas = Parser::new(tok).parse().unwrap();

        match Interpreter::new().interpreter(&pas) {
            Ok(()) => {
                println!("[    PASS!     ] ---> Compile Successfully.");
            }
            Err(v) => {
                println!("[    Error!    ] ---> {}", v);
            }
        }
        //        dbg!(pas);
    }

    #[test]
    fn test_inter_five() {
        let sources = "var a = 10.0; print a = 20.0; a = a + 20.0; print a ; ".to_string();

        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        let pas = Parser::new(tok).parse().unwrap();

        match Interpreter::new().interpreter(&pas) {
            Ok(()) => {
                println!("[    PASS!     ] ---> Compile Successfully.");
            }
            Err(v) => {
                println!("[    Error!    ] ---> {}", v);
            }
        }
        //        dbg!(pas);
    }
}

// cargo test unique-keyword -- --nocapture
