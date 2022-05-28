use std::fs;
use pest::Parser;
use pest::error::Error;
use pest::iterators::Pairs;

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod cli;

#[derive(Parser)]
#[grammar = "lib.pest"]
pub struct LIBParser;

#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    books: Vec<Book>
}

#[derive(Debug, Clone)]
struct Library {
    name: String,
    users: Vec<User>
}

impl Library {
    fn output(&self) {
        println!("------------------");
        println!("Users in library: ");
            &self.users.iter().for_each(|x| println!("{:?}", x.name));
        println!("Books in library: ");
            for i in self.users.iter() {
                i.books.iter()
                    .for_each(|x| println!("Loaner: {:?}, {:?}, {:?}", i.name, x.title, x.author));
            }
        println!("Structure: {:?}", &self);
        println!("------------------");
    }
}

#[derive(PartialEq)]
enum Type {
    Book,
    User
}

fn parser(file: Pairs<Rule>) -> Result<Library, Error<Rule>> {
    let mut libraries: Vec<Library> = Vec::new();

    for expression in file {
        for test in expression.into_inner() {
            match test.as_rule() {
                Rule::init => {
                    let name = test.into_inner().as_str();
                    libraries.push(Library {
                        name: name.to_string(),
                        users: Vec::new()
                    })
                },
    
                Rule::add => {
                    let inner_values = test.into_inner();
    
                    let mut type_name = Type::User;
    
                    for pair in inner_values {
                        match pair.as_rule() {
                            Rule::name_type => {
                            }, // library name
                            Rule::types => {
                                type_name = match pair.as_str() {
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
                                        let mut index = 1usize;
                                        let mut book_data: (String, String) = (String::new(), String::new());

                                        for item in pair.into_inner() {
                                            match item.as_rule() {
                                                Rule::index => index = item.into_inner().as_str().parse::<usize>().unwrap(),
                                                Rule::title => book_data.0 = item.as_str().to_string(),
                                                Rule::author => book_data.1 = item.as_str().to_string(),
                                                _ => unreachable!()
                                            }
                                        }

                                        libraries.iter_mut().next().unwrap().users.iter_mut().nth(index - 1usize).unwrap().books.push(Book {
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
                    for item in test.into_inner() {
                        let mut name = String::new();
                        let mut index = 0usize;
                        match item.as_rule() {
                            Rule::name_type => {},
                            Rule::types => {
                                let type_name = item.as_str();
                                if type_name != "user" {
                                    panic!("Sorry, can't print book.");
                                }
                            },
                            Rule::brackets => {
                                index = item.into_inner()
                                    .as_str()
                                    .replace("\n", "")
                                    .split(" ")
                                    .last().unwrap().parse().unwrap();
                            },
                            _ => unreachable!()
                        }
                        println!("User at index: {}, Name: {}, Books: {}", index+1, libraries.iter().next().unwrap().users[index].name, index);
                    }
                },
                _ => unreachable!()
            }
        }
    }

    Ok(libraries.iter().next().unwrap().clone())
}

fn main() {
    let path = cli::get_path();

    // get file
    let unparsed_file = fs::read_to_string(&path.1)
        .expect("cannot read file.");

    // use parser
    let file = LIBParser::parse(Rule::main, &unparsed_file)
        .expect("cannot parse file.");

    match path.0 {
        true => {
            let library = parser(file).unwrap();
            library.output();
        },
        false => {
            println!("Built file to {:?}", path.1)
        }
    }
}