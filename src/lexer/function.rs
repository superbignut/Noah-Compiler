use crate::lexer::environment::Environment;

use super::{
    callable::Callable, expr::ExprLiteral, interpreter::Interpreter, stmt::Stmt, stmt::Stmt::Block,
    stmt::Stmt::Function, token::Token,
};

#[derive(Debug, Clone)]
pub struct MyFunction {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
}
impl MyFunction {
    pub fn new(declaration: Stmt) -> Result<Self, String> {
        if let Stmt::Function { name, params, body } = declaration.clone() {
            if let Stmt::Block { statements } = *body {
                return Ok(Self {
                    name,
                    params,
                    body: statements,
                });
            }
        }
        Err(
            format!("Error occur when construct MyFunction, find mismatched type, expected Stmt::Function got {:?}.", declaration)
        )
    }
}
impl Callable for MyFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<ExprLiteral>,
    ) -> Result<ExprLiteral, String> {
        let mut environment = Environment::new(Some(Box::new(interpreter.globals.clone())));
        //let mut environment = Environment::new(Some(Box::new(self.closure.clone())));
        // new  clean environment including no variables expect gloabls. so the funtion also can't be found.

        for (index, item) in self.params.iter().enumerate() {
            environment.define(item.lexeme.clone(), arguments[index].clone()); // including params. Combine virtual params with real arguments.
        }

        interpreter.execute_function_block(&self.body, environment) // execute and refresh global.

        // Ok(ExprLiteral::Nil)
    }

    fn clone_box(&self) -> Box<dyn Callable> {
        Box::new(self.clone())
    }

    // Todo: add two_string()
}
