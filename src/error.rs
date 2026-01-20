//! Aurora Font Library
//! 
//! Author: Colton McGraw <github.com/ColtMcG1>
//! License: Apache-2.0
//! Date: January 2026
//! 
//! Error handling utilities for the Aurora Font Library. This module defines
//! common error types used throughout the library.


use std::fmt;

/// General error type for the Aurora Font Library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// I/O related errors. Additional details provided in `IoError`.
    Io(IoError),
    /// Invalid format encountered.
    InvalidFormat,
}

/// Specific I/O error types for font file processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IoError {
    /// Attempted to read `requested` bytes but only `available` bytes remain.
    OutOfBounds { requested: usize, available: usize },
    /// A seek attempted to move before the start of the stream.
    SeekBeforeStart,
    /// A read was requested at an absolute `offset` that is invalid.
    InvalidOffset { offset: usize },
    /// A 4-byte tag was invalid or unexpected.
    InvalidTag([u8; 4]),
    /// Version not supported.
    UnsupportedVersion(u32),
    /// Checksum mismatch during validation. Includes table tag and file offset
    /// where the table data begins, plus expected and found checksum values.
    ChecksumMismatch { table: [u8; 4], offset: usize, expected: u32, found: u32 },
    /// A table was truncated; table id, expected and found lengths.
    TruncatedTable { table: [u8; 4], expected_len: usize, found_len: usize },
    /// UTF-8 decoding failed at `offset`.
    InvalidUtf8 { offset: usize },
    /// Glyph index out of range.
    InvalidGlyphIndex { index: u32 },
    /// Generic invalid data placeholder.
    InvalidData,
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::OutOfBounds { requested, available } => {
                write!(f, "out of bounds read: requested {}, available {}", requested, available)
            }
            IoError::SeekBeforeStart => write!(f, "seek before start"),
            IoError::InvalidOffset { offset } => write!(f, "invalid offset: {}", offset),
            IoError::InvalidTag(tag) => write!(f, "invalid tag: {}{}{}{}", tag[0] as char, tag[1] as char, tag[2] as char, tag[3] as char),
            IoError::UnsupportedVersion(v) => write!(f, "unsupported version: {}", v),
            IoError::ChecksumMismatch { table, offset, expected, found } => write!(
                f,
                "checksum mismatch for table {}{}{}{} at offset {}: expected {}, found {}",
                table[0] as char,
                table[1] as char,
                table[2] as char,
                table[3] as char,
                offset,
                expected,
                found
            ),
            IoError::TruncatedTable { table, expected_len, found_len } => write!(f, "truncated table {}{}{}{}: expected {}, found {}", table[0] as char, table[1] as char, table[2] as char, table[3] as char, expected_len, found_len),
            IoError::InvalidUtf8 { offset } => write!(f, "invalid utf8 at offset {}", offset),
            IoError::InvalidGlyphIndex { index } => write!(f, "invalid glyph index {}", index),
            IoError::InvalidData => write!(f, "invalid data"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::InvalidFormat => write!(f, "invalid format"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
        Error::Io(IoError::InvalidData)
    }
}