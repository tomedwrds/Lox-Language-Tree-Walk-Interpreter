use std::collections::HashMap;

use crate::{expr::{Expr, Literal}, interpreter::{RuntimeError, Value}, lox_callable::{LoxCallable, LoxClass, LoxFunction}, scanner::{Token, TokenType}, stmt::Stmt};

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

        if let Some(method) = self.clone().class.find_method(name.lexeme.clone()) {
            if let Stmt::Function(name,params ,mut code ) = method.stmt {
                let added_this = Stmt::Var(Token { token_type: TokenType::THIS, lexeme: "this".to_string(), literal: None, line: name.line }, Expr::This(Value::LoxInstance(self.clone())));
                code.insert(0, added_this);
                let func_stmt = Stmt::Function(name, params, code);
                return Ok(Value::LoxCallable(Box::new(LoxCallable::LoxFunction( LoxFunction {stmt: func_stmt}))));
            }
        }
        return Err(RuntimeError::Class(format!("Undefined property {}.", name.lexeme)))
        
    }

    pub fn set(&mut self, name: Token, value: Value) {
        self.fields.insert(name.lexeme, value);
    }
}