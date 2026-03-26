use crate::ast::{BinaryOp, Expr, ForInit, Program, Stmt, Type, UnaryOp};
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

pub fn parse_block(pair: Pair<Rule>) -> Vec<Stmt> {
    pair.into_inner()
        .filter(|p| p.as_rule() == Rule::stmt)
        .map(parse_stmt)
        .collect()
}

fn parse_if(pair: Pair<Rule>) -> Stmt {
    let mut inner = pair.into_inner();
    let cond = parse_expr(inner.next().unwrap());
    let then_br = Box::new(parse_stmt(inner.next().unwrap()));
    let else_br = inner.next().map(|s| Box::new(parse_stmt(s)));

    Stmt::If {
        cond: cond,
        then_br: then_br,
        else_br: else_br,
    }
}

fn parse_while(pair: Pair<Rule>) -> Stmt {
    let mut inner = pair.into_inner();
    let cond = parse_expr(inner.next().unwrap());
    let body = Box::new(parse_stmt(inner.next().unwrap()));

    Stmt::While {
        cond: cond,
        body: body,
    }
}

fn parse_do_while(pair: Pair<Rule>) -> Stmt {
    let mut inner = pair.into_inner();
    let body = Box::new(parse_stmt(inner.next().unwrap()));
    let cond = parse_expr(inner.next().unwrap());

    Stmt::DoWhile {
        body: body,
        cond: cond,
    }
}

fn parse_for(pair: Pair<Rule>) -> Stmt {
    let mut inner = pair.into_inner();

    let init = {
        let p = inner.next().unwrap();

        match p.into_inner().next() {
            Some(e) => ForInit::Expr(parse_expr(e)),
            None => ForInit::Empty,
        }
    };
    let cund = inner.next().unwrap().into_inner().next().map(parse_expr);
    let update = inner.next().unwrap().into_inner().next().map(parse_expr);
    let body = Box::new(parse_stmt(inner.next().unwrap()));

    Stmt::For {
        init: init,
        cond: cund,
        update: update,
        body: body,
    }
}

pub fn parse_stmt(pair: Pair<Rule>) -> Stmt {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::block_stmt => Stmt::Block(parse_block(inner)),
        Rule::if_stmt => parse_if(inner),
        Rule::while_stmt => parse_while(inner),
        Rule::do_while_stmt => parse_do_while(inner),
        Rule::for_stmt => parse_for(inner),
        Rule::return_stmt => {
            let expr = inner
                .into_inner()
                .find(|p| p.as_rule() == Rule::expr)
                .map(parse_expr);

            Stmt::Return(expr)
        }
        Rule::expr_stmt => Stmt::Expr(parse_expr(inner.into_inner().next().unwrap())),
        Rule::empty_stmt => Stmt::Empty,
        r => unreachable!("stmt: {r:?}"),
    }
}
