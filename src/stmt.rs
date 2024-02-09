use crate::{expr::Expr, scanner::Token};
#[derive(Clone, Debug, PartialEq)]

pub enum Stmt {
    Block(Vec<Stmt>),
    Class(Token, Vec<Stmt>),
    Expression(Expr),
    Function(Token, Vec<Token>, Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Print(Expr),
    Return(Token, Option<Expr>),
    Var(Token, Expr),
    While(Expr, Box<Stmt>)
}

