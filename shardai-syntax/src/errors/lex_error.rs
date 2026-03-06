// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use crate::errors::messages::ErrorMessage;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct LexError {
    pub position: usize,
    pub message: ErrorMessage,
}

impl Display for LexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Lex) {}, at position {}", self.message, self.position)
    }
}

impl Error for LexError {}
