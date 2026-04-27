// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use crate::error::CompileError;
use shardai_bytecode::constant::Constant;
use shardai_bytecode::file::BytecodeFile;
use shardai_bytecode::header::BytecodeHeader;
use shardai_bytecode::instruction::Instruction;
use shardai_bytecode::opcodes::*;
use shardai_syntax::parser::expr::Expr;
use shardai_syntax::parser::stmt::Stmt;
use std::collections::HashMap;

const VERSION_MAJOR: u8 = 0;
const VERSION_MINOR: u8 = 0;

#[derive(Copy, Clone)]
enum Local {
    Mutable(u8),
    Immutable(u8),
}

pub struct Compiler {
    next_register: u8, // This sucks really badly since we can't free registers but we'll fix it later
    locals: HashMap<String, Local>,
    constants: Vec<Constant>,
    instructions: Vec<Instruction>,
}


impl Compiler {
    pub fn new() -> Self {
        Self { next_register: 0, locals: HashMap::new(), constants: Vec::new(), instructions: Vec::new() }
    }

    fn build_header(&self) -> BytecodeHeader {
        BytecodeHeader {
            signature: *b"SBC",
            version_major: VERSION_MAJOR,
            version_minor: VERSION_MINOR,
            constant_count: self.constants.len() as u16,
            instruction_count: self.instructions.len() as u32,
        }
    }

    fn add_constant(&mut self, constant: Constant) -> Result<u8, CompileError> {
        if self.constants.len() >= u8::MAX as usize {
            return Err(CompileError::TooManyConstants);
        }

        self.constants.push(constant);
        Ok((self.constants.len() - 1) as u8)
    }

    fn emit(&mut self, opcode: Op, a: u8, b: u8, c: u8) -> usize {
        let instr = Instruction { opcode, a, b, c };
        let instr_pos = self.instructions.len();

        self.instructions.push(instr);
        instr_pos
    }

    fn register_local(&mut self, name: String, register: u8) -> Result<(), CompileError> {
        self.locals.insert(name, Local::Mutable(register));

        Ok(())
    }

    fn register_local_immutable(&mut self, name: String, register: u8) -> Result<(), CompileError> {
        self.locals.insert(name, Local::Immutable(register));

        Ok(())
    }

    fn get_local(&mut self, name: &String) -> Option<Local> {
        self.locals.get(name).copied()
    }

