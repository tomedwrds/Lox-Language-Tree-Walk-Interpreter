use std::collections::HashMap;

use crate::{enviroment::{create_enviroment, Enviroment}, expr::{Expr, Literal}, lox_instane::{Callable, LoxFunction, LoxInstance}, scanner::{Token, TokenType}, stmt::Stmt};
#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    LoxInstance(LoxInstance)
}

#[derive(Debug)]
pub enum RuntimeError {
    Variable(Token, String),
    Type(String),
    Function(String),
    Return(Option<Value>)
}
mod tests;

#[derive(Debug)]
pub struct Interpreter {
    pub enviroment: Enviroment,
    pub global: Enviroment
}

pub fn interpret(statements: Vec<Stmt>) {
    let mut interpreter = Interpreter {
        global: create_enviroment(None),
        enviroment: create_enviroment(None)
    };
    interpreter.interpret(statements);
}

impl Interpreter {
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements.iter() {
            let stmt = self.interpret_statement(statement.clone());
            if let Err(error) = stmt {
                print!{"{:?}", error};
            }
        }
    }

    fn interpret_statement(&mut self, stmt: Stmt) -> Result<(), RuntimeError>  {
        match stmt {
            Stmt::Block(ve) => self.interpret_statement_block(ve, create_enviroment(Some(self.enviroment.clone()))),
            Stmt::If(c,i ,e) => self.intepret_statement_if(c, *i, e),
            Stmt::Expression(e) => self.interpret_statement_expression(e),
            Stmt::Function(n, p, c) => self.interpret_statement_function(n, p, c),
            Stmt::Print(e) => self.interpret_statement_print(e),
            Stmt::Return(t,e ) => self.interpret_statement_return(t, e),
            Stmt::Var(t, e) => self.interpret_statement_variable(t, e),
            Stmt::While(e, s) => self.interpret_statement_while(e, *s),
        }
    }

    fn interpret_statement_while(&mut self, condition: Expr, stmt: Stmt) -> Result<(), RuntimeError> {
        while self.is_truth(condition.clone()) {
            self.interpret_statement(stmt.clone())?
        } 
        Ok(())
    }

    fn is_truth(&mut self, expr: Expr) -> bool {
        let value = self.interpret_expression(expr).unwrap();
        if let Value::Bool(bool_expr) = value {
            return bool_expr
        } else {
            return false
        }
    }
    fn intepret_statement_if(&mut self, condition: Expr, if_stmt: Stmt, else_stmt: Option<Box<Stmt>>) -> Result<(), RuntimeError> {
        let eval_condition = self.interpret_expression(condition).unwrap();
        if let Value::Bool(bool_eval_condition) = eval_condition {
            if bool_eval_condition {
                self.interpret_statement(if_stmt)?
            } else if let Some(else_stmt_defined) = else_stmt {
                self.interpret_statement(*else_stmt_defined)?
            }
        } else {
            print!("If statement cannot evaluate non boolean expression")
        }
        Ok(())
    }
    
    pub fn interpret_statement_block(&mut self, stmts: Vec<Stmt>, env: Enviroment) -> Result<(), RuntimeError> {
        self.enviroment = env;

        for stmt in stmts {
            self.interpret_statement(stmt)?;
        }
        if let Some(enclosing) = self.enviroment.enclosing.clone() {
            self.enviroment = *enclosing
        } else {
            panic!("Invalid enviroment");
        }
        Ok(())
    }

    fn interpret_statement_variable(&mut self, token: Token, expr: Expr) -> Result<(), RuntimeError>  {
        let mut value = Value::Nil;
        if expr != Expr::Literal(Literal::Nil) {
            value = self.interpret_expression(expr)?;
        }

        match &self.enviroment.enclosing {
            Some(env) => self.enviroment.put(token.lexeme, value),
            None => self.global.put(token.lexeme, value),
        } 
        Ok(())
    }
    fn interpret_statement_expression(&mut self, expr: Expr) -> Result<(), RuntimeError>  {
        let value = self.interpret_expression(expr)?;
        Ok(())
    }

    fn interpret_statement_function(&mut self, name: Token, params: Vec<Token>, code: Vec<Stmt>) -> Result<(), RuntimeError>   {
        let func = Value::LoxInstance(LoxInstance::LoxFunction(LoxFunction { stmt: Stmt::Function(name.clone(), params, code)  }));
        self.global.put(name.lexeme, func);
        Ok(())
    }
    
    fn interpret_statement_print(&mut self, expr: Expr) -> Result<(), RuntimeError> {
        //TODO better error handling
        let value = self.interpret_expression(expr)?;
        print!("{:?}\n", value);
        Ok(())
    }

    fn interpret_statement_return(&mut self, token: Token, expr: Option<Expr>) -> Result<(), RuntimeError>  {
        let mut value: Option<Value> = None;
        if let Some(expr_set) = expr {
            value = Some(self.interpret_expression(expr_set)?)
        }
        Err(RuntimeError::Return(value))
    }
    
    fn interpret_expression(&mut self, expr: Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Grouping(e) => self.interpret_expression(*e),
            Expr::Unary(o, e) => self.interpret_unary(o, *e),
            Expr::Binary(l, o, r) => self.interpret_binary(*l, o, *r),
            Expr::Literal(l) => Ok(self.interpret_literal(l)),
            Expr::Variable(t) => self.interpret_expression_variable(t),
            Expr::Assign(t, e) => self.interpret_expression_assignment(t, *e),
            Expr::Logical(l, o, r) => self.interpret_expression_logical(*l, o, *r),
            Expr::Call(c, p, a) => self.interpret_expression_call(*c, p, a)
        }
    }

    fn interpret_expression_call(&mut self, call: Expr, paren: Token, arguments: Vec<Expr>) -> Result<Value, RuntimeError> {
      
      let function_var = match call {
        Expr::Variable(token) => self.interpret_expression_variable(token),
        _ => Err(RuntimeError::Type("Attempting to call non functions and classes".to_string()))
      }?;

      let mut arguments_interpreted: Vec<Value> = vec![];

      for argument in arguments {
        arguments_interpreted.push(self.interpret_expression(argument)?);
      }

      if let Value::LoxInstance(lox_instane) = function_var {
        let LoxInstance::LoxFunction(function) = lox_instane; 
        let func_arity = function.clone().arity(); 
        if func_arity == arguments_interpreted.len() {
            return Ok(function.call_function(self, arguments_interpreted));
        } else {
            Err(RuntimeError::Function(format!("Expected {} arguments but got {}.",func_arity, arguments_interpreted.len())))
        }   
        
      } else {
        Err(RuntimeError::Type("Attempting to call functions and classes".to_string()))
      }
    
    }

    fn interpret_expression_logical(&mut self, left: Expr, operator: Token, right: Expr) -> Result<Value, RuntimeError> {
        let left = self.interpret_expression(left)?;
        if operator.token_type == TokenType::OR {
            if let Value::Bool(left_val) = left {
                if left_val {
                    return Ok(left);
                }
            }
        } else {
            if let Value::Bool(left_val) = left {
                if !left_val {
                    return Ok(left);
                }
            }
        }
        return self.interpret_expression(right);
    }

    fn interpret_expression_assignment(&mut self, token: Token, expr: Expr) -> Result<Value, RuntimeError> {
        let value = self.interpret_expression(expr)?;
        match &self.enviroment.enclosing {
            Some(env) => self.enviroment.assign(token, &value, &mut self.global)?,
            None => self.global.assign_global(token, &value)?,
        } 
        
        Ok(value)
    }
    
    fn interpret_expression_variable(&mut self, token: Token) -> Result<Value, RuntimeError> {
        return self.enviroment.get(token, &self.global);
    }
    fn interpret_literal(&mut self, literal: Literal) -> Value {
        match literal {
            Literal::False => Value::Bool(false),
            Literal::True => Value::Bool(true),
            Literal::Number(n) => Value::Number(n),
            Literal::String(s) => Value::String(s),
            Literal::Nil => Value::Nil,
        }
    }
    
    fn interpret_binary(&mut self, left: Expr, operator: Token, right: Expr) -> Result<Value, RuntimeError> {
        let value_left = self.interpret_expression(left)?;
        let value_right = self.interpret_expression(right)?;
        match (value_left, operator.token_type, value_right) {
            //Arithmetic
            (Value::Number(l), TokenType::MINUS, Value::Number(r)) => Ok(Value::Number(l-r)),
            (Value::Number(l), TokenType::SLASH, Value::Number(r)) => Ok(Value::Number(l/r)),
            (Value::Number(l), TokenType::STAR, Value::Number(r)) => Ok(Value::Number(l*r)),
            (Value::Number(l), TokenType::PLUS, Value::Number(r)) => Ok(Value::Number(l+r)),
            (Value::String(l), TokenType::PLUS, Value::String(r)) => Ok(Value::String(l.to_string() + &r)),
            //Logic
            (Value::Number(l), TokenType::GREATER, Value::Number(r)) => Ok(Value::Bool(l>r)),
            (Value::Number(l), TokenType::GREATER_EQUAL, Value::Number(r)) => Ok(Value::Bool(l>=r)),
            (Value::Number(l), TokenType::LESS, Value::Number(r)) => Ok(Value::Bool(l<r)),
            (Value::Number(l), TokenType::LESS_EQUAL, Value::Number(r)) => Ok(Value::Bool(l<=r)),
            //Equality
            (Value::Number(l), TokenType::EQUAL_EQUAL, Value::Number(r)) => Ok(Value::Bool(l==r)),
            (Value::Number(l), TokenType::BANG_EQUAL, Value::Number(r)) => Ok(Value::Bool(l!=r)),
            (Value::String(l), TokenType::EQUAL_EQUAL, Value::String(r)) => Ok(Value::Bool(l==r)),
            (Value::String(l), TokenType::BANG_EQUAL, Value::String(r)) => Ok(Value::Bool(l!=r)),
            (Value::Nil, TokenType::EQUAL_EQUAL, Value::Nil) => Ok(Value::Bool(true)),
            (Value::Nil, TokenType::BANG_EQUAL, Value::Nil) => Ok(Value::Bool(false)),
            (Value::Bool(l), TokenType::EQUAL_EQUAL, Value::Bool(r)) => Ok(Value::Bool(l==r)),
            (Value::Bool(l), TokenType::BANG_EQUAL, Value::Bool(r)) => Ok(Value::Bool(l!=r)),
            (l, m, r) => Err(RuntimeError::Type(format!(
                "invalid operands in binary expression ({:?},{:?},{:?})",
                l, m, r
            ))),
        }
    }
    
    fn interpret_unary(&mut self, operator: Token, expr: Expr) -> Result<Value, RuntimeError> {
        let value = self.interpret_expression(expr)?;
        match operator.token_type {
            TokenType::MINUS => match value {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(RuntimeError::Type(format!("Invalid application of - operator to no numeric type")))
            },
            TokenType::BANG => match value {
                Value::Bool(n) => Ok(Value::Bool(!n)),
                Value::Number(n) => Ok(Value::Bool(n == 0.0)),
                _ => Err(RuntimeError::Type(format!("Invalid application of ! opeator to non bool/numeric type")))
            }
            _ => Err(RuntimeError::Type(format!("Cannot apply unary operation to {:?}", operator.token_type)))
        }
    }
}

