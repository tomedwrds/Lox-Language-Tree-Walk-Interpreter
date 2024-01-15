use crate::scanner;


pub enum Expr {
    Binary(Box<Expr>, scanner::Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(scanner::Token, Box<Expr>)
}

pub enum Literal {
    True,
    False,
    Nil,
    Number(f64),
    String(String)
}