// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use std::fmt::{write, Debug, Formatter};
use std::io;
use std::io::Write;
use crate::opcodes::Op;

#[derive(Clone)]
pub struct Instruction {
    pub opcode: Op,

    pub a: u8,
    pub b: u8,
    pub c: u8
}

impl Instruction {
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&[self.opcode as u8])?;
        writer.write_all(&[self.a, self.b, self.c])?;

        Ok(())
    }
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