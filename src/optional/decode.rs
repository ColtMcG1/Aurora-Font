//! Aurora Font Library
//! 
//! Author: Colton McGraw <github.com/ColtMcG1>
//! License: Apache-2.0
//! Date: January 2026

/// High-level font decoder dispatch.
///
/// Detects the file header without consuming the stream and delegates to the
/// appropriate decoding function in the format-specific module.
pub fn decode_font_file(stream: &mut FontDataStream) -> Result<S, Error> {
    use super::common::*;
    match decode_font_type(stream)? {
        FontFileHeader::SFNT => decode_font_ttf(stream),
        FontFileHeader::TRUE => decode_font_ttf(stream),
        FontFileHeader::TYP1 => decode_font_otf(stream),
        FontFileHeader::OTTO => decode_font_otf(stream),
        FontFileHeader::WOFF => decode_font_woff(stream),
        FontFileHeader::WOF2 => decode_font_woff2(stream),
        FontFileHeader::SVG => decode_font_svg(stream),
        _ => Err(Error::InvalidFormat),
    }
}

/// High-level decode convenience: decode into built-in `Font` representation.
pub fn decode(stream: &mut FontDataStream) -> Result<crate::font::Font, Error> {
    // The actual generic `decode_into` implementation lives in `common::snft`.
    let font = crate::font::Font::new();
    let out = snft::decode_into(stream, font)?;
    Ok(out)
}

use crate::font::Font;

/// Decode a font from the provided data stream. Into a high-level `Font`
/// representation.
pub fn decode(stream: &mut FontDataStream) -> Result<Font, Error> {
    let font = Font::new();
    let out = decode_into(stream, font)?;
    Ok(out)
}

use crate::font::{FontSink};

/// Decode into a provided `FontSink` implementation.
///
/// The sink receives table entries (and SVG data for `SVG` files). Sink
/// errors are converted into crate `Error` via `Into<Error>`.
pub fn decode_into<S>(stream: &mut FontDataStream, mut sink: S) -> Result<S::Output, Error>
where
    S: FontSink,
    S::Err: Into<Error>,
{
    use super::snft::extract_snft_tables_from_stream;

    match decode_font_type(stream)? {
        FontFileHeader::SFNT
        | FontFileHeader::TRUE
        | FontFileHeader::TYP1
        | FontFileHeader::OTTO => {
            let snft = extract_snft_tables_from_stream(stream)?;
                sink.consume_snft(snft).map_err(|e| e.into())?;
        }
        FontFileHeader::WOFF | FontFileHeader::WOF2 => return Err(Error::InvalidFormat),
        FontFileHeader::SVG => {
            let bytes = stream.slice_range(0..stream.len())?;
            //sink.consume_svg(bytes).map_err(|e| e.into())?;
        }
        _ => {
            let snft = extract_snft_tables_from_stream(stream)?;
            sink.consume_snft(snft).map_err(|e| e.into())?;
        }
    }

    sink.finish().map_err(|e| e.into())
}