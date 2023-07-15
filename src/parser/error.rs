use std::fmt;

use crate::tokenizer::Token;

#[derive(Debug)]
pub enum ParserError {
    ExpectedToken(Token, Token),
    UnexpectedToken(Token),
    UnexpectedEOF,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedToken(expected, actual) => {
                write!(f, "Expected token: {:?}, but got {:?}", expected, actual)
            }
            Self::UnexpectedToken(token) => write!(f, "Unexpected token: {:?}", token),
            Self::UnexpectedEOF => write!(f, "Unexpected end-of-file"),
        }
    }
}
