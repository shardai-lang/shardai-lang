use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::errors::messages::ErrorMessage;

#[derive(Debug)]
pub struct LexError {
    pub line: usize,
    pub message: ErrorMessage
}

impl Display for LexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] {}", self.line, self.message)
    }
}

impl Error for LexError {}