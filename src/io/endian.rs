//! Aurora Font Library
//! 
//! Author: Colton McGraw <github.com/ColtMcG1>
//! License: Apache-2.0
//! Date: January 2026
//! 
//! Endianness handling utilities for the Aurora Font Library. This module provides
//! functions to handle data conversion between different endianness formats.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ByteOrder {
    #[default]
    BigEndian,
    LittleEndian,
}

impl ByteOrder {
    pub fn is_big(self) -> bool {
        matches!(self, ByteOrder::BigEndian)
    }

    pub fn is_little(self) -> bool {
        matches!(self, ByteOrder::LittleEndian)
    }
}

impl ByteOrder {

    /// Detect the native endianness of the current platform.
    pub fn detect_native() -> Self {
        if cfg!(target_endian = "big") {
            ByteOrder::BigEndian
        } else {
            ByteOrder::LittleEndian
        }
    }

    /// Detect endianness from a 2-byte sample using a simple heuristic.
    ///
    /// WARNING: This is a very weak heuristic and should only be used when
    /// there is no format metadata available. It compares the first two
    /// bytes (b0, b1) and returns a best-effort guess: if `b0 < b1` the
    /// function returns `LittleEndian`, if `b0 > b1` it returns
    /// `BigEndian`. Equal bytes or very small samples return `None`.
    pub fn detect_from_u16_data(data: &[u8]) -> Option<Self> {
        if data.len() < 2 {
            return None;
        }
        // Heuristic: compare first two bytes. This is NOT reliable for
        // font file format detection — prefer explicit table metadata.
        let (b0, b1) = (data[0], data[1]);
        if b0 < b1 {
            Some(ByteOrder::LittleEndian)
        } else if b0 > b1 {
            Some(ByteOrder::BigEndian)
        } else {
            None
        }
    }

    /// Detect endianness from u32 data using simple heuristics.
    /// 
    /// WARNING: This is a very weak heuristic and should only be used when
    /// there is no format metadata available. It compares the first four
    /// bytes and returns a best-effort guess; equal bytes (or small
    /// samples) will return `None`.
    pub fn detect_from_u32_data(data: &[u8]) -> Option<Self> {
        if data.len() < 4 {
            return None;
        }
        // Heuristic: compare the most- and least-significant bytes. This is NOT
        // reliable for font file format detection — prefer explicit table metadata.
        let b0 = data[0];
        let b3 = data[3];
        if b0 < b3 {
            Some(ByteOrder::LittleEndian)
        } else if b0 > b3 {
            Some(ByteOrder::BigEndian)
        } else {
            None
        }
    }

    /// Detect endianness from a BOM (Byte Order Mark).
    ///
    /// Note: a UTF-8 BOM (`0xEF,0xBB,0xBF`) does not indicate machine
    /// endianness and is therefore treated as `None` (unknown).
    pub fn detect_from_bom(bom: &[u8]) -> Option<Self> {
        // Check for UTF BOMs (treat UTF-8 BOM as unknown)
        match bom {
            [0xFE, 0xFF] => Some(ByteOrder::BigEndian),    // UTF-16 BE
            [0xFF, 0xFE] => Some(ByteOrder::LittleEndian), // UTF-16 LE
            [0xFF, 0xFE, 0x00, 0x00] => Some(ByteOrder::LittleEndian), // UTF-32 LE
            [0x00, 0x00, 0xFE, 0xFF] => Some(ByteOrder::BigEndian),    // UTF-32 BE
            _ => None,
        }
    }

    /// Convert a 2-byte array to a `u16` using this endianness.
    #[inline]
    pub fn u16_from(self, b: [u8; 2]) -> u16 {
        match self {
            ByteOrder::BigEndian => u16::from_be_bytes(b),
            ByteOrder::LittleEndian => u16::from_le_bytes(b),
        }
    }

    /// Convert a 4-byte array to a `u32` using this endianness.
    #[inline]
    pub fn u32_from(self, b: [u8; 4]) -> u32 {
        match self {
            ByteOrder::BigEndian => u32::from_be_bytes(b),
            ByteOrder::LittleEndian => u32::from_le_bytes(b),
        }
    }

    /// Read a `u16` from the start of the provided slice if it contains
    /// at least two bytes. Returns `None` if the slice is too short.
    #[inline]
    pub fn read_u16_from_slice(self, s: &[u8]) -> Option<u16> {
        if s.len() >= 2 {
            Some(self.u16_from([s[0], s[1]]))
        } else {
            None
        }
    }

    /// Read a `u32` from the start of the provided slice if it contains
    /// at least four bytes. Returns `None` if the slice is too short.
    #[inline]
    pub fn read_u32_from_slice(self, s: &[u8]) -> Option<u32> {
        if s.len() >= 4 {
            Some(self.u32_from([s[0], s[1], s[2], s[3]]))
        } else {
            None
        }
    }
}