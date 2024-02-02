use std::collections::HashMap;

use crate::{enviroment::{create_enviroment, Enviroment}, expr::{Expr, Literal}, scanner::{Token, TokenType}, stmt::Stmt};
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

#[derive(Debug)]
pub enum RuntimeError {
    Variable(Token, String),
    Type(String)
}
mod tests;

#[derive(Debug)]
struct Interpreter {
    enviroment: Enviroment,
}

pub fn interpret(statements: Vec<Stmt>) {
    let mut interpreter = Interpreter {
        enviroment: create_enviroment(None)
    };
    interpreter.interpret(statements);
}

impl Interpreter {
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements.iter() {
            self.interpret_statement(statement.clone());
        }
    }

    fn interpret_statement(&mut self, stmt: Stmt)  {
        match stmt {
            Stmt::Block(ve) => self.interpret_statement_block(ve, create_enviroment(Some(self.enviroment.clone()))),
            Stmt::If(c,i ,e) => self.intepret_statement_if(c, *i, e),
            Stmt::Expression(e) => self.interpret_statement_expression(e),
            Stmt::Print(e) => self.interpret_statement_print(e),
            Stmt::Var(t, e) => self.interpret_statement_variable(t, e),
            Stmt::While(e, s) => self.interpret_statement_while(e, *s),
        }
    }

    fn interpret_statement_while(&mut self, condition: Expr, stmt: Stmt) {
        while self.is_truth(condition.clone()) {
            self.interpret_statement(stmt.clone())
        } 
    }

    fn is_truth(&mut self, expr: Expr) -> bool {
        let value = self.interpret_expression(expr).unwrap();
        if let Value::Bool(bool_expr) = value {
            return bool_expr
        } else {
            return false
        }
    }
    fn intepret_statement_if(&mut self, condition: Expr, if_stmt: Stmt, else_stmt: Option<Box<Stmt>>) {
        let eval_condition = self.interpret_expression(condition).unwrap();
        if let Value::Bool(bool_eval_condition) = eval_condition {
            if bool_eval_condition {
                self.interpret_statement(if_stmt)
            } else if let Some(else_stmt_defined) = else_stmt {
                self.interpret_statement(*else_stmt_defined)
            }
        } else {
            print!("If statement cannot evaluate non boolean expression")
        }
    }
    
    fn interpret_statement_block(&mut self, stmts: Vec<Stmt>, env: Enviroment) {
        self.enviroment = env;

        for stmt in stmts {
            self.interpret_statement(stmt);
        }
        if let Some(enclosing) = self.enviroment.enclosing.clone() {
            self.enviroment = *enclosing
        } else {
            panic!("Invalid enviroment");
        }
    }

    fn interpret_statement_variable(&mut self, token: Token, expr: Expr)  {
        let mut value = Value::Nil;
        if expr != Expr::Literal(Literal::Nil) {
            value = self.interpret_expression(expr).unwrap();
        }

        self.enviroment.put(token.lexeme, value);
    }
    
    fn interpret_statement_expression(&mut self, expr: Expr)  {
        let value = self.interpret_expression(expr);
    }
    
    fn interpret_statement_print(&mut self, expr: Expr)  {
        ///TODO better error handling
        let value = self.interpret_expression(expr).unwrap();
        print!("{:?}\n", value);
    }
    
    fn interpret_expression(&mut self, expr: Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Grouping(e) => self.interpret_expression(*e),
            Expr::Unary(o, e) => self.interpret_unary(o, *e),
            Expr::Binary(l, o, r) => self.interpret_binary(*l, o, *r),
            Expr::Literal(l) => Ok(self.interpret_literal(l)),
            Expr::Variable(t) => self.interpret_expression_variable(t),
            Expr::Assign(t, e) => self.interpret_expression_assignment(t, *e),
            Expr::Logical(l, o, r) => self.interpret_expression_logical(*l, o, *r)
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
        self.enviroment.assign(token, &value)?;
        Ok(value)
    }
    
    fn interpret_expression_variable(&mut self, token: Token) -> Result<Value, RuntimeError> {
        return self.enviroment.get(token);
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

