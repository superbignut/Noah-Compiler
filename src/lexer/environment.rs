use super::{expr::ExprLiteral, token::Token};
use std::collections::{btree_map::OccupiedEntry, hash_map::VacantEntry, HashMap};

pub struct Environment {
    values: HashMap<String, ExprLiteral>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: ExprLiteral) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<ExprLiteral, String> {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => Err(format!("Undefined variable {}.", name.lexeme)),
        }
    }

    pub fn assign(&mut self, name: &Token, value: ExprLiteral) -> Result<(), String> {
        match self.values.get_mut(&name.lexeme) {
            Some(v) => {
                *v = value;
                Ok(())
            }
            None => Err(format!("Undefined variable {}.", name.lexeme)),
        }
    }
}
