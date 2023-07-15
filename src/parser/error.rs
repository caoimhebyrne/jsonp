use std::fmt;

use crate::{
    location::{LocatedError, Location},
    tokenizer::Token,
};

#[derive(Debug)]
pub enum ParserError {
    ExpectedToken(Token, Token, Location),
    UnexpectedToken(Token, Location),
    UnexpectedEOF,
}

impl LocatedError for ParserError {
    fn location(&self) -> Option<Location> {
        match self {
            Self::ExpectedToken(_, _, location) => Some(location.clone()),
            Self::UnexpectedToken(_, location) => Some(location.clone()),
            Self::UnexpectedEOF => None,
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedToken(expected, actual, _) => {
                write!(f, "Expected token: {:?}, but got {:?}", expected, actual)
            }
            Self::UnexpectedToken(token, _) => write!(f, "Unexpected {:?}", token),
            Self::UnexpectedEOF => write!(f, "Unexpected end-of-file"),
        }
    }
}
