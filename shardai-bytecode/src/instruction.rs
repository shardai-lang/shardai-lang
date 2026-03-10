// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use std::fmt::{write, Debug, Formatter};

#[derive(Clone)]
pub struct Instruction {
    pub opcode: u8,

    pub a: u8,
    pub b: u8,
    pub c: u8
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let op_name = match self.opcode {
            0 => "LOADCONST",
            1 => "MOVE",
            _ => "UNKNOWN"
        };

        write!(f, "{} {} {} {}", op_name, self.a, self.b, self.c)
    }
}