// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use std::fmt::{Debug, Formatter};
use shardai_syntax::literal_value::LiteralValue;

#[derive(Clone)]
pub enum Constant {
    String(String),
    Number(f64),
    Bool(bool),
    Nil
}

impl From<LiteralValue> for Constant {
    fn from(value: LiteralValue) -> Self {
        match value {
            LiteralValue::String(s) => Constant::String(s),
            LiteralValue::Number(n) => Constant::Number(n),
            LiteralValue::Bool(b) => Constant::Bool(b),
            LiteralValue::Nil => Constant::Nil,
        }
    }
}

impl Debug for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::String(s) => write!(f, "Constant({})", s),
            Constant::Number(n) => write!(f, "Constant({})", n),
            Constant::Bool(b) => write!(f, "Constant({})", b),
            Constant::Nil => write!(f, "Constant(nil)")
        }
    }
}