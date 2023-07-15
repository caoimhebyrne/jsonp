use std::{env, fmt::Display, fs, path::Path};

use location::LocatedError;
use parser::Parser;
use tokenizer::Tokenizer;

mod element_stream;
mod location;
mod parser;
mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Usage: jsonp {{file}}");
        return;
    }

    let path = Path::new(&args[1]);
    if !path.exists() {
        eprintln!("{} doesn't exist!", args[1]);
        return;
    }

    let mut should_skip_comments = false;
    for arg in args.iter() {
        if arg == "--ignore-comments" {
            should_skip_comments = true;
            break;
        }
    }

    let json = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Failed to read {}!", args[1]);
            return;
        }
    };

    let mut tokenizer = Tokenizer::new(json.clone(), should_skip_comments);
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
