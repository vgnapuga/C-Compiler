#[derive(Debug)]
pub struct Program;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Char,
    Bool,
    Void,
    Array(Box<Type>, Option<usize>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    IntLit(i64),
    CharLit(char),
    BoolLet(bool),
    Ident(String),
}
