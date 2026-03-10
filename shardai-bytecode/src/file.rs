// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use crate::constant::Constant;
use crate::header::BytecodeHeader;
use crate::instruction::Instruction;

#[derive(Debug)]
pub struct BytecodeFile {
    pub header: BytecodeHeader,
    pub constants: Vec<Constant>,
    pub instructions: Vec<Instruction>
}