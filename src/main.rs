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
    name: String,
    books: Vec<Book>
}

#[derive(Debug)]
struct Library {
    name: String,
    users: Vec<User>
}

#[derive(PartialEq)]
enum Type {
    Book,
    User
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

                let mut lib: Library = Library { name: String::new(), users: Vec::new() };
                let mut type_name = Type::User;

                for pair in inner_values {
                    match pair.as_rule() {
                        Rule::name_type => {
                            let name = pair.into_inner().as_str();
                        }, // library name
                        Rule::types => {
                            type_name = match pair.into_inner().as_str() {
                                "user" => Type::User,
                                "book" => Type::Book,
                                _ => unreachable!()
                            }
                        }, // user, book
                        Rule::brackets => {
                            match type_name {
                                Type::User => {
                                    let user_name = &pair.into_inner()
                                        .into_iter()
                                        .filter(|x| x.as_rule() == Rule::name)
                                        .next().unwrap().as_str().to_string();

                                    libraries.iter_mut().next().unwrap().users.push(User {
                                        name: user_name.to_string(), 
                                        books: Vec::new()
                                    });
                                }
                                Type::Book => {
                                    let mut index: usize;
                                    let mut book_data: (String, String) = (String::new(), String::new());

                                    match pair.as_rule() {
                                        Rule::digit => index = pair.as_str().parse::<usize>().unwrap(),
                                        Rule::title => book_data.0 = pair.as_str().to_string(),
                                        Rule::author => book_data.1 = pair.as_str().to_string(),
                                        _ => unreachable!()
                                    }

                                    let index = pair.into_inner().as_str().parse::<usize>().unwrap();
                                    lib.users.iter_mut().nth(index - 1 as usize).unwrap().books.push(Book {
                                        title: book_data.0.to_string(),
                                        author: book_data.1.to_string()
                                    });

                                },
                                _ => unreachable!()
                            }
                        },
                        _ => unreachable!()
                    }


                }
            },
            Rule::print => {
                match expression.as_rule() {
                    Rule::name_type => {
                        let name = expression.into_inner().next().unwrap().as_str();
                    },
                    Rule::types => {
                        let type_name = expression.into_inner().next().unwrap().as_str();
                        if type_name != "user" {
                            panic!("Sorry, can't print book.");
                        }
                    },
                    Rule::brackets => {
                        let mut index: usize = expression.into_inner().next().unwrap().as_str().parse().unwrap();

                        println!("Book {:?}", libraries.iter().next().unwrap().users.iter().nth(index - 1 as usize).unwrap());
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
    println!("Library: {:?}", libraries.iter().next());
    println!("Library user size: {:?}", libraries.iter().next().unwrap().users.iter().count());
}