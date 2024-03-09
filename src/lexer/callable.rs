use super::{expr::ExprLiteral, interpreter::Interpreter};

pub trait Callable {
    fn call(&self, interpreter: Interpreter, arguments: Vec<ExprLiteral>) {
        todo!()
    }
}
