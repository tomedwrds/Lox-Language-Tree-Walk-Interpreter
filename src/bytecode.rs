use core::fmt;
use std::{fmt::Display, ops::{Add, Div, Mul, Neg, Sub}, vec};

pub enum OpCode {
    Return,
    Constant(usize),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Equal,
    Greater,
    Less,
    Print,
    Pop,
    DefineGlobal(usize, bool),
    GetGlobal(String),
    SetGlobal(String),
    GetLocal(usize),
    SetLocal(usize),
    JumpIfFalse(usize)
}

pub struct Chunk {
    pub code: Vec<(OpCode, usize)>,
    pub constant: Vec<Value>
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            code: vec![],
            constant: vec![]
        }
    }
}

impl Chunk {
    pub fn chunk_write(&mut self, op_code: OpCode, line: usize) {
        self.code.push((op_code, line));
    }

    pub fn constant_write(&mut self, value: Value) -> usize {
        self.constant.push(value);
        return self.constant.len() - 1;
    }
}

#[derive(Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Nil
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "Nil")
        }
    }
}
