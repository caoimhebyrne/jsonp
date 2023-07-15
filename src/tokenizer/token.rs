use crate::location::Location;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    String(String),
    Number(u32),

    OpenBrace,
    CloseBrace,
    OpenSquareBracket,
    CloseSquareBracket,
    Colon,
    Comma,
}

pub type TokenAndLocation = (Token, Location);
