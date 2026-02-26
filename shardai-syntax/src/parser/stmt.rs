use crate::lexer::token::Token;
use crate::parser::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Var {
        name: Token,
        initializer: Option<Expr>
    }
}