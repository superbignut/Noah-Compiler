use std::fmt::Debug;

use super::{expr::ExprLiteral, interpreter::Interpreter};

pub trait Callable: Debug {
    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: Vec<ExprLiteral>,
    ) -> Result<ExprLiteral, String>;

    fn arity(&self) -> usize;

    fn clone_box(&self) -> Box<dyn Callable>;
}

impl Clone for Box<dyn Callable> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn Callable> {
    fn eq(&self, other: &Self) -> bool {
        false // Todo
    }
}

#[derive(Debug, Copy, Clone)]
struct MyCallable;

impl Callable for MyCallable {
    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: Vec<ExprLiteral>,
    ) -> Result<ExprLiteral, String> {
        todo!()
    }

    fn arity(&self) -> usize {
        todo!()
    }

    fn clone_box(&self) -> Box<dyn Callable> {
        Box::new(*self)
    }
}
