use crate::scanner::Token;
use std::fmt::{self};

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
    Set(Box<Expr>, Token, Box<Expr>),
    Get(Box<Expr>, Token),
    Grouping(Box<Expr>),
    Literal(Literal),
    Logical(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Variable(Token),
    Assign(Token, Box<Expr>)
}
#[derive(Clone, PartialEq, Debug)]
pub enum Literal {
    True,
    False,
    Nil,
    Number(f64),
    String(String)
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::True => write!(
                f,
                "True",
            ),
            Literal::False => write!(
                f,
                "False",
            ),
            Literal::Nil => write!(
                f,
                "Nil",
            ),
            Literal::Number(n) => write!(
                f,
                "{}", n
            ),
            Literal::String(s) => write!(
                f,
                "{}", s
            ),
        }
        
    }
  }

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Binary(R, O, L) => write!(
                f,
                "Binary({} {} {}) ", R, O, L
            ),
            Expr::Grouping(M) => write!(
                f,
                "Grouping({}) ", M
            ),
            Expr::Literal(l) => write!(
                f,
                "Literal({}) ", l
            ),
            Expr::Unary(R, O) => write!(
                f,
                "Unary({} {}) ", R, O
            ),
            _ => write!(f,"Invalid")
        }
        
    }
  }