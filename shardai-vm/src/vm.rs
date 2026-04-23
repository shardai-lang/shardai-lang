// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use crate::heap_obj::HeapObj;
use crate::value::Value;
use shardai_bytecode::constant::Constant;
use shardai_bytecode::file::BytecodeFile;
use shardai_bytecode::instruction::Instruction;
use shardai_bytecode::opcodes::Op;

pub struct VM {
    instructions: Vec<Instruction>,
    registers: Vec<Value>,
    constants: Vec<Constant>,
    heap: Vec<HeapObj>,
    pc: usize,
}

impl VM {
    pub fn new(bytecode_file: BytecodeFile) -> Self {
        let instructions = bytecode_file.instructions;
        let constants = bytecode_file.constants;
        let registers = vec![Value::Void; 256];
        let heap = Vec::new();

        Self {
            instructions,
            registers,
            constants,
            heap,
            pc: 0,
        }
    }

    pub fn run(&mut self) -> Result<Value, &'static str> {
        while let Some(i) = self.instructions.get(self.pc) {
            let inst = *i;

            match inst.opcode {
                Op::LoadConst => self.load_const(inst.a, inst.b)?,
                Op::Move => self.r#move(inst.a, inst.b)?,
                
                Op::Return => return Ok(self.registers[inst.a as usize]),
                Op::ReturnVoid => return Ok(Value::Void),

                _ => unimplemented!()
            }

            self.pc += 1;
        }
        
        Ok(Value::Void)
    }

    // Opcode handlers

    #[inline]
    fn load_const(&mut self, a: u8, b: u8) -> Result<(), &'static str> {
        let constant = self
            .constants
            .get(b as usize)
            .ok_or("Illegal operation: invalid constant index")?
            .clone();

        let register_value = if let Constant::String(s) = constant {
            self.heap.push(HeapObj::String(s));

            Value::HeapObj(self.heap.len() - 1)
        } else {
            Value::from(constant)
        };

        self.registers[a as usize] = register_value;

        Ok(())
    }

    #[inline]
    fn r#move(&mut self, a: u8, b: u8) -> Result<(), &'static str> {
        let right = self.registers[b as usize];
        self.registers[a as usize] = right;

        Ok(())
    }
}
