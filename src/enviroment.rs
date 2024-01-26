use std::collections::HashMap;

use crate::{interpreter::{RuntimeError, Value}, scanner::Token};

#[derive(Clone)]
pub struct Enviroment {
    pub values: HashMap<String, Value>,
    enclosing: Option<Box<Enviroment>>
}

pub fn create_enviroment(enclosing: Option<Enviroment>) -> Enviroment {
    Enviroment {
        values: HashMap::new(),
        enclosing: match enclosing {
            Some(env) => Some(Box::new(env)),
            None => None
        }
    }
}

impl Enviroment {
    pub fn enclosing(&mut self, env: Enviroment) {
        self.enclosing = Some(Box::new(env));
    }

    pub fn put(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, token: Token) -> Result<Value, RuntimeError> {
        if let Some(value) = self.values.get(&token.lexeme) {
            Ok(value.clone())
        } else if let Some(env) = &self.enclosing {
            if let Ok(value) = (*env.clone()).get(token.clone()) {
                Ok(value.clone())
            }  else {
                Err(RuntimeError::Variable(token.clone(), format!("Undefined variable '{:?}'.", token.lexeme)))
            }    
        } else {
            Err(RuntimeError::Variable(token.clone(), format!("Undefined variable '{:?}'.", token.lexeme)))
        }

    }

    pub fn assign(&mut self, token: Token, value: &Value) -> Result<(), RuntimeError> {
        match self.values.get(&token.lexeme) {
            Some(v) => {
                self.values.insert(token.lexeme, value.clone());
                return Ok(());
            },
            None => Err(RuntimeError::Variable(token.clone(), format!("Undefined variable '{:?}'.", token.lexeme)))
        }
    }
}