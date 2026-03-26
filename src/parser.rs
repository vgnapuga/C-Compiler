use crate::ast::{BinaryOp, Expr, Program, Type, UnaryOp};
use pest::Parser;
use pest::iterators::Pair;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct CParser;

pub fn parse_program(src: &str) -> Result<Program, pest::error::Error<Rule>> {
    let _pairs = CParser::parse(Rule::program, src)?;
    Ok(Program)
}

fn parse_base_type(pair: Pair<Rule>) -> Type {
    match pair.into_inner().next().unwrap().as_rule() {
        Rule::kw_int => Type::Int,
        Rule::kw_char => Type::Char,
        Rule::kw_bool => Type::Bool,
        Rule::kw_void => Type::Void,
        r => unreachable!("parse_base_type: {r:?}"),
    }
}

pub fn parse_type(pair: Pair<Rule>) -> Type {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::base_type => parse_base_type(inner),
        Rule::array_type => {
            let mut parts = inner.into_inner();
            let base = parse_base_type(parts.next().unwrap());
            let size = parts.next().map(|p| p.as_str().parse::<usize>().unwrap());
            Type::Array(Box::new(base), size)
        }
        r => unreachable!("parse_type: {r:?}"),
    }
}

fn parse_assign(pair: Pair<Rule>) -> Expr {
    let mut inner = pair.into_inner();
    let lhs = parse_expr(inner.next().unwrap());

    match inner.next() {
        Some(_op) => {
            let rhs = parse_expr(inner.next().unwrap());
            Expr::Binary {
                op: BinaryOp::Assign,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }
        }
        None => lhs,
    }
}

fn pair_to_binary_op(pair: &Pair<Rule>) -> BinaryOp {
    match pair.as_rule() {
        Rule::op_add => BinaryOp::Add,
        Rule::op_sub => BinaryOp::Sub,
        Rule::op_mul => BinaryOp::Mul,
        Rule::op_div => BinaryOp::Div,
        Rule::op_mod => BinaryOp::Mod,
        Rule::op_eq => BinaryOp::Eq,
        Rule::op_ne => BinaryOp::Ne,
        Rule::op_lt => BinaryOp::Lt,
        Rule::op_gt => BinaryOp::Gt,
        Rule::op_le => BinaryOp::Le,
        Rule::op_ge => BinaryOp::Ge,
        r => unreachable!("binary_op: {r:?}"),
    }
}

fn parse_char(s: &str) -> char {
    let body = &s[1..s.len() - 1];

    if body.starts_with('\\') {
        match body.chars().nth(1).unwrap() {
            'n' => '\n',
            't' => '\t',
            '0' => '\0',
            '\\' => '\\',
            '\'' => '\'',
            c => c,
        }
    } else {
        body.chars().next().unwrap()
    }
}

fn parse_primary(pair: Pair<Rule>) -> Expr {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::integer_lit => Expr::IntLit(inner.as_str().parse().unwrap()),
        Rule::char_lit => Expr::CharLit(parse_char(inner.as_str())),
        Rule::bool_lit => {
            let kw = inner.into_inner().next().unwrap();
            Expr::BoolLit(kw.as_rule() == Rule::kw_true)
        }
        Rule::expr => parse_expr(inner),
        Rule::ident => Expr::Ident(inner.as_str().to_owned()),
        r => unreachable!("primary: {r:?}"),
    }
}

fn parse_unary(pair: Pair<Rule>) -> Expr {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();

    match first.as_rule() {
        Rule::op_not => {
            let operand = parse_expr(inner.next().unwrap());
            Expr::Unary {
                op: UnaryOp::Not,
                operand: Box::new(operand),
            }
        }
        _ => parse_primary(first),
    }
}

fn parse_left_fold(pair: Pair<Rule>) -> Expr {
    let mut inner = pair.into_inner();
    let mut result = parse_expr(inner.next().unwrap());

    while let Some(op_pair) = inner.next() {
        let op = pair_to_binary_op(&op_pair);
        let rhs = parse_expr(inner.next().unwrap());
        result = Expr::Binary {
            op: op,
            lhs: Box::new(result),
            rhs: Box::new(rhs),
        };
    }

    result
}

pub fn parse_expr(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::expr => parse_expr(pair.into_inner().next().unwrap()),
        Rule::assign_expr => parse_assign(pair),
        Rule::compare_expr | Rule::additive | Rule::multiplicative => parse_left_fold(pair),
        Rule::unary => parse_unary(pair),
        Rule::primary => parse_primary(pair),
        r => unreachable!("parse_expr: {r:?}"),
    }
}
