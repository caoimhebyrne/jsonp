#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

pub trait LocatedError {
    fn location(&self) -> Option<Location>;
}
