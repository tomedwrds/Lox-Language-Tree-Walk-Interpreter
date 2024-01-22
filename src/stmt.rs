use crate::expr::Expr;
#[derive(Clone)]

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
}

