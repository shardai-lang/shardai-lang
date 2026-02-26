use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::errors::messages::ErrorMessage;
use crate::lexer::token::Token;

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: ErrorMessage
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "at {}: {}", self.token.lexeme, self.message)
    }
}