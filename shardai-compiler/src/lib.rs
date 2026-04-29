pub mod compiler;
mod error;
mod register_allocator;

use crate::compiler::Compiler;
use crate::error::CompileError;
use shardai_bytecode::file::BytecodeFile;
use shardai_syntax::parser::stmt::Stmt;

pub fn compile_ast(ast: Vec<Stmt>) -> Result<BytecodeFile, CompileError> {
    let mut compiler = Compiler::new();
    compiler.compile(ast)
}
