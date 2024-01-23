use crate::{expr::Expr, scanner::Token};
#[derive(Clone)]

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Expr)
}

