// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use std::collections::HashMap;
use shardai_bytecode::file::BytecodeFile;
use shardai_bytecode::header::BytecodeHeader;
use shardai_bytecode::constant::Constant;
use shardai_bytecode::instruction::Instruction;
use shardai_bytecode::opcodes::*;
use shardai_syntax::parser::expr::Expr;
use shardai_syntax::parser::stmt::Stmt;
use crate::error::CompileError;

const VERSION_MAJOR: u8 = 0;
const VERSION_MINOR: u8 = 0;

pub struct Compiler {
    next_register: u8, // This sucks really badly since we can't free registers but we'll fix it later
    locals: HashMap<String, u8>,
    constants: Vec<Constant>,
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            next_register: 0,
            locals: HashMap::new(),
            constants: Vec::new(),
            instructions: Vec::new(),
        }
    }

    fn build_header(&self) -> BytecodeHeader {
        BytecodeHeader {
            signature: *b"SBC",
            version_major: VERSION_MAJOR,
            version_minor: VERSION_MINOR,
            constant_count: self.constants.len() as u32
        }
    }

    fn add_constant(&mut self, constant: Constant) -> Result<u8, CompileError> {
        if self.constants.len() >= u8::MAX as usize {
            return Err(CompileError::TooManyConstants)
        }

        self.constants.push(constant);
        Ok((self.constants.len() - 1) as u8)
    }

    fn emit(&mut self, opcode: u8, a: u8, b: u8, c: u8) {
        let instr = Instruction { opcode, a, b, c };

        self.instructions.push(instr)
    }

    fn register_local(&mut self, name: String, register: u8) -> Result<(), CompileError> {
        self.locals.insert(name, register);

        Ok(())
    }

    fn get_local(&mut self, name: &String) -> Option<u8> {
        self.locals.get(name).cloned()
    }

    pub fn compile(&mut self, ast: Vec<Stmt>) -> Result<BytecodeFile, CompileError> {
        self.compile_ast(ast)?;

        Ok(BytecodeFile {
            header: self.build_header(),
            constants: self.constants.clone(),
            instructions: self.instructions.clone()
        })
    }

    fn compile_ast(&mut self, ast: Vec<Stmt>) -> Result<(), CompileError> {
        for stmt in ast {
            self.compile_stmt(stmt)?;
        }

        Ok(())
    }

    fn compile_stmt(&mut self, stmt: Stmt) -> Result<(), CompileError> {
        match stmt {
            Stmt::Var { name, initializer } => {
                let register = self.next_register;
                self.next_register += 1;

                if let Some(init) = initializer {
                    let value_register = self.compile_expr(init)?;

                    self.emit(OP_MOVE, register, value_register, 0);
                }

                self.register_local(name.lexeme, register)?;
                Ok(())
            }
            Stmt::Assign { target, value } => {
                let target_register = self.compile_expr(target)?;
                let value_register = self.compile_expr(value)?;

                self.emit(OP_MOVE, target_register, value_register, 0);
                Ok(())
            }
            Stmt::Expr(e) => {
                self.compile_expr(e)?;

                Ok(())
            }
        }
    }

    fn compile_expr(&mut self, expr: Expr) -> Result<u8, CompileError> {
        match expr {
            Expr::Literal(value) => {
                let const_idx = self.add_constant(value.into())?;
                let dest = self.next_register;
                self.next_register += 1;

                self.emit(OP_LOADCONST, dest, const_idx, 0);
                Ok(dest)
            },

            Expr::Identifier(token) => {
                let local = self.get_local(&token.lexeme)
                    .ok_or(CompileError::UnknownLocal(token.lexeme));

                local
            }
        }
    }
}