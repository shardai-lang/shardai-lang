pub mod compiler;
mod error;

use shardai_bytecode::file::BytecodeFile;
use shardai_syntax::parser::stmt::Stmt;
use crate::compiler::Compiler;
use crate::error::CompileError;

pub fn compile_ast(ast: Vec<Stmt>) -> Result<BytecodeFile, CompileError> {
    let mut compiler = Compiler::new();
    compiler.compile(ast)
}