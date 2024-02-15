use core::fmt;
use std::{fmt::Display, ops::{Add, Div, Mul, Neg, Sub}};

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
    pub code: Vec<(OpCode, u16)>,
    pub constant: Vec<Constant>
}

#[derive(Clone)]
pub enum Constant {
    Number(f64)
}

impl Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Constant::Number(n) => write!(f, "{}", n),
        }
    }
}

impl Neg for Constant {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Number(n) => Self::Number(-n)
        }
    }
}

impl Add for Constant {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Self::Number(n1) => match other {
                Self::Number(n2) => return Self::Number(n1 + n2)
            }
        }
    }
}

impl Sub for Constant {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Self::Number(n1) => match other {
                Self::Number(n2) => return Self::Number(n1 - n2)
            }
        }
    }
}

impl Div for Constant {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match self {
            Self::Number(n1) => match other {
                Self::Number(n2) => return Self::Number(n1 / n2)
            }
        }
    }
}

impl Mul for Constant {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match self {
            Self::Number(n1) => match other {
                Self::Number(n2) => return Self::Number(n1 * n2)
            }
        }
    }
}