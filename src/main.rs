use std::fs;
use pest::Parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "lib.pest"]
pub struct LIBParser;

#[derive(Debug)]
struct Book {
    title: String,
    author: String
}

#[derive(Debug)]
struct User {
    index: u16,
    name: String,
    books: Vec<Book>
}

#[derive(Debug)]
struct Library {
    name: String,
    users: Vec<User>
}

fn main() {
    // get file
    let unparsed_file = fs::read_to_string("lang_target.lib")
        .expect("cannot read file.");

    // use parser
    let file = LIBParser::parse(Rule::main, &unparsed_file)
        .expect("cannot parse file.")
        .next().unwrap();

    let mut libraries: Vec<Library> = Vec::new();

    for expression in file.into_inner() {
        match expression.as_rule() {
            Rule::init => {
                let name = expression.into_inner().as_str();
                libraries.push(Library {
                    name: name.to_string(),
                    users: Vec::new()
                })
            },

            Rule::add => {
                
            },
            Rule::print => {

            },
            _ => unreachable!()
        }
    }

    println!("library {:?}", libraries)
}