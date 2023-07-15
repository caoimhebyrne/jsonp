pub mod token;

pub use token::*;

use crate::{element_stream::ElementStream, location::Location};

pub struct Tokenizer {
    element_stream: ElementStream<char>,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(content: String) -> Self {
        Self {
            element_stream: ElementStream::new(content.replace("\r\n", "\n").chars().collect()),
            line: 0,
            column: 0,
        }
    }

    pub fn process(&mut self) -> Vec<TokenAndLocation> {
        let mut tokens = vec![];

        loop {
            let Some(character) = self.consume() else {
                break;
            };

            let token = match character {
                '{' => Some(Token::OpenBrace),
                '}' => Some(Token::CloseBrace),
                ':' => Some(Token::Colon),
                ',' => Some(Token::Comma),
                '"' => self.try_parse_string(),

                '\n' => {
                    self.line += 1;
                    self.column = 0;
                    None
                }

                _ => {
                    // Try to see if it's a number
                    if character.is_numeric() {
                        self.try_parse_number(character)
                    } else if character == ' ' {
                        None
                    } else {
                        // TODO: Error handling in the tokenizer 💯

                        println!("Unknown character: {}", character);
                        None
                    }
                }
            };

            if let Some(token) = token {
                tokens.push((
                    token,
                    Location {
                        line: self.line,
                        column: self.column,
                    },
                ));
            }
        }

        tokens
    }

    fn try_parse_string(&mut self) -> Option<Token> {
        let mut characters = vec![];

        loop {
            let Some(character) = self.consume() else {
                break;
            };

            if character == '"' {
                break;
            }

            characters.push(character);
        }

        let string = characters.into_iter().collect();
        Some(Token::String(string))
    }

    fn try_parse_number(&mut self, character: char) -> Option<Token> {
        let mut characters = vec![character];

        loop {
            let Some(character) = self.element_stream.peek() else {
                break;
            };

            if !character.is_alphanumeric() {
                break;
            }

            self.skip();
            characters.push(character);
        }

        let parsed_value = characters
            .into_iter()
            .map(|char| char.to_digit(10))
            .try_fold(0, |ans, i| i.map(|i| ans * 10 + i));

        parsed_value.map(|value| Token::Number(value))
    }

    fn consume(&mut self) -> Option<char> {
        self.column += 1;
        self.element_stream.consume()
    }

    fn skip(&mut self) {
        self.column += 1;
        self.element_stream.skip()
    }
}
