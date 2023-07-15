use std::fs;

use parser::Parser;
use tokenizer::Tokenizer;

mod element_stream;
mod parser;
mod tokenizer;

fn main() {
    let json = fs::read_to_string("examples/simple.json").unwrap();

    let mut tokenizer = Tokenizer::new(json);
    let tokens = tokenizer.process();

    let mut parser = Parser::new(tokens);
    let result = parser.parse_object();
    match result {
        Ok(object) => println!("{:#?}", object),
        Err(error) => println!("Error: {}", error),
    }
}
