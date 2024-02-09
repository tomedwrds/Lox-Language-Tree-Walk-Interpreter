use std::collections::HashMap;

use crate::{interpreter::{RuntimeError, Value}, lox_callable::LoxClass, scanner::Token};

#[derive(PartialEq, Clone, Debug)]

pub struct LoxInstance {
    pub class: LoxClass,
    pub fields: HashMap<String, Value>,
    //pub instance_name: String
}

impl LoxInstance {
    pub fn get(self, name: Token) -> Result<Value, RuntimeError> {
        if self.fields.contains_key(&name.lexeme) {
            return Ok(self.fields.get(&name.lexeme).unwrap().clone());
        } else {
            return Err(RuntimeError::Class(format!("Undefined propert {}.", name.lexeme)))
        }
    }

    pub fn set(&mut self, name: Token, value: Value) {
        self.fields.insert(name.lexeme, value);
    }
}