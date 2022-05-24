extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "lib.pest"]
pub struct CSVParser;

fn main() {
    println!("Hello, world!");
}
