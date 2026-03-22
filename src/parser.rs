use std::usize;

use crate::ast::{Expr, Program, Type};
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
