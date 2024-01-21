use crate::{expr::{Expr, Literal}, scanner::{Token, TokenType}};
#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}
mod tests;



pub fn interpret(expr: Expr) -> Result<Value, String> {
    match expr {
        Expr::Grouping(e) => interpret(*e),
        Expr::Unary(o, e) => interpret_unary(o, *e),
        Expr::Binary(l, o, r) => interpret_binary(*l, o, *r),
        Expr::Literal(l) => Ok(interpret_literal(l))
    }
}

fn interpret_literal(literal: Literal) -> Value {
    match literal {
        Literal::False => Value::Bool(false),
        Literal::True => Value::Bool(true),
        Literal::Number(n) => Value::Number(n),
        Literal::String(s) => Value::String(s),
        Literal::Nil => Value::Nil,
    }
}

fn interpret_binary(left: Expr, operator: Token, right: Expr) -> Result<Value, String> {
    let value_left = interpret(left)?;
    let value_right = interpret(right)?;
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
        (l, m, r) => Err(format!(
            "invalid operands in binary expression ({:?},{:?},{:?})",
            l, m, r
        )),
    }
}

fn interpret_unary(operator: Token, expr: Expr) -> Result<Value, String> {
    let value = interpret(expr)?;
    match operator.token_type {
        TokenType::MINUS => match value {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(format!("Invalid application of - operator to no numeric type"))
        },
        TokenType::BANG => match value {
            Value::Bool(n) => Ok(Value::Bool(!n)),
            Value::Number(n) => Ok(Value::Bool(n == 0.0)),
            _ => Err(format!("Invalid application of ! opeator to non bool/numeric type"))
        }
        _ => Err(format!("Cannot apply unary operation to {:?}", operator.token_type))
    }
}