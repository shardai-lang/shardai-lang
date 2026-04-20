// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use shardai_syntax::literal_value::LiteralValue;
use std::fmt::{Debug, Formatter};
use std::io;
use std::io::Write;

#[derive(Clone)]
pub enum Constant {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

pub const STRING_TAG: u8 = 0x00;
pub const NUMBER_TAG: u8 = 0x01;
pub const BOOL_TAG: u8 = 0x02;
pub const NIL_TAG: u8 = 0x03;

impl Constant {
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        match self {
            Constant::String(s) => {
                writer.write_all(&[STRING_TAG])?; // 1 byte tag
                writer.write_all(&(s.len() as u32).to_le_bytes())?; // 4 byte length
                writer.write_all(s.as_bytes())?
            }
            Constant::Number(n) => {
                writer.write_all(&[NUMBER_TAG])?; // 1 byte tag
                writer.write_all(&n.to_le_bytes())?; // 8 byte length (all numbers are f64)
            }
            Constant::Bool(n) => {
                writer.write_all(&[NUMBER_TAG])?; // 1 byte tag
                writer.write_all(&[*n as u8])?; // 1 byte length (inefficient but whatever)
            }
            Constant::Nil => {
                writer.write_all(&[NIL_TAG])?; // 1 byte tag
            }
        }

        Ok(())
    }
}

impl From<LiteralValue> for Constant {
    fn from(value: LiteralValue) -> Self {
        match value {
            LiteralValue::String(s) => Constant::String(s),
            LiteralValue::Number(n) => Constant::Number(n),
            LiteralValue::Bool(b) => Constant::Bool(b),
            LiteralValue::Nil => Constant::Nil,
        }
    }
}

impl Debug for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::String(s) => write!(f, "Constant({})", s),
            Constant::Number(n) => write!(f, "Constant({})", n),
            Constant::Bool(b) => write!(f, "Constant({})", b),
            Constant::Nil => write!(f, "Constant(nil)"),
        }
    }
}
