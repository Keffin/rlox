use std::collections::HashMap;

use crate::{expr::LiteralRepresentations, token::Token};

#[derive(Debug)]
pub struct EnvironmentErr {
    reason: String,
}

pub struct Environment {
    pub values: HashMap<String, LiteralRepresentations>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralRepresentations) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<LiteralRepresentations, EnvironmentErr> {
        if self.values.contains_key(&name.lexeme) {
            match self.values.get(&name.lexeme) {
                Some(val) => return Ok(val.clone()),
                None => Err(EnvironmentErr {
                    reason: format!("Environment coult not find key {}", &name.lexeme),
                }),
            }
        } else {
            Err(EnvironmentErr {
                reason: format!("Undefined variable '{}'.", &name.lexeme),
            })
        }
    }
}
