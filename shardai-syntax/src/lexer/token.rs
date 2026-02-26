use crate::literal_value::LiteralValue;

#[derive(Clone, Debug)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
    pub literal: Option<LiteralValue>,

    pub start: usize,
    pub length: usize,
}

impl Token {
    pub fn eof(pos: usize) -> Self {
        Self {
            literal: None,
            lexeme: "".into(),
            token_type: TokenType::Eof,
            start: pos,
            length: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Equals,
    Semicolon,
    Number,
    Var,
    Identifier,
    Eof,
}