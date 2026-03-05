// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use crate::literal_value::LiteralValue;

#[derive(Debug)]
pub enum Expr {
    Literal(LiteralValue),
}
