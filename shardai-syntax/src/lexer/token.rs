use crate::lexer::LexLiteral;

#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
    pub literal: Option<LexLiteral>,

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