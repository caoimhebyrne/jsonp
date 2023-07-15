pub mod error;
pub mod token;

pub use error::*;
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

    pub fn process(&mut self) -> Result<Vec<TokenAndLocation>, TokenizerError> {
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
                '[' => Some(Token::OpenSquareBracket),
                ']' => Some(Token::CloseSquareBracket),

                '"' => self.try_parse_string()?.into(),

                '/' => {
                    self.skip_comment();
                    continue;
                }

                ' ' => continue,

                '\n' => {
                    self.line += 1;
                    self.column = 0;
                    None
                }

                _ => {
                    if Tokenizer::is_json_number(character) {
                        self.try_parse_number(character)
                    } else if character.is_alphanumeric() {
                        self.try_parse_identifier(character)?.into()
                    } else {
                        return Err(TokenizerError::UnexpectedCharacter(
                            character,
                            self.location(),
                        ));
                    }
                }
            };

            if let Some(token) = token {
                tokens.push((token, self.location()));
            }
        }

        Ok(tokens)
    }

    fn location(&self) -> Location {
        Location {
            line: self.line,
            column: self.column,
        }
    }

    fn skip_comment(&mut self) {
        loop {
            let Some(character) = self.element_stream.peek() else {
                break;
            };

            if character == '\n' {
                break;
            }

            self.element_stream.skip()
        }
    }

    fn try_parse_string(&mut self) -> Result<Token, TokenizerError> {
        let mut characters = vec![];

        loop {
            let Some(character) = self.consume() else {
                break;
            };

            if character == '"' {
                break;
            }

            if character == '\n' {
                return Err(TokenizerError::ExpectedCharacter('"', self.location()));
            }

            characters.push(character);
        }

        let string = characters.into_iter().collect();
        Ok(Token::String(string))
    }

    fn try_parse_identifier(&mut self, character: char) -> Result<Token, TokenizerError> {
        let mut characters = vec![character];

        loop {
            let Some(character) = self.element_stream.peek() else {
                break;
            };

            if !character.is_alphanumeric() {
                break;
            }

            if character == '\n' {
                return Err(TokenizerError::UnexpectedCharacter(' ', self.location()));
            }

            self.skip();
            characters.push(character);
        }

        let string = characters.into_iter().collect();
        Ok(Token::Identifier(string))
    }

    fn try_parse_number(&mut self, character: char) -> Option<Token> {
        let mut characters = vec![character];

        loop {
            let Some(character) = self.element_stream.peek() else {
                break;
            };

            if !Tokenizer::is_json_number(character) {
                break;
            }

            self.skip();
            characters.push(character);
        }

        let number_string: String = characters.into_iter().collect();
        match number_string.parse::<f64>() {
            Ok(value) => Some(Token::Number(value)),
            Err(_) => None,
        }
    }

    fn consume(&mut self) -> Option<char> {
        self.column += 1;
        self.element_stream.consume()
    }

    fn skip(&mut self) {
        self.column += 1;
        self.element_stream.skip()
    }

    fn is_json_number(character: char) -> bool {
        return character == '-'
            || character == '.'
            || character == 'e'
            || character == 'E'
            || character.is_numeric();
    }
}
