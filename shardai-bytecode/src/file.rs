// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use crate::constant::Constant;
use crate::header::BytecodeHeader;
use crate::instruction::Instruction;
use std::io;
use std::io::Write;

#[derive(Debug)]
pub struct BytecodeFile {
    pub header: BytecodeHeader,
    pub constants: Vec<Constant>,
    pub instructions: Vec<Instruction>,
}

impl BytecodeFile {
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        self.header.write(writer)?;

        for constant in &self.constants {
            constant.write(writer)?;
        }

        for instruction in &self.instructions {
            instruction.write(writer)?;
        }

        Ok(())
    }
}
