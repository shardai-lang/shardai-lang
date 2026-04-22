// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use shardai_bytecode::constant::Constant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    HeapObj(usize),
    Bool(bool),
    Number(f64),
    Nil,
}

impl From<Constant> for Value {
    fn from(value: Constant) -> Self {
        match value {
            Constant::Number(n) => Value::Number(n),
            Constant::Bool(b) => Value::Bool(b),
            Constant::Nil => Value::Nil,
            Constant::String(_) => panic!("String is incompatible with Value, put it in the heap"),
        }
    }
}
