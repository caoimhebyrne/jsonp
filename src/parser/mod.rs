pub mod error;
pub mod value;

use std::collections::HashMap;

pub use error::*;
pub use value::*;

use crate::{
    element_stream::ElementStream,
    location::Location,
    tokenizer::{Token, TokenAndLocation},
};

pub struct Parser {
    element_stream: ElementStream<TokenAndLocation>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenAndLocation>) -> Self {
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
            let Some((token, location)) = self.element_stream.consume() else {
                break;
            };

            if token == Token::CloseBrace {
                // If the last token was a comma, we need to report an error of a trailing comma.
                self.check_for_trailing_comma()?;
                break;
            }

            let (key, value) = self.try_parse_key_value_pair(token, location)?;
            map.insert(key, value);
        }

        Ok(JsonValue::Object(map))
    }

    pub fn try_parse_array(&mut self) -> Result<JsonValue, ParserError> {
        // The first key in an object should always be a OpenSquareBracket.
        self.expect_token(Token::OpenSquareBracket)?;

        let mut values = vec![];

        loop {
            // We need to check if the next token is a closing bracket, if it is, then we are finished parsing.
            match self.element_stream.peek() {
                Some((token, _)) => {
                    if token == Token::CloseSquareBracket {
                        self.element_stream.skip();
                        self.check_for_trailing_comma()?;
                        break;
                    }
                }

                None => return Err(ParserError::UnexpectedEOF),
            }

            let value = self.try_parse_value()?;
            values.push(value);

            let (next_token, location) = self.try_consume()?;
            match next_token {
                Token::Comma => {}
                Token::CloseSquareBracket => break,
                _ => return Err(ParserError::UnexpectedToken(next_token, location)),
            }
        }

        Ok(JsonValue::Array(values))
    }

    // This can be a String literal, boolean literal, number literal, object, etc.
    pub fn try_parse_value(&mut self) -> Result<JsonValue, ParserError> {
        let (first_token, location) = self.try_peek()?;

        let value = match first_token.clone() {
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

            Token::Identifier(identifier) => {
                // This token requires no extra parsing, we can consume this token.
                self.element_stream.skip();

                match identifier.as_str() {
                    "true" => JsonValue::Boolean(true),
                    "false" => JsonValue::Boolean(false),
                    "null" => JsonValue::Null,

                    _ => return Err(ParserError::UnexpectedToken(first_token, location)),
                }
            }

            // An open square bracket denotes an array
            Token::OpenSquareBracket => self.try_parse_array()?,

            // An open brace denotes an object
            Token::OpenBrace => self.parse_object()?,

            _ => return Err(ParserError::UnexpectedToken(first_token, location)),
        };

        Ok(value)
    }

    // Attempts to parse a key and value.
    // Example: "hello" -> "world",
    fn try_parse_key_value_pair(
        &mut self,
        token: Token,
        location: Location,
    ) -> Result<(String, JsonValue), ParserError> {
        let Token::String(key) = token else {
            return Err(ParserError::UnexpectedToken(token, location));
        };

        // The next token should be a colon, if it isn't we've gone wrong somewhere.
        self.expect_token(Token::Colon)?;

        let value = self.try_parse_value()?;

        // The last token should be a comma, indicating that the key value pair is complete, only if the next token is a closing brace.
        let (next_token, _) = self.try_peek()?;
        if next_token == Token::Comma {
            self.element_stream.skip()
        }

        Ok((key, value))
    }

    // Expects the next token to be a certain token.
    // This will be consumed.
    fn expect_token(&mut self, token: Token) -> Result<(), ParserError> {
        let (next_token, location) = self.try_consume()?;
        if next_token != token {
            return Err(ParserError::ExpectedToken(token, next_token, location));
        }

        Ok(())
    }

    // Checks if the last token is a trailing comma.
    fn check_for_trailing_comma(&mut self) -> Result<(), ParserError> {
        let previous = self.element_stream.previous(2);
        let Some((token, location)) = previous else {
            return Ok(());
        };

        if token == Token::Comma {
            return Err(ParserError::UnexpectedToken(token, location));
        } else {
            Ok(())
        }
    }

    fn try_consume(&mut self) -> Result<TokenAndLocation, ParserError> {
        match self.element_stream.consume() {
            Some(value) => Ok(value),
            None => Err(ParserError::UnexpectedEOF),
        }
    }

    fn try_peek(&mut self) -> Result<TokenAndLocation, ParserError> {
        match self.element_stream.peek() {
            Some(value) => Ok(value),
            None => Err(ParserError::UnexpectedEOF),
        }
    }
}
