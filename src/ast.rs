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
    BoolLit(bool),
    Ident(String),
    Binary {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Assign,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
}
