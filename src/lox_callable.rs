use std::{collections::HashMap};

use crate::{enviroment::create_enviroment, interpreter::{self, Interpreter, RuntimeError, Value}, lox_instance::LoxInstance, stmt::Stmt};

#[derive(Debug, PartialEq, Clone)]
pub enum LoxCallable {
    LoxFunction(LoxFunction),
    LoxClass(LoxClass)
}

pub trait Callable {
    fn arity(self) -> usize;
    fn call_function(self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value;
}

#[derive(Debug, PartialEq, Clone)]

pub struct LoxFunction {
    pub stmt: Stmt
}
impl Callable for LoxFunction {
    fn call_function(self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        let env = interpreter.enviroment.clone();
        let mut enviroment = create_enviroment(Some(interpreter.global.clone()));

        if let Stmt::Function(name, params, body) = self.stmt {
            for i in 0..params.len() {
                enviroment.put(params[i].lexeme.clone(), arguments[i].clone())
            }
            if let Err(error_return) = interpreter.interpret_statement_block(body,enviroment) {
                if let RuntimeError::Return(return_value_option) = error_return {
                    if let Some(return_value) = return_value_option {
                        interpreter.enviroment = env;
                        return return_value
                    }   
                }
            }

        } else {
            panic!("Interpreter has failed to enforce type checking on statements.")
        }
        interpreter.enviroment = env;
        return Value::Nil
        
    }
    fn arity(self) -> usize {
        if let Stmt::Function(name, params, body) = self.stmt {
            return params.len()
        } else {
            panic!("Interpreter has failed to enforce type checking on statements.")
        }   
    }
}


#[derive(Debug, PartialEq, Clone)]

pub struct LoxClass {
    pub name: String,
    pub methods: HashMap<String, LoxFunction>,
    pub superclass: Box<Option<LoxClass>>
}

impl Callable for LoxClass {
    fn call_function(self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        let instance = LoxInstance {
            class: self.clone(),
            fields: HashMap::new(),
        };

        let initalizer = self.find_method("init".to_string());
        if let Some(initalizer_func) = initalizer {
            initalizer_func.call_function(interpreter, arguments);
        }
        return Value::LoxInstance(instance);
    }
    fn arity(self) -> usize {
        let initalizer = self.find_method("init".to_string());
        if let Some(initalizer_func) = initalizer {
            return initalizer_func.arity()
        }
        return 0
    }
}

impl LoxClass {
    pub fn find_method(self, name: String) ->Option<LoxFunction> {
        if let Some(method) = self.methods.get(&name) {
            return Some(method.clone())
        }

        if let Some(superclass_exists) = *self.superclass {
            return superclass_exists.find_method(name)
        }
        return None
    }
}
