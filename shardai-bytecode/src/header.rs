// Copyright 2026 wyteroze. Licensed under the Apache License, Version 2.0.

use std::fmt::{Debug, Formatter};
use std::io;
use std::io::{Read, Write};

pub struct BytecodeHeader {
    pub signature: [u8; 3], // "SBC" (Shardai bytecode)
    pub version_major: u8,
    pub version_minor: u8
}

impl Debug for BytecodeHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let signature = str::from_utf8(&self.signature).unwrap_or("Invalid UTF8 signature");

        write!(f, "Signature: {},", signature)?;
        write!(f, " Version: {}.{},", self.version_major, self.version_minor)?;
        Ok(())
    }
}

impl BytecodeHeader {
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&self.signature)?; // Signature
        writer.write_all(&[self.version_major, self.version_minor])?; // Version

        Ok(())
    }

    pub fn read(reader: &mut impl Read) -> io::Result<Self> {
        let mut signature = [0u8; 3];
        let mut version_major = [0u8; 1];
        let mut version_minor = [0u8; 1];
        let mut constant_count_bytes = [0u8; 2]; // one u16
        let mut instruction_count_bytes = [0u8; 4]; // one u16

        reader.read_exact(&mut signature)?;
        reader.read_exact(&mut version_major)?;
        reader.read_exact(&mut version_minor)?;
        reader.read_exact(&mut constant_count_bytes)?;
        reader.read_exact(&mut instruction_count_bytes)?;

        Ok(Self {
            signature,
            version_major: u8::from_le_bytes(version_major),
            version_minor: u8::from_le_bytes(version_minor)
        })
    }
}
