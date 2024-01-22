use crate::scanner;
use std::fmt::{self, Binary};

#[derive(Clone)]
pub enum Expr {
    Binary(Box<Expr>, scanner::Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(scanner::Token, Box<Expr>)
}
#[derive(Clone)]
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
            _ => write!(f,"Invalid")
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