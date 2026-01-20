//! Aurora Font Library
//! 
//! Author: Colton McGraw <github.com/ColtMcG1>
//! License: Apache-2.0
//! Date: January 2026

/// Trait defining a sink for font decoding output.
/// 
/// Implementors of this trait can consume various font tables and data
/// during the decoding process and produce a final output.
/// 
/// Type Parameters:
/// - `Output`: The type of the final output produced by the sink.
/// - `Err`: The error type that the sink may return during consumption or finalization.
/// 
/// Examples:
/// ```ignore
/// struct MyFontSink { /* fields */ }
///
/// impl FontSink for MyFontSink {
///     type Output = MyFontRepresentation;
///     type Err = MyFontError;
///     fn consume_snft(&mut self, snft: crate::common::snft::SnftTable) -> Result<(), Self::Err> {
///        // Process the SNFT table
///       Ok(())
///     }
///     fn finish(self) -> Result<Self::Output, Self::Err> {
///       // Produce the final font representation
///       Ok(MyFontRepresentation { /* fields */ })
///     }
/// }
/// ```
pub trait FontSink {
    /// The final output produced by the sink when decoding finishes.
    type Output;
    /// The error type the sink may return.
    type Err;

    /// Consume an SNFT table. Implementations should store or process
    /// the table; returning an error aborts decoding.
    fn consume_snft(&mut self, snft: crate::common::snft::SnftTable) -> Result<(), Self::Err>;

    /// Consume SVG font data. The slice is only valid for the duration of the
    /// call; implementors should copy it if they need to retain it.
    //fn consume_svg(&mut self, svg_data: &[u8]) -> Result<(), Self::Err>;

    /// Finalize the sink and produce its output value.
    fn finish(self) -> Result<Self::Output, Self::Err>;
}