use crate::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    String(String),
    Identifier(String),
    Number(f64),

    OpenBrace,
    CloseBrace,
    OpenSquareBracket,
    CloseSquareBracket,
    Colon,
    Comma,
}

pub type TokenAndLocation = (Token, Location);