    pub fn compile(&mut self, ast: Vec<Stmt>) -> Result<BytecodeFile, CompileError> {
        self.compile_ast(ast)?;

        Ok(BytecodeFile {
            header: self.build_header(),
            constants: self.constants.clone(),
            instructions: self.instructions.clone(),
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

                    self.emit(Op::Move, register, value_register, 0);
                }

                self.register_local(name.lexeme, register)?;
                Ok(())
            }
            Stmt::Const { name, initializer } => {
                let register = self.next_register;
                self.next_register += 1;

                let value_register = self.compile_expr(initializer)?;
                self.emit(Op::Move, register, value_register, 0);

                self.register_local_immutable(name.lexeme, register)?;
                Ok(())
            }
            Stmt::Assign { target, value } => {
                if let Expr::Identifier(t) = &target {
                    let local = self
                        .get_local(&t.lexeme)
                        .ok_or(CompileError::UnknownLocal(t.lexeme.clone()))?;

                    if let Local::Mutable(r) = local {
                        let value_register = self.compile_expr(value)?;

                        self.emit(Op::Move, r, value_register, 0);
                        Ok(())
                    } else {
                        Err(CompileError::ImmutableLocal(t.lexeme.clone()))
                    }
                } else {
                    Err(CompileError::InvalidAssignment)
                }
            }
            Stmt::Expr(e) => {
                self.compile_expr(e)?;

                Ok(())
            }
            Stmt::Return { return_value } => {
                if let Some(v) = return_value {
                    let return_value_register = self.compile_expr(v)?;
                    self.emit(Op::Return, return_value_register, 0, 0);
                    Ok(())
                } else {
                    self.emit(Op::ReturnVoid, 0, 0, 0);
                    Ok(())
                }
            }
            Stmt::If { condition, if_branch, else_branch } => {
                let condition_register = self.compile_expr(condition)?;

                match else_branch {
                    // `if` only
                    None => {
                        let cond_jump_pos = self.emit(Op::JumpIfFalsy, 0, 0, condition_register);

                        for stmt in if_branch {
                            self.compile_stmt(stmt)?;
                        }

                        self.patch_jump(cond_jump_pos);

                        Ok(())
                    }

                    // `if` and `else`
                    Some(else_branch) => {
                        let cond_jump_pos = self.emit(Op::JumpIfFalsy, 0, 0, condition_register);

                        for stmt in if_branch {
                            self.compile_stmt(stmt)?;
                        }

                        // jump past else block, emitted at end of if branch
                        let end_jump_pos = self.emit(Op::Jump, 0, 0, 0);

                        self.patch_jump(cond_jump_pos);

                        for stmt in else_branch {
                            self.compile_stmt(stmt)?;
                        }

                        self.patch_jump(end_jump_pos);

                        Ok(())
                    }
                }
            }
            Stmt::Func { name, params, body } => unimplemented!()
        }
    }

    fn compile_expr(&mut self, expr: Expr) -> Result<u8, CompileError> {
        match expr {
            Expr::Literal(value) => {
                let const_idx = self.add_constant(value.into())?;
                let dest = self.next_register;
                self.next_register += 1;

                self.emit(Op::LoadConst, dest, const_idx, 0);
                Ok(dest)
            }

            Expr::Identifier(token) => {
                let local = self
                    .get_local(&token.lexeme)
                    .ok_or(CompileError::UnknownLocal(token.lexeme.clone()))?;

                match local {
                    Local::Immutable(r) => Ok(r),
                    Local::Mutable(r) => Ok(r),
                }
            }

            Expr::Add { left, right } => self.compile_binary_op(*left, *right, Op::Add),
            Expr::Subtract { left, right } => self.compile_binary_op(*left, *right, Op::Subtract),
            Expr::Multiply { left, right } => self.compile_binary_op(*left, *right, Op::Multiply),
            Expr::Divide { left, right } => self.compile_binary_op(*left, *right, Op::Divide),
            Expr::Exponentiation { left, right } => self.compile_binary_op(*left, *right, Op::Exponentiate),
            Expr::GreaterThan { left, right } => self.compile_binary_op(*left, *right, Op::GreaterThan),
            Expr::GreaterEqualThan { left, right } => self.compile_binary_op(*left, *right, Op::GreaterEqualThan),
            Expr::LessThan { left, right } => self.compile_binary_op(*left, *right, Op::LessThan),
            Expr::LessEqualThan { left, right } => self.compile_binary_op(*left, *right, Op::LessEqualThan),
            Expr::Equals { left, right } => self.compile_binary_op(*left, *right, Op::Equals),
            Expr::NotEquals { left, right } => self.compile_binary_op(*left, *right, Op::NotEquals),
            Expr::Modulo { left, right } => self.compile_binary_op(*left, *right, Op::Modulo),
            Expr::Group { expr } => self.compile_expr(*expr),

            Expr::Not { operand } => {
                let source_register = self.compile_expr(*operand)?;
                let destination_register = self.next_register;
                self.next_register += 1;

                self.emit(Op::LogicalNot, source_register, destination_register, 0);
                Ok(destination_register)
            }

            Expr::Negate { operand } => {
                let source_register = self.compile_expr(*operand)?;
                let destination_register = self.next_register;
                self.next_register += 1;

                self.emit(Op::Negate, source_register, destination_register, 0);
                Ok(destination_register)
            }

            Expr::And { left, right } => {
                let destination_register = self.next_register;
                self.next_register += 1;
                let left_register = self.compile_expr(*left)?;

                // if left is falsy then short circuit, move left to destination and jump past right
                self.emit(Op::Move, destination_register, left_register, 0);
                let short_circuit = self.emit(Op::JumpIfFalsy, 0, 0, left_register);

                // left was truthy so we evaluate the right and move it to the destination
                let right_reg = self.compile_expr(*right)?;
                self.emit(Op::Move, destination_register, right_reg, 0);

                self.patch_jump(short_circuit);
                Ok(destination_register)
            }

            Expr::Or { left, right } => {
                let destination_register = self.next_register;
                self.next_register += 1;
                let left_register = self.compile_expr(*left)?;

                self.emit(Op::Move, destination_register, left_register, 0);
                let short_circuit = self.emit(Op::JumpIfTruthy, 0, 0, left_register);

                let right_register = self.compile_expr(*right)?;
                self.emit(Op::Move, destination_register, right_register, 0);

                self.patch_jump(short_circuit);
                Ok(destination_register)
            }

            Expr::Func { params, body } => unimplemented!()
        }
    }

    fn compile_binary_op(&mut self, left: Expr, right: Expr, op: Op) -> Result<u8, CompileError> {
        let left = self.compile_expr(left)?;
        let right = self.compile_expr(right)?;
        let dest = self.next_register;
        self.next_register += 1;

        self.emit(op, dest, left, right);
        Ok(dest)
    }

    fn patch_jump(&mut self, jump_pos: usize) {
        // subtract one since pc will be pointing past jump instruction
        let offset = (self.instructions.len() - jump_pos - 1) as i16;
        let [a, b] = offset.to_le_bytes();
        let inst = self.instructions.get_mut(jump_pos).unwrap();

        inst.a = a;
        inst.b = b;
    }
}
