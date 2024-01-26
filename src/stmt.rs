use crate::{expr::Expr, scanner::Token};
#[derive(Clone)]

pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    Print(Expr),
    Var(Token, Expr)
}

