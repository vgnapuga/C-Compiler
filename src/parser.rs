use crate::ast::Program;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct CParser;

pub fn parse_program(src: &str) -> Result<Program, pest::error::Error<Rule>> {
    let _pairs = CParser::parse(Rule::program, src)?;
    Ok(Program)
}
