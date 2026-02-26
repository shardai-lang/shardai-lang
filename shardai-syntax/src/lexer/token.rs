use crate::literal_value::LiteralValue;

#[derive(Clone, Debug)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
    pub literal: Option<LiteralValue>,

    pub start: usize,
    pub length: usize,
}

#[derive(Clone, Debug)]
pub enum TokenType {
    Equals,
    Semicolon,
    Number,
    Var,
    Identifier,
}