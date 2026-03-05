// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use shardai_bytecode::constant::Constant;

#[derive(Clone, Debug)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl From<Constant> for LiteralValue {
    fn from(value: Constant) -> Self {
        match value {
            Constant::Nil => LiteralValue::Nil,
            Constant::Boolean(b) => LiteralValue::Bool(b),
            Constant::Number(n) => LiteralValue::Number(n),
            Constant::String(s) => LiteralValue::String(s),
        }
    }
}
