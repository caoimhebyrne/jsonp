#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    String(String),
    Number(u32),

    OpenBrace,
    CloseBrace,
    Colon,
    Comma,
}
