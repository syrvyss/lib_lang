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

fn parse_add_from_expr(pair: pest::iterators::Pair<Rule>) {

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
                let mut inner_values = expression.into_inner();

                let mut lib: &Library;
                let mut type_name: &str;

                for pair in inner_values {

                    match pair.as_rule() {
                        Rule::name_type => {
                            let name = expression.into_inner().as_str();
                            lib = libraries.iter().filter(|x| x.name == name).next().unwrap();
                        }, // library name
                        Rule::types => {
                            type_name = expression.into_inner().as_str();
                        }, // user, book
                        Rule::brackets => {
                            
                        },
                        _ => unreachable!()
                    }


                }
            },
            Rule::print => {
                let mut inner_values = expression.into_inner();
                match inner_values.as_rule() {
                    Rule::name_type => {},
                    Rule::types => {},
                    Rule::brackets => {},
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }

    println!("library {:?}", libraries)
}