use std::time::UNIX_EPOCH;
use std::{fmt::Debug, time::SystemTime};

use super::{expr::ExprLiteral, interpreter::Interpreter};

pub trait Callable: Debug {
    fn call(
        &self,
        interpreter: &mut Interpreter,
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
pub struct MyCallable;

impl Callable for MyCallable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
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

#[derive(Debug, Copy, Clone)]
pub struct MyClock;

impl Callable for MyClock {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<ExprLiteral>,
    ) -> Result<ExprLiteral, String> {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Ok(ExprLiteral::NumberLiteral(since_the_epoch.as_secs_f64()))
    }

    fn arity(&self) -> usize {
        0
    }
    fn clone_box(&self) -> Box<dyn Callable> {
        Box::new(*self)
    }
}
