use crate::literal_value::LiteralValue;

#[derive(Debug)]
pub enum Expr {
    Literal(LiteralValue)
}