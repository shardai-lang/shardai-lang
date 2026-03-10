// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use std::fmt::{Debug, Formatter};

pub struct BytecodeHeader {
    pub signature: [u8; 3], // "SBC" (Shardai bytecode)
    pub version_major: u8,
    pub version_minor: u8,
    pub constant_count: u16
}

impl Debug for BytecodeHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let signature = str::from_utf8(&self.signature)
            .unwrap_or_else(|_| "Invalid UTF8 signature");

        write!(f, "Signature: {},", signature)?;
        write!(f, " Version: {}.{},", self.version_major, self.version_minor)?;
        write!(f, " Constant Count: {}", self.constant_count)?;

        Ok(())
    }
}