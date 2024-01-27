use crate::{expr::Expr, scanner::Token};
#[derive(Clone)]

pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Print(Expr),
    Var(Token, Expr)
}

