// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use crate::lexer::token::Token;
use crate::literal_value::LiteralValue;

#[derive(Debug)]
pub enum Expr {
    Literal(LiteralValue),
    Identifier(Token),
    Add { left: Box<Expr>, right: Box<Expr> },
    Subtract { left: Box<Expr>, right: Box<Expr> },
    Multiply { left: Box<Expr>, right: Box<Expr> },
    Divide { left: Box<Expr>, right: Box<Expr> },
    Exponentiation { left: Box<Expr>, right: Box<Expr> },

    Or { left: Box<Expr>, right: Box<Expr> },
    And { left: Box<Expr>, right: Box<Expr> },
    Equals { left: Box<Expr>, right: Box<Expr> },
    NotEquals { left: Box<Expr>, right: Box<Expr> },
    GreaterThan { left: Box<Expr>, right: Box<Expr> },
    GreaterEqualThan { left: Box<Expr>, right: Box<Expr> },
    LessThan { left: Box<Expr>, right: Box<Expr> },
    LessEqualThan { left: Box<Expr>, right: Box<Expr> },
    Not { operand: Box<Expr> },
    Negate { operand: Box<Expr> },
}
