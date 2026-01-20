//! Aurora Font Library
//!
//! Author: Colton McGraw <github.com/ColtMcG1>
//! License: Apache-2.0
//! Date: January 2026
//!
//! Font data streaming utilities for the Aurora Font Library. This module provides
//! functions to read from and write to font data streams,
//! handling buffering and encoding/decoding as necessary.

use super::endian;
use crate::error::{Error, IoError};

/// # Font Data Stream
///
/// A structure representing a font data stream.
///
/// It provides methods to read various data types from the stream,
/// taking into account the specified endianness.
///
/// This structure is designed to facilitate reading font data, it also
/// supports reading multi-byte values with the correct byte order.
///
/// # Fields
///
/// - `data` is a byte slice representing the font data.
/// - `position` is the current read position in the stream.
/// - `endianness` specifies the byte order for multi-byte reads.
#[derive(Debug)]
pub struct FontDataStream<'a> {
    /// The byte slice representing the font data.
    data: &'a [u8],
    /// The current read position in the stream.
    position: usize,
    /// The endianness for multi-byte reads.
    endianness: endian::ByteOrder,
}

impl<'a> FontDataStream<'a> {
    /// # FontDataStream
    ///
    /// Creates a new FontDataStream with the given data slice.
    ///
    /// **Endianness** defaults to Big Endian. See `with_endianness` to change it.
    /// Most fonts use Big Endian byte order, since most are built on SNFT.
    pub fn new(data: &'a [u8]) -> Self {
        FontDataStream {
            data,
            position: 0,
            endianness: endian::ByteOrder::BigEndian,
        }
    }

    /// Sets the endianness for the stream.
    pub fn with_endianness(mut self, endianness: endian::ByteOrder) -> Self {
        self.endianness = endianness;
        self
    }

    /// Reads a single byte from the stream. Advances the position by 1 byte.
    pub fn read_u8(&mut self) -> Result<u8, Error> {
        let available = self.data.len().saturating_sub(self.position);
        if available >= 1 {
            let value = self.data[self.position];
            self.position += 1;
            Ok(value)
        } else {
            Err(Error::Io(IoError::OutOfBounds {
                requested: 1,
                available,
            }))
        }
    }

    /// Reads a 16-bit unsigned integer from the stream. Advances the position by 2 bytes.
    pub fn read_u16(&mut self) -> Result<u16, Error> {
        let available = self.data.len().saturating_sub(self.position);
        if available >= 2 {
            let bytes = &self.data[self.position..self.position + 2];
            self.position += 2;
            Ok(match self.endianness {
                endian::ByteOrder::BigEndian => u16::from_be_bytes([bytes[0], bytes[1]]),
                endian::ByteOrder::LittleEndian => u16::from_le_bytes([bytes[0], bytes[1]]),
            })
        } else {
            Err(Error::Io(IoError::OutOfBounds {
                requested: 2,
                available,
            }))
        }
    }

    /// Reads a 32-bit unsigned integer from the stream. Advances the position by 4 bytes.
    pub fn read_u32(&mut self) -> Result<u32, Error> {
        let available = self.data.len().saturating_sub(self.position);
        if available >= 4 {
            let bytes = &self.data[self.position..self.position + 4];
            self.position += 4;
            Ok(match self.endianness {
                endian::ByteOrder::BigEndian => {
                    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                }
                endian::ByteOrder::LittleEndian => {
                    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                }
            })
        } else {
            Err(Error::Io(IoError::OutOfBounds {
                requested: 4,
                available,
            }))
        }
    }

    /// Reads a signed 8-bit integer from the stream. Advances the position by 1 byte.
    pub fn read_i8(&mut self) -> Result<i8, Error> {
        self.read_u8().map(|v| v as i8)
    }
    /// Reads a signed 16-bit integer from the stream. Advances the position by 2 bytes.
    pub fn read_i16(&mut self) -> Result<i16, Error> {
        self.read_u16().map(|v| v as i16)
    }
    /// Reads a signed 32-bit integer from the stream. Advances the position by 4 bytes.
    pub fn read_i32(&mut self) -> Result<i32, Error> {
        self.read_u32().map(|v| v as i32)
    }

    /// Skips a specified number of bytes in the stream.
    pub fn skip(&mut self, bytes: usize) {
        self.position = usize::min(self.position + bytes, self.data.len());
    }

    /// Resets the stream position to the beginning.
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Returns the current position in the stream.
    pub fn position(&self) -> usize {
        self.position
    }
    /// Returns the current position in the stream.
    pub fn tell(&self) -> usize {
        self.position
    }

    /// Returns whether the end of the stream has been reached.
    pub fn is_eof(&self) -> bool {
        self.position >= self.data.len()
    }

    /// Returns the remaining bytes in the stream.
    pub fn remaining_bytes(&self) -> &[u8] {
        &self.data[self.position..]
    }

    /// Returns the total length of the stream data.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns whether the stream is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Sets the stream position to a specific offset.
    ///
    /// Note: If the position is beyond the end of the stream, it is set to the end.
    /// No error is returned for out-of-bounds seeks; the position is clamped.
    pub fn seek(&mut self, position: usize) {
        self.position = usize::min(position, self.data.len());
    }

    /// Reads a specified number of bytes from the stream.
    /// Advances the position by the number of bytes read.
    pub fn read_bytes(&mut self, length: usize) -> Result<&'a [u8], Error> {
        let available = self.data.len().saturating_sub(self.position);
        if available >= length {
            let bytes = &self.data[self.position..self.position + length];
            self.position += length;
            Ok(bytes)
        } else {
            Err(Error::Io(IoError::OutOfBounds {
                requested: length,
                available,
            }))
        }
    }

    /// Peeks a single byte at the current position without advancing.
    pub fn peek_u8(&self) -> Result<u8, Error> {
        let available = self.data.len().saturating_sub(self.position);
        self.data
            .get(self.position)
            .copied()
            .ok_or(Error::Io(IoError::OutOfBounds {
                requested: 1,
                available,
            }))
    }

    /// Peeks a slice of bytes at the current position without advancing.
    pub fn peek_bytes(&self, length: usize) -> Result<&'a [u8], Error> {
        let available = self.data.len().saturating_sub(self.position);
        self.data
            .get(self.position..self.position + length)
            .ok_or(Error::Io(IoError::OutOfBounds {
                requested: length,
                available,
            }))
    }

    /// Reads a 8-bit unsigned integer at an absolute offset without changing position.
    pub fn read_at_u8(&self, offset: usize) -> Result<u8, Error> {
        let len = self.data.len();
        if let Some(&byte) = self.data.get(offset) {
            Ok(byte)
        } else {
            let available = if offset < len { len - offset } else { 0 };
            Err(Error::Io(IoError::OutOfBounds {
                requested: 1,
                available,
            }))
        }
    }

    /// Reads a 16-bit unsigned integer at an absolute offset without changing position.
    pub fn read_at_u16(&self, offset: usize) -> Result<u16, Error> {
        let len = self.data.len();
        if let Some(bytes) = self.data.get(offset..offset + 2) {
            Ok(match self.endianness {
                endian::ByteOrder::BigEndian => u16::from_be_bytes([bytes[0], bytes[1]]),
                endian::ByteOrder::LittleEndian => u16::from_le_bytes([bytes[0], bytes[1]]),
            })
        } else {
            let available = if offset < len { len - offset } else { 0 };
            Err(Error::Io(IoError::OutOfBounds {
                requested: 2,
                available,
            }))
        }
    }

    /// Reads a 32-bit unsigned integer at an absolute offset without changing position.
    pub fn read_at_u32(&self, offset: usize) -> Result<u32, Error> {
        let len = self.data.len();
        if let Some(bytes) = self.data.get(offset..offset + 4) {
            Ok(match self.endianness {
                endian::ByteOrder::BigEndian => {
                    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                }
                endian::ByteOrder::LittleEndian => {
                    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                }
            })
        } else {
            let available = if offset < len { len - offset } else { 0 };
            Err(Error::Io(IoError::OutOfBounds {
                requested: 4,
                available,
            }))
        }
    }

    /// Returns a zero-copy slice of the underlying data at an absolute offset.
    pub fn slice_at(&self, offset: usize, length: usize) -> Result<&'a [u8], Error> {
        let len = self.data.len();
        self.data
            .get(offset..offset + length)
            .ok_or(Error::Io(IoError::OutOfBounds {
                requested: length,
                available: if offset < len { len - offset } else { 0 },
            }))
    }

    /// Returns a zero-copy slice for the given range (`start..end`, end exclusive).
    ///
    /// Returns `Err(Error::Io(IoError::OutOfBounds{..}))` when the requested
    /// range extends past the available data. If `start > end` returns
    /// `Err(Error::Io(IoError::InvalidOffset { offset: start }))`.
    pub fn slice_range(&self, range: std::ops::Range<usize>) -> Result<&'a [u8], Error> {
        let len = self.data.len();
        let start = range.start;
        let end = range.end;

        if start > end {
            return Err(Error::Io(IoError::InvalidOffset { offset: start }));
        }

        // If the end is within bounds, return the slice directly.
        if end <= len {
            return Ok(&self.data[start..end]);
        }

        // Otherwise compute available bytes at `start` and report OutOfBounds.
        let available = if start < len { len - start } else { 0 };
        let requested = end.saturating_sub(start);
        Err(Error::Io(IoError::OutOfBounds {
            requested,
            available,
        }))
    }

    /// Reads a four-byte tag (common in font tables) and advances the position.
    pub fn read_tag(&mut self) -> Result<[u8; 4], Error> {
        let bytes = self.read_bytes(4)?;
        Ok([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl std::io::Read for FontDataStream<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_to_read = usize::min(buf.len(), self.data.len() - self.position);
        buf[..bytes_to_read]
            .copy_from_slice(&self.data[self.position..self.position + bytes_to_read]);
        self.position += bytes_to_read;
        Ok(bytes_to_read)
    }
}

impl std::io::Seek for FontDataStream<'_> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        let new_position = match pos {
            std::io::SeekFrom::Start(offset) => {
                if offset > i64::MAX as u64 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "seek offset out of range",
                    ));
                }
                offset as i64
            }
            std::io::SeekFrom::End(offset) => (self.data.len() as i64).saturating_add(offset),
            std::io::SeekFrom::Current(offset) => (self.position as i64).saturating_add(offset),
        };

        if new_position < 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Seek before start",
            ));
        }

        self.position = usize::min(new_position as usize, self.data.len());
        Ok(self.position as u64)
    }
}
