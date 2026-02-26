use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum ErrorMessage {
    UnexpectedChar(char),
    UnexpectedEof,
    ExpectedIdentifier(&'static str),
    MalformedNumber(String),
    ExpectedChar(char),
    ExpectedExpression,
}

impl Display for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedChar(c) => write!(f, "Unexpected character '{}'", c),
            Self::ExpectedIdentifier(s) => write!(f, "Expected identifier after '{}'", s),
            Self::UnexpectedEof => write!(f, "Unexpected EOF"),
            Self::MalformedNumber(s) => write!(f, "Malformed number '{}'", s),
            Self::ExpectedChar(c) => write!(f, "Expected character '{}'", c),
            Self::ExpectedExpression => write!(f, "Expected expression")
        }
    }
}