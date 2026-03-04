// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

#[derive(Clone, Debug)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil
}