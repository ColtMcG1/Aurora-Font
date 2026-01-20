//! Aurora Font Library
//! 
//! Author: Colton McGraw <github.com/ColtMcG1>
//! License: Apache-2.0
//! Date: January 2026
//! 
//! # Purpose
//! 
//! Font file decoding utilities for the Aurora Font Library. This module provides
//! functions to decode various font file formats into specification defined representations.
//! 
//! It supports formats such as OTF, TTF, WOFF, WOFF2, and SVG fonts.
//! 
//! The decoding functions read from a `FontDataStream` and return structured
//! representations of the font data.
//! 
//! For futher information about the structures returned by these functions, refer to the
//! respective format modules in `src/common/`.
//! 
//! # Limitations
//! 
//! The decoding functions do not perform high-level font manipulation or rendering.
//! They focus solely on parsing the raw font data into structured forms. For high-level
//! font operations, consider using the optional `full` feature of the library. See
//! `src/optional/` for high-level utilities.

/// Enumeration of supported font file headers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FontFileHeader {
    /// SNFT / TrueType / OpenType font file.
    SFNT,
    /// 'true' - Apple TrueType font. (Internally treated as SFNT)
    TRUE,
    /// 'typ1' - Type 1 font. (Deprecated)
    /// Adobe Type 1 fonts are largely superseded by OpenType with CFF outlines. Adobe EOLed
    /// support for Type 1 fonts in 2023. This variant is retained for legacy compatibility.
    /// However, no parsing or decoding functionality for Type 1 fonts is provided in this library.
    #[warn(deprecated)]
    TYP1,
    /// 'OTTO' - OpenType font with CFF outlines. (Internally treated as SFNT)
    OTTO,
    /// 'wOFF' - Web Open Font Format. (Internally treated as SFNT)
    WOFF,
    /// 'wOF2' - Web Open Font Format 2. (Internally treated as SFNT after decompression)
    WOF2,
    /// 'svg ' - SVG font. (Uses XML-based representation)
    SVG,
    /// Unknown or unsupported font file header.
    Unknown,
}

impl From<u32> for FontFileHeader {
    fn from(value: u32) -> Self {
        match value {
            0x00010000 => FontFileHeader::SFNT,      // sfnt version for TrueType
            0x74727565 => FontFileHeader::TRUE,      // 'true'
            0x74797031 => FontFileHeader::TYP1,      // 'typ1'
            0x4F54544F => FontFileHeader::OTTO,      // 'OTTO'
            0x774F4646 => FontFileHeader::WOFF,      // 'wOFF'
            0x774F4632 => FontFileHeader::WOF2,      // 'wOF2'
            0x73766720 => FontFileHeader::SVG,       // 'svg '
            _ => FontFileHeader::Unknown,
        }
    }
}

use crate::io::FontDataStream;
use crate::error::Error;

#[inline]
pub fn decode_font_type(stream: &mut FontDataStream) -> Result<FontFileHeader, Error> {
    // Read the first 4 bytes without changing the stream position so callers
    // can continue parsing from the file start. Use `slice_range` to obtain
    // a zero-copy view of the header.
    let bytes = stream.slice_range(0..4)?;
    let signature = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    Ok(FontFileHeader::from(signature))
}

use super::snft;

/// Decode an OTF font from the provided data stream.
#[inline]
pub fn decode_font_otf(stream: &mut FontDataStream) -> Result<snft::SnftTable, Error> {
    snft::read_snft(stream)
}

/// Decode a TrueType Collection (TTC) font from the provided data stream.
#[inline]
pub fn decode_font_ttf(stream: &mut FontDataStream) -> Result<snft::SnftTable, Error> {
    snft::read_snft(stream)
}

/// Decode a WOFF font from the provided data stream.
#[inline]
pub fn decode_font_woff(_stream: &mut FontDataStream) -> Result<snft::SnftTable, Error> {
    // WOFF decoding not yet implemented
    Err(Error::InvalidFormat)
}

/// Decode a WOFF2 font from the provided data stream.
#[inline]
pub fn decode_font_woff2(_stream: &mut FontDataStream) -> Result<snft::SnftTable, Error> {
    // WOFF2 decoding not yet implemented
    Err(Error::InvalidFormat)
}

/// Decode an SVG font from the provided data stream.
#[inline]
pub fn decode_font_svg(_stream: &mut FontDataStream) -> Result<snft::SnftTable, Error> {
    // SVG font decoding not yet implemented
    Err(Error::InvalidFormat)
}