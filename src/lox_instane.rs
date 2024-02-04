use crate::{enviroment::create_enviroment, interpreter::{Interpreter, Value}, stmt::Stmt};

#[derive(Debug, PartialEq, Clone)]
pub enum LoxInstance {
    LoxFunction(LoxFunction)
}

pub trait Callable {
    fn arity(self) -> usize;
    fn call_function(self, interpreter: &Interpreter, arguments: Vec<Value>);
}

#[derive(Debug, PartialEq, Clone)]

struct LoxFunction {
    stmt: Stmt
}
impl Callable for LoxFunction {
    fn call_function(self, interpreter: &Interpreter, arguments: Vec<Value>) {
        let mut enviroment = create_enviroment(Some(interpreter.global));
        if let Stmt::Function(name, params, body) = self.stmt {
            for i in 0..params.len() {
                enviroment.put(params[i].lexeme, arguments[i])
            }

            interpreter.interpret_statement_block(body,enviroment);
        } else {
            panic!("Interpreter has failed to enforce type checking on statements.")
        }
        
    }
    fn arity(self) -> usize {
        if let Stmt::Function(name, params, body) = self.stmt {
            return params.len()
        } else {
            panic!("Interpreter has failed to enforce type checking on statements.")
        }   
    }
}