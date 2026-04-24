// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

mod common;

use crate::common::bytecode_file_builder::build;
use shardai_bytecode::constant::Constant;
use shardai_bytecode::instruction::Instruction;
use shardai_bytecode::opcodes::Op;
use shardai_vm::value::Value;
use shardai_vm::vm::VM;

#[test]
fn add() {
    #[rustfmt::skip]
    let bytecode_file = build(vec![
        Instruction {opcode: Op::LoadConst, a: 1, b: 0, c: 0},
        Instruction {opcode: Op::LoadConst, a: 2, b: 1, c: 0},
        Instruction {opcode: Op::Add, a: 0, b: 1, c: 2},
        Instruction {opcode: Op::Return, a: 0, b: 0, c: 0}
    ], vec![
        Constant::Number(5.into()),
        Constant::Number(10.into())
    ]);

    let mut vm = VM::new(bytecode_file);
    assert_eq!(vm.run().unwrap(), Value::Number(15.into()));
}

#[test]
fn subtract() {
    #[rustfmt::skip]
    let bytecode_file = build(vec![
        Instruction {opcode: Op::LoadConst, a: 1, b: 0, c: 0},
        Instruction {opcode: Op::LoadConst, a: 2, b: 1, c: 0},
        Instruction {opcode: Op::Subtract, a: 0, b: 1, c: 2},
        Instruction {opcode: Op::Return, a: 0, b: 0, c: 0}
    ], vec![
        Constant::Number(10.into()),
        Constant::Number(5.into())
    ]);

    let mut vm = VM::new(bytecode_file);
    assert_eq!(vm.run().unwrap(), Value::Number(5.into()));
}

#[test]
fn multiply() {
    #[rustfmt::skip]
    let bytecode_file = build(vec![
        Instruction {opcode: Op::LoadConst, a: 1, b: 0, c: 0},
        Instruction {opcode: Op::LoadConst, a: 2, b: 1, c: 0},
        Instruction {opcode: Op::Multiply, a: 0, b: 1, c: 2},
        Instruction {opcode: Op::Return, a: 0, b: 0, c: 0}
    ], vec![
        Constant::Number(5.into()),
        Constant::Number(10.into())
    ]);

    let mut vm = VM::new(bytecode_file);
    assert_eq!(vm.run().unwrap(), Value::Number(50.into()));
}

#[test]
fn divide() {
    #[rustfmt::skip]
    let bytecode_file = build(vec![
        Instruction {opcode: Op::LoadConst, a: 1, b: 0, c: 0},
        Instruction {opcode: Op::LoadConst, a: 2, b: 1, c: 0},
        Instruction {opcode: Op::Divide, a: 0, b: 1, c: 2},
        Instruction {opcode: Op::Return, a: 0, b: 0, c: 0}
    ], vec![
        Constant::Number(5.into()),
        Constant::Number(10.into())
    ]);

    let mut vm = VM::new(bytecode_file);
    assert_eq!(vm.run().unwrap(), Value::Number(0.5.into()));
}

#[test]
fn exponentiation() {
    #[rustfmt::skip]
    let bytecode_file = build(vec![
        Instruction {opcode: Op::LoadConst, a: 1, b: 0, c: 0},
        Instruction {opcode: Op::LoadConst, a: 2, b: 1, c: 0},
        Instruction {opcode: Op::Exponentiate, a: 0, b: 1, c: 2},
        Instruction {opcode: Op::Return, a: 0, b: 0, c: 0}
    ], vec![
        Constant::Number(2.into()),
        Constant::Number(3.into())
    ]);

    let mut vm = VM::new(bytecode_file);
    assert_eq!(vm.run().unwrap(), Value::Number(8.into()));
}
