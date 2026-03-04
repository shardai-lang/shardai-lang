// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::parser::stmt::Stmt;

pub mod lexer;
pub mod parser;
pub mod errors;
mod literal_value;

pub fn parse_source(source: String) -> Result<Vec<Stmt>, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex()?;
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    Ok(ast)
}