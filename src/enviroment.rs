use std::collections::HashMap;

use crate::{interpreter::{RuntimeError, Value}, scanner::Token};

pub struct Enviroment {
    pub values: HashMap<String, Value>
}

impl Enviroment {
    pub fn put(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, token: Token) -> Result<Value, RuntimeError> {
        if let Some(value) = self.values.get(&token.lexeme) {
            Ok(value.clone())
        } else {
            Err(RuntimeError::Variable(token.clone(), format!("Undefined variable '{:?}'.", token.lexeme)))
        }

    }
}