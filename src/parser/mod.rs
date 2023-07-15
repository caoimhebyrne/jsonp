pub mod error;
pub mod value;

use std::collections::HashMap;

pub use error::*;
pub use value::*;

use crate::{element_stream::ElementStream, tokenizer::Token};

pub struct Parser {
    element_stream: ElementStream<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            element_stream: ElementStream::new(tokens),
        }
    }

    // Attempts to parse an object.
    // An object can have an infinite number of key value pairs.
    pub fn parse_object(&mut self) -> Result<JsonValue, ParserError> {
        // The first key in an object should always be an OpenBrace.
        self.expect_token(Token::OpenBrace)?;

        let mut map: HashMap<String, JsonValue> = HashMap::new();

        // We loop through the object until there's a closing brace, or there's no more tokens left.
        loop {
            let Some(token) = self.element_stream.consume() else {
                break;
            };

            if token == Token::CloseBrace {
                // If the last token was a comma, we need to report an error of a trailing comma.
                if self.element_stream.previous(2) == Some(Token::Comma) {
                    return Err(ParserError::UnexpectedToken(Token::Comma));
                }

                break;
            }

            let (key, value) = self.try_parse_key_value_pair(token)?;
            map.insert(key, value);
        }

        Ok(JsonValue::Object(map))
    }

    // This can be a String literal, boolean literal, number literal, object, etc.
    pub fn try_parse_value(&mut self) -> Result<JsonValue, ParserError> {
        let first_token = self.try_peek()?;

        let value = match first_token {
            Token::String(string) => {
                // This token requires no extra parsing, we can consume this token.
                self.element_stream.skip();

                JsonValue::String(string)
            }

            Token::Number(number) => {
                // This token requires no extra parsing, we can consume this token.
                self.element_stream.skip();

                JsonValue::Number(number.into())
            }

            // An open brace denotes an object
            Token::OpenBrace => self.parse_object()?,

            _ => return Err(ParserError::UnexpectedToken(first_token)),
        };

        Ok(value)
    }

    // Attempts to parse a key and value.
    // Example: "hello" -> "world",
    fn try_parse_key_value_pair(
        &mut self,
        token: Token,
    ) -> Result<(String, JsonValue), ParserError> {
        let Token::String(key) = token else {
            return Err(ParserError::UnexpectedToken(token));
        };

        // The next token should be a colon, if it isn't we've gone wrong somewhere.
        self.expect_token(Token::Colon)?;

        let value = self.try_parse_value()?;

        // The last token should be a comma, indicating that the key value pair is complete, only if the next token is a closing brace.
        let next_token = self.try_peek()?;
        if next_token == Token::Comma {
            self.element_stream.skip()
        }

        Ok((key, value))
    }

    // Expects the next token to be a certain token.
    // This will be consumed.
    fn expect_token(&mut self, token: Token) -> Result<(), ParserError> {
        let next_token = self.try_consume()?;
        if next_token != token {
            return Err(ParserError::ExpectedToken(token, next_token));
        }

        Ok(())
    }

    fn try_consume(&mut self) -> Result<Token, ParserError> {
        match self.element_stream.consume() {
            Some(token) => Ok(token),
            None => Err(ParserError::UnexpectedEOF),
        }
    }

    fn try_peek(&mut self) -> Result<Token, ParserError> {
        match self.element_stream.peek() {
            Some(token) => Ok(token),
            None => Err(ParserError::UnexpectedEOF),
        }
    }
}
