// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use crate::lexer::token::Token;
use crate::parser::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    Assign {
        target: Expr,
        value: Expr,
    },
    Return {
        return_value: Option<Expr>,
    },
    If {
        condition: Expr,
        if_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>
    }
}
