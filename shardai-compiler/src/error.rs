// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CompileError {
    TooManyConstants,
    UnknownLocal(String)
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::TooManyConstants => write!(f, "Too many constants"),
            CompileError::UnknownLocal(s) => write!(f, "Unknown local {}", s)
        }
    }
}

impl Error for CompileError {}