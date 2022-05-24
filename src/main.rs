use std::fs;
use pest::Parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "lib.pest"]
pub struct LIBParser;

fn main() {
    // get file
    let unparsed_file = fs::read_to_string("lang_target.lib")
        .expect("cannot read file.");

    // use parser
    let file = LIBParser::parse(Rule::main, &unparsed_file)
        .expect("cannot parse file.")
        .next().unwrap();

    for expression in file.into_inner() {
        match expression.as_rule() {
            Rule::
        }
    }
}
