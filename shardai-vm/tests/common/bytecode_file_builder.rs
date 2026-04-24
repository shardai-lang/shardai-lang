// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use shardai_bytecode::constant::Constant;
use shardai_bytecode::file::BytecodeFile;
use shardai_bytecode::header::BytecodeHeader;
use shardai_bytecode::instruction::Instruction;

pub fn build(instructions: Vec<Instruction>, constants: Vec<Constant>) -> BytecodeFile {
    BytecodeFile {
        header: BytecodeHeader {
            signature: *b"SBC",
            version_major: 255,
            version_minor: 255,
            constant_count: constants.len() as u16,
            instruction_count: instructions.len() as u32,
        },

        constants,
        instructions,
    }
}
