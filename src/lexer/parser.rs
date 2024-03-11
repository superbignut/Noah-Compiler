use super::{
    expr::{Expr, ExprLiteral},
    stmt::Stmt,
    token::{LiterialValue, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>, //
    current: usize,     // num to index when parse Vec<Token>
}

impl Parser {
    // brief: Create a Parser with Token vector , and set self.current to 0.
    // input:
    // output:
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /*
    program -> declaration * EOF

    declaration -> letDecl | statement

    letDecl -> "let" Identifier ( "=" expression ) ? ";"

    statement -> exprStmt | printStmt | block | ifStmt | whileStmt

    whileStmt -> "while" "(" expression ")" statement

    ifStmt -> "if" "(" expression ")" statement ("else" statement ) ?

    block -> "{" declaration "}"

    exprStmt -> expression ";"

    printstmt -> "print" expression ";"

    expression -> assignment

    assignment -> Identifier "=" assignment | logic_or

    logic_or -> logic_and ( "or" logic_and) *

    logic_and -> equality ( "and" equality) *

    equality -> comparision ( ("!=" | "==") comparision  ) *

    comparision -> term ( ( ">" | ">=" | "<" | "<=") term ) *

    term -> factor ( ( "-" | "+" ) factor ) *

    factor -> unary ( ( "/" | "*") unary ) *

    unary -> ( ( "!" | "-" ) unary ) | call

    call -> primary ( "(" arguments ? ")" ) *

    arguments -> expression ( "," expression ) *

    primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | Identifier
    */

    // brief: Pub function used to Parse a Vec<Stmt>.
    // input:
    // output:
    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = vec![];
        while !self.is_at_end() {
            // while not at end, continue to call self.declaration().
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    // brief: declaration -> letDecl | statement
    // input:
    // output:
    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_tokens(&[TokenType::Let]) {
            match self.let_declaration() {
                Ok(v) => {
                    return Ok(v);
                }
                Err(err) => {
                    self.synchronize();
                    return Err(err);
                }
            }
        }
        match self.statement() {
            Ok(v) => Ok(v),
            Err(err) => {
                self.synchronize(); // Todo: Check, the return of parse() function will be changed. not a single string, but Vec<String>
                Err(err)
            }
        }
    }

    // brief: letDecl -> "let" Identifier ( "=" expression ) ? ";"
    // input:
    // output:
    fn let_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier)?;
        let mut initializer = Expr::Literal {
            value: ExprLiteral::Nil,
        };
        if self.match_tokens(&[TokenType::Equal]) {
            initializer = self.expression()?;
        }
        let _ = self.consume(TokenType::Semicolon)?;

        Ok(Stmt::Let { name, initializer })
    }

    // brief: statement -> exprStmt | printStmt | block | ifStmt
    // input:
    // output:
    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_tokens(&[TokenType::Print]) {
            self.print_statement()
        } else if self.match_tokens(&[TokenType::LeftBrace]) {
            self.block()
        } else if self.match_tokens(&[TokenType::If]) {
            self.if_statement()
        } else if self.match_tokens(&[TokenType::While]) {
            self.while_statement()
        } else if self.match_tokens(&[TokenType::For]) {
            self.for_statement() // Syntactic sugar.
        } else {
            self.expression_statement()
        }
    }

    // for ( initializer condition increment ) body
    // --------------Syntactic sugar------------->
    // {
    //  initializer while ( condition ) { body increment }
    // }

    // brief: for_statement -> "for" "(" ( letDecl | exprStmt | ";" ) expression ? ";" expression ? ")" statement
    // input:
    // output:
    fn for_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen)?;

        let initializer = if self.match_tokens(&[TokenType::Semicolon]) {
            None
        } else if self.match_tokens(&[TokenType::Let]) {
            Some(self.let_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            // Always true.
            Some(Expr::Literal {
                value: ExprLiteral::True,
            })
        };
        self.consume(TokenType::Semicolon)?;

        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::RightParen)?;

        let mut body = self.statement()?;

        if increment.is_some() {
            body = Stmt::Block {
                statements: vec![body, Stmt::Expression(increment.unwrap())],
            }
        }

        body = Stmt::While {
            condition: condition.unwrap(),
            body: Box::new(body),
        };

        if initializer.is_some() {
            body = Stmt::Block {
                statements: vec![initializer.unwrap(), body],
            }
        }

        Ok(body)
    }

    // brief: whileStmt -> "while" "(" expression ")" statement
    // input:
    // output:
    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen)?;

        let condition = self.expression()?;

        self.consume(TokenType::RightParen)?;

        let body = Box::new(self.statement()?);

        Ok(Stmt::While { condition, body })
    }

    // brief: ifStmt -> "if" "(" expression ")" statement ("else" statement ) ?
    // input:
    // output:
    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen)?;

        let condition = self.expression()?;

        self.consume(TokenType::RightParen)?;

        let then_branch = Box::new(self.statement()?);

        let mut else_branch = None;

        if self.match_tokens(&[TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    // brief: printstmt -> "print" expression ";"
    // input:
    // output:
    fn print_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon)?;

        Ok(Stmt::Print(expr))
    }

    // brief: exprStmt -> expression ";"
    // input:
    // output:
    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon)?;

        Ok(Stmt::Expression(expr))
    }

    // brief: block -> "{" declaration "}"
    // input:
    // output:
    fn block(&mut self) -> Result<Stmt, String> {
        let mut statements = vec![];
        // is_at_end check for forgeting closing "}"
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace)?;
        Ok(Stmt::Block { statements })
    }

    // brief: expression -> assignment
    // input:
    // output:
    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    // brief: assignment -> Identifier "=" assignment | logic_or
    // input:
    // output:
    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.logic_or()?;
        if self.match_tokens(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;
            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            } else {
                return Err(format!(
                    "Error occurs when assignment at line: {} at {}.",
                    equals.line_number, equals.lexeme
                ));
            }
        }
        Ok(expr)
    }

    // brief: logic_or -> logic_and ( "or" logic_and) *
    // input:
    // output:
    fn logic_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.logic_and()?;

        while self.match_tokens(&[TokenType::Or]) {
            let operator = self.previous();
            let right_expr = self.logic_and()?;

            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right_expr),
            };
        }
        Ok(expr)
    }

    // brief: logic_and -> equality ( "and" equality) *
    // input:
    // output:
    fn logic_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;

        while self.match_tokens(&[TokenType::And]) {
            let operator = self.previous();
            let right_expr = self.equality()?;

            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right_expr),
            }
        }
        Ok(expr)
    }

    // brief: equality -> comparision ( ("!=" | "==") comparision  ) *
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

    // brief: comparision -> term ( ( ">" | ">=" | "<" | "<=") term ) *
    // input:
    // output:
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

    // brief: term -> factor ( ( "-" | "+" ) factor ) *
    // input:
    // output:
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

    // brief: factor -> unary ( ( "/" | "*") unary ) *
    // input:
    // output:
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

    // brief: unary -> ( ( "!" | "-" ) unary ) | call
    // input:
    // output:
    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right_expr = self.unary()?;

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right_expr),
            });
        }
        self.call()
    }

    // brief: call -> primary ( "(" arguments ? ")" ) *
    // input:
    // output
    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;
        loop {
            if self.match_tokens(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);
                if !self.match_tokens(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RightParen)?;

        if arguments.len() >= 255 {
            return Err(format!(
                "There are too many arguments at line {} at {}.",
                paren.line_number, paren.lexeme
            ));
        }

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    // brief: arguments -> expression ( "," expression ) *
    // input:
    // output
    fn arguments(&mut self) {
        todo!()
    }

    // brief: primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | Idetifier
    // input:
    // output:
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
            Err(format!(
                "Error occur at parsering String at line {} in {}, Maybe an error from Scanner.",
                self.peek().line_number,
                self.peek().lexeme
            ))
        } else if self.match_tokens(&[TokenType::Number]) {
            if let Some(LiterialValue::FloatValue(v)) = self.previous().literial {
                return Ok(Expr::Literal {
                    value: ExprLiteral::NumberLiteral(v),
                });
            }
            Err(format!(
                "Error occur at parsering Number at line {} in {}, Maybe an error from Scanner.",
                self.peek().line_number,
                self.peek().lexeme
            ))
        } else if self.match_tokens(&[TokenType::Identifier]) {
            Ok(Expr::Variable {
                name: self.previous(),
            })
        } else if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            let _ = self.consume(TokenType::RightParen)?; // Consume the RightParen.
            Ok(Expr::Grouping {
                expression: Box::new(expr),
            })
        } else {
            Err(format!(
                "Parsering error occurs for finding nothing to match with at line {} in {}.",
                self.peek().line_number,
                self.peek().lexeme,
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

    // brief: Consume the current token, if tokentype matched.
    // input:
    // output:
    fn consume(&mut self, token_type: TokenType) -> Result<Token, String> {
        if self.check(token_type.clone()) {
            Ok(self.advance())
        } else {
            Err(format!(
                "Parsering error occur when consuming token {} at line: {} in {}.",
                token_type,
                self.peek().line_number,
                self.peek().lexeme,
            ))
        }
    }

    // brief: Synchronize to give up the error code untill find a Unerror Defination..
    // input:
    // output:
    fn synchronize(&mut self) {
        self.advance(); // Consume the error Token.
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::CLass
                | TokenType::Fun
                | TokenType::Let
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,

                _ => {
                    self.advance();
                }
            }
        }
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

    // brief: Check tempToken by using peek().
    // input:
    // output:
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

    // brief: return current token and self.current ++
    // input:
    // output:
    // Attention : if is_at_end() return will be the last one, and current do not increase.
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    // brief: peek the previous token.
    // input:
    // output:
    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    // brief: check if self current is at end.
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

        // let pas = Parser::new(tok).parse().unwrap().two_string();

        // dbg!(pas);
    }

    #[test]
    fn parser_test_two() {
        let sources = "1.0 >= * 3.0".to_string();
        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        match Parser::new(tok).parse() {
            Err(error) => {
                println!("[    Error!    ] ---> {}", error);
            }
            Ok(v) => {
                dbg!(v);
            }
        }
    }

    #[test]
    fn parser_test_three() {
        let sources = "1.0 >= 1.0 + 2.0 == 4.0".to_string();
        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        match Parser::new(tok).parse() {
            Err(error) => {
                println!("[    Error!    ] ---> {}", error);
            }
            Ok(v) => {
                dbg!(v);
            }
        }
    }

    #[test]
    fn parser_test_four() {
        let sources = "- - - - - - - - true".to_string();
        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        match Parser::new(tok).parse() {
            Err(error) => {
                println!("[    Error!    ] ---> {}", error);
            }
            Ok(v) => {
                dbg!(v);
            }
        }
    }

    #[test]
    fn parser_test_five() {
        let sources = "1.0 + ".to_string();
        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        match Parser::new(tok).parse() {
            Err(error) => {
                println!("[    Error!    ] ---> {}", error);
            }
            Ok(v) => {
                dbg!(v);
            }
        }
    }
    #[test]
    fn parser_test_six() {
        let sources = "1.0 + 2.0; \n 2.0 * 3.0 + 4.0;\n".to_string();
        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        match Parser::new(tok).parse() {
            Err(error) => {
                println!("[    Error!    ] ---> {}", error);
            }
            Ok(v) => {
                dbg!(v);
            }
        }
    }

    #[test]
    fn parser_test_seven() {
        let sources = "let abc = 123.0; \n print abc;\n print  2.0 * abc + 4.0;\n".to_string();
        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        match Parser::new(tok).parse() {
            Err(error) => {
                println!("[    Error!    ] ---> {}", error);
            }
            Ok(v) => {
                dbg!(v);
            }
        }
    }

    #[test]
    fn parser_test_eight() {
        let sources =
            "let abc = 123.0; let ltl = true; \n print abc;\n print  2.0 * abc + 4.0 > 0.0 == true;"
                .to_string();
        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        match Parser::new(tok).parse() {
            Err(error) => {
                println!("[    Error!    ] ---> {}", error);
            }
            Ok(v) => {
                dbg!(v);
            }
        }
    }

    #[test]
    fn parser_test_nine() {
        let sources = "let abc = 123.0;let bbb =10.0; abc =bbb= 10.0;".to_string();
        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        match Parser::new(tok).parse() {
            Err(error) => {
                println!("[    Error!    ] ---> {}", error);
            }
            Ok(v) => {
                dbg!(v);
            }
        }
    }

    #[test]
    fn parser_test_ten() {
        let sources = "abc();".to_string();
        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        match Parser::new(tok).parse() {
            Err(error) => {
                println!("[    Error!    ] ---> {}", error);
            }
            Ok(v) => {
                dbg!(v);
            }
        }
    }
    #[test]
    fn parser_test_tenone() {
        let sources = "abc()();".to_string();
        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        match Parser::new(tok).parse() {
            Err(error) => {
                println!("[    Error!    ] ---> {}", error);
            }
            Ok(v) => {
                dbg!(v);
            }
        }
    }

    #[test]
    fn parser_test_tentwo() {
        let sources = "abc(1.0,2.0,3.0)(4.0);".to_string();
        let mut scan = Scanner::new(sources);

        let tok = scan.scan_tokens().unwrap();

        match Parser::new(tok).parse() {
            Err(error) => {
                println!("[    Error!    ] ---> {}", error);
            }
            Ok(v) => {
                dbg!(v);
            }
        }
    }
}
// cargo test <unique keyword> --  --nocapture
