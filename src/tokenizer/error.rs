use std::fmt;

use crate::location::{LocatedError, Location};

#[derive(Debug)]
pub enum TokenizerError {
    ExpectedCharacter(char, Location),
    UnexpectedCharacter(char, Location),
}

impl LocatedError for TokenizerError {
    fn location(&self) -> Option<Location> {
        match self {
            Self::ExpectedCharacter(_, location) => Some(location.clone()),
            Self::UnexpectedCharacter(_, location) => Some(location.clone()),
        }
    }
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedCharacter(expected, _) => {
                write!(f, "Expected character: {}", expected)
            }

            Self::UnexpectedCharacter(character, _) => {
                write!(f, "Unexpected character: {}", character)
            }
        }
    }
}
