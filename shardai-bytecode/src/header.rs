// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use std::fmt::{Debug, Formatter};
use std::io;
use std::io::Write;

pub struct BytecodeHeader {
    pub signature: [u8; 3], // "SBC" (Shardai bytecode)
    pub version_major: u8,
    pub version_minor: u8,
    pub constant_count: u16,
}

impl Debug for BytecodeHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let signature =
            str::from_utf8(&self.signature).unwrap_or("Invalid UTF8 signature");

        write!(f, "Signature: {},", signature)?;
        write!(
            f,
            " Version: {}.{},",
            self.version_major, self.version_minor
        )?;
        write!(f, " Constant Count: {}", self.constant_count)?;

        Ok(())
    }
}

impl BytecodeHeader {
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&self.signature)?; // Signature
        writer.write_all(&[self.version_major, self.version_minor])?; // Version
        writer.write_all(&self.constant_count.to_le_bytes())?; // Constant count

        Ok(())
    }
}
