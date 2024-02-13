use core::fmt;
use std::fmt::Display;

pub enum OpCode {
    Return,
    Constant(usize)
}

pub struct Chunk {
    pub code: Vec<(OpCode, u16)>,
    pub constant: Vec<Constant>
}

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