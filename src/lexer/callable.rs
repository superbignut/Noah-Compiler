use std::fmt::Debug;

use super::{expr::ExprLiteral, interpreter::Interpreter};

pub trait Callable: Debug {
    fn call(&self, interpreter: Interpreter, arguments: Vec<ExprLiteral>) {
        todo!()
    }

    fn clone_box(&self) -> Box<dyn Callable>;
}

impl Clone for Box<dyn Callable> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn Callable> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
struct MyCallable;

impl Callable for MyCallable {
    fn call(&self, interpreter: Interpreter, arguments: Vec<ExprLiteral>) {
        todo!()
    }

    fn clone_box(&self) -> Box<dyn Callable> {
        todo!()
    }
}
