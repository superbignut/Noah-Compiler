use super::{expr::ExprLiteral, token::Token};
use std::collections::{btree_map::OccupiedEntry, hash_map::VacantEntry, HashMap};

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    values: HashMap<String, ExprLiteral>,
}

impl Environment {
    pub fn new(env: Option<Box<Environment>>) -> Self {
        Self {
            enclosing: env,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: ExprLiteral) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<ExprLiteral, String> {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => {
                if let Some(v) = &self.enclosing {
                    v.get(name)
                } else {
                    Err(format!("Undefined variable {}.", name.lexeme))
                }
            }
        }
    }

    pub fn assign(&mut self, name: &Token, value: ExprLiteral) -> Result<(), String> {
        match self.values.get_mut(&name.lexeme) {
            Some(v) => {
                *v = value;
                Ok(())
            }
            None => {
                if let Some(v) = &mut self.enclosing {
                    v.assign(name, value)
                } else {
                    Err(format!("Undefined variable {}.", name.lexeme))
                }
            }
        }
    }
}
