// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use crate::heap_obj::HeapObj;
use crate::value::Value;
use shardai_bytecode::constant::Constant;
use shardai_bytecode::file::BytecodeFile;
use shardai_bytecode::instruction::Instruction;
use shardai_bytecode::opcodes::Op;
use crate::runtime_error::RuntimeError;

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

    pub fn run(&mut self) -> Result<Value, RuntimeError> {
        while let Some(i) = self.instructions.get(self.pc) {
            let inst = *i;

            match inst.opcode {
                Op::LoadConst => self.load_const(inst.a, inst.b)?,
                Op::Move => self.r#move(inst.a, inst.b)?,
                Op::Add => self.add(inst.a, inst.b, inst.c)?,
                Op::Subtract => self.subtract(inst.a, inst.b, inst.c)?,
                Op::Multiply => self.multiply(inst.a, inst.b, inst.c)?,
                Op::Divide => self.divide(inst.a, inst.b, inst.c)?,
                Op::Exponentiate => self.exponentiate(inst.a, inst.b, inst.c)?,
                
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
    fn load_const(&mut self, a: u8, b: u8) -> Result<(), RuntimeError> {
        let constant = self
            .constants
            .get(b as usize)
            .ok_or(RuntimeError::IllegalOperation("invalid constant index".into()))?
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
    fn r#move(&mut self, a: u8, b: u8) -> Result<(), RuntimeError> {
        let right = self.registers[b as usize];
        self.registers[a as usize] = right;

        Ok(())
    }

    #[inline]
    fn add(&mut self, a: u8, b: u8, c: u8) -> Result<(), RuntimeError> {
        let left = self.registers[b as usize];
        let right = self.registers[c as usize];

        match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(self.registers[a as usize] = Value::Number(l + r)),
            (Value::HeapObj(l_idx), Value::HeapObj(r_idx)) => {
                let concatenated = {
                    let l = self.heap.get(l_idx)
                        .ok_or(RuntimeError::IllegalOperation("invalid heap index".into()))?;
                    let r = self.heap.get(r_idx)
                        .ok_or(RuntimeError::IllegalOperation("invalid heap index".into()))?;

                    match (l, r) {
                        (HeapObj::String(l_str), HeapObj::String(r_str)) => {
                            let mut s = String::with_capacity(l_str.len() + r_str.len());
                            s.push_str(l_str);
                            s.push_str(r_str);
                            s
                        }

                        _ => return Err(RuntimeError::InvalidOperation(format!("cannot add {} and {}", l, r))),
                    }
                };

                self.heap.push(HeapObj::String(concatenated));
                self.registers[a as usize] = Value::HeapObj(self.heap.len() - 1);
                Ok(())
            }

            _ => Err(RuntimeError::InvalidOperation(format!("cannot add {} and {}", left, right)))
        }
    }

    #[inline]
    fn subtract(&mut self, a: u8, b: u8, c: u8) -> Result<(), RuntimeError> {
        let left = self.registers[b as usize];
        let right = self.registers[c as usize];

        if let Value::Number(l) = left && let Value::Number(r) = right {
            self.registers[a as usize] = Value::Number(l - r);
            return Ok(())
        }

        Err(RuntimeError::InvalidOperation(format!("cannot subtract {} and {}", left, right)))
    }

    #[inline]
    fn multiply(&mut self, a: u8, b: u8, c: u8) -> Result<(), RuntimeError> {
        let left = self.registers[b as usize];
        let right = self.registers[c as usize];

        if let Value::Number(l) = left && let Value::Number(r) = right {
            self.registers[a as usize] = Value::Number(l * r);
            return Ok(())
        }

        Err(RuntimeError::InvalidOperation(format!("cannot multiply {} and {}", left, right)))
    }

    #[inline]
    fn divide(&mut self, a: u8, b: u8, c: u8) -> Result<(), RuntimeError> {
        let left = self.registers[b as usize];
        let right = self.registers[c as usize];

        if let Value::Number(l) = left && let Value::Number(r) = right {
            self.registers[a as usize] = Value::Number(l / r);
            return Ok(())
        }

        Err(RuntimeError::InvalidOperation(format!("cannot divide {} and {}", left, right)))
    }

    #[inline]
    fn exponentiate(&mut self, a: u8, b: u8, c: u8) -> Result<(), RuntimeError> {
        let left = self.registers[b as usize];
        let right = self.registers[c as usize];

        if let Value::Number(l) = left && let Value::Number(r) = right {
            self.registers[a as usize] = Value::Number(l.powf(r));
            return Ok(())
        }

        Err(RuntimeError::InvalidOperation(format!("cannot exponentiate {} and {}", left, right)))
    }
}
