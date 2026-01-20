//! Aurora Font Library
//! 
//! Author: Colton McGraw <github.com/ColtMcG1>
//! License: Apache-2.0
//! Date: January 2026

use super::sink::FontSink;
use crate::common::snft::{SnftTableEntry, SnftTable};
use std::convert::Infallible;

/// Simple built-in font representation used by the convenience decoder.
pub struct Font {
    /// SNFT tables contained in the font.
    pub snft_tables: Vec<SnftTableEntry>,
    /// Optional SVG data associated with the font.
    pub svg_data: Option<Vec<u8>>,
}

impl Font {
    pub fn new() -> Self {
        Self {
            snft_tables: Vec::new(),
            svg_data: None,
        }
    }
}

impl FontSink for Font {
    type Output = Self;
    type Err = Infallible;

    fn consume_snft(&mut self, snft: SnftTable) -> Result<(), Self::Err> {
        self.snft_tables.extend(snft.tables);
        Ok(())
    }

    // fn consume_svg(&mut self, svg_data: &[u8]) -> Result<(), Self::Err> {
    //     self.svg_data = Some(svg_data.to_vec());
    //     Ok(())
    // }

    fn finish(self) -> Result<Self::Output, Self::Err> {
        Ok(self)
    }
}