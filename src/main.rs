use std::{fmt::Display, fs};

use location::LocatedError;
use parser::Parser;
use tokenizer::Tokenizer;

mod element_stream;
mod location;
mod parser;
mod tokenizer;

fn main() {
    let json = fs::read_to_string("examples/simple.json").unwrap();

    let mut tokenizer = Tokenizer::new(json.clone());
    let tokens = match tokenizer.process() {
        Ok(tokens) => tokens,
        Err(error) => return print_error(json, error),
    };

    let mut parser = Parser::new(tokens);
    let result = parser.parse_object();
    match result {
        Ok(object) => println!("{:#?}", object),
        Err(error) => print_error(json, error),
    }
}

fn print_error<T: LocatedError + Display>(json: String, error: T) {
    let optional_location = error.location();
    if let Some(location) = optional_location {
        println!(
            "Error at line {} column {}:",
            location.line + 1,
            location.column
        );

        println!("{}", json.lines().nth(location.line).unwrap());
        println!("{}^", " ".repeat(location.column - 1));
        println!("{}{}", " ".repeat(location.column - 1), error);
    } else {
        println!("Error: {}", error);
    }
}
