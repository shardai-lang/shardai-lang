// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use std::fmt::{Debug, Display, Formatter};
use shardai_bytecode::chunk::Chunk;

pub enum HeapObj {
    String(String),
    Chunk(Chunk)
}

impl PartialEq for HeapObj {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HeapObj::String(s1), HeapObj::String(s2)) if s1 == s2 => true,

            // TODO: check if they are stored at same index in heap
            (HeapObj::Chunk(c1), HeapObj::Chunk(c2)) => false,

            _ => false
        }
    }
}

impl Debug for HeapObj {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HeapObj::String(_) => write!(f, "string"),
            HeapObj::Chunk(_) => write!(f, "chunk")
        }
    }
}

impl Display for HeapObj {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HeapObj::String(s) => write!(f, "{}", s),
            HeapObj::Chunk(_) => write!(f, "chunk")
        }
    }
}
