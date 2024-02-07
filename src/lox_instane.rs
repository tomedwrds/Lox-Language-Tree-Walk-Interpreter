use crate::{enviroment::create_enviroment, interpreter::{self, Interpreter, Value}, stmt::Stmt};

#[derive(Debug, PartialEq, Clone)]
pub enum LoxInstance {
    LoxFunction(LoxFunction)
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
            interpreter.interpret_statement_block(body,enviroment);
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