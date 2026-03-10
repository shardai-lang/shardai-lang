// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use std::fmt::{write, Debug, Formatter};
use crate::opcodes::Op;

#[derive(Clone)]
pub struct Instruction {
    pub opcode: Op,

    pub a: u8,
    pub b: u8,
    pub c: u8
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let op_name = match self.opcode {
            Op::LoadConst => "LOADCONST",
            Op::Move => "MOVE",
        };

        write!(f, "{} {} {} {}", op_name, self.a, self.b, self.c)
    }
}