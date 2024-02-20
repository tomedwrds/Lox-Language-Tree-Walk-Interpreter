use core::fmt;
use std::{fmt::Display, ops::{Add, Div, Mul, Neg, Sub}, vec};

pub enum OpCode {
    Return,
    Constant(usize),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide
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

#[derive(Clone)]
pub enum Value {
    Number(f64)
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Number(n) => Self::Number(-n)
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Self::Number(n1) => match other {
                Self::Number(n2) => return Self::Number(n1 + n2)
            }
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Self::Number(n1) => match other {
                Self::Number(n2) => return Self::Number(n1 - n2)
            }
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match self {
            Self::Number(n1) => match other {
                Self::Number(n2) => return Self::Number(n1 / n2)
            }
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match self {
            Self::Number(n1) => match other {
                Self::Number(n2) => return Self::Number(n1 * n2)
            }
        }
    }
}