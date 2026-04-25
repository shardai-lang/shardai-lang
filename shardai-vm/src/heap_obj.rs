// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq)]
pub enum HeapObj {
    String(String),
}

impl Debug for HeapObj {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HeapObj::String(_) => write!(f, "string"),
        }
    }
}

impl Display for HeapObj {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HeapObj::String(s) => write!(f, "{}", s),
        }
    }
}
