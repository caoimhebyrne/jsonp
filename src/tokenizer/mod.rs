pub mod token;

pub use token::*;

use crate::element_stream::ElementStream;

pub struct Tokenizer {
    element_stream: ElementStream<char>,
}

impl Tokenizer {
    pub fn new(content: String) -> Self {
        Self {
            element_stream: ElementStream::new(content.chars().collect()),
        }
    }

    pub fn process(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        loop {
            let Some(character) = self.element_stream.consume() else {
                break;
            };

            let token = match character {
                '{' => Some(Token::OpenBrace),
                '}' => Some(Token::CloseBrace),
                ':' => Some(Token::Colon),
                ',' => Some(Token::Comma),
                '"' => self.try_parse_string(),

                _ => {
                    // Try to see if it's a number
                    if character.is_numeric() {
                        self.try_parse_number(character)
                    } else {
                        None
                    }
                }
            };

            if let Some(token) = token {
                tokens.push(token);
            }
        }

        tokens
    }

    fn try_parse_string(&mut self) -> Option<Token> {
        let mut characters = vec![];

        loop {
            let Some(character) = self.element_stream.consume() else {
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

            self.element_stream.skip();
            characters.push(character);
        }

        let parsed_value = characters
            .into_iter()
            .map(|char| char.to_digit(10))
            .try_fold(0, |ans, i| i.map(|i| ans * 10 + i));

        parsed_value.map(|value| Token::Number(value))
    }
}
