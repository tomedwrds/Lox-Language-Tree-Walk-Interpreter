use std::collections::HashMap;

use crate::{interpreter::{RuntimeError, Value}, lox_callable::{LoxClass, LoxFunction, LoxCallable}, scanner::Token};

#[derive(PartialEq, Clone, Debug)]

pub struct LoxInstance {
    pub class: LoxClass,
    pub fields: HashMap<String, Value>,
}

impl LoxInstance {
    pub fn get(self, name: Token) -> Result<Value, RuntimeError> {
        if self.fields.contains_key(&name.lexeme) {
            return Ok(self.fields.get(&name.lexeme).unwrap().clone());
        }

        let method = self.class.find_method(name.lexeme.clone());
        if method != None {
            return Ok(Value::LoxCallable(LoxCallable::LoxFunction(method.unwrap())));
        }

       
        return Err(RuntimeError::Class(format!("Undefined property {}.", name.lexeme)))
        
    }

    pub fn set(&mut self, name: Token, value: Value) {
        self.fields.insert(name.lexeme, value);
    }
}