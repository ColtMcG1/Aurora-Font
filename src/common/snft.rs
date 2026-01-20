//! Aurora Font Library
//! 
//! Author: Colton McGraw <github.com/ColtMcG1>
//! License: Apache-2.0
//! Date: January 2026
//! 
//! SNFT (Simple New Font Table) representation and utilities for the Aurora Font Library. 
//! This module provides functions to extract and validate SNFT tables from font data streams.

use crate::error::{Error, IoError};
use crate::io::stream::FontDataStream;

/// SNFT (Simple New Font Table) representation and utilities.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SnftTable {
    /// Font file header.
    pub header: SnftTableHeader,
    /// List of SNFT table entries.
    pub tables: Vec<SnftTableEntry>,
}

impl SnftTable {

    /// Retrieves a table entry by its tag.
    pub fn get_table_by_tag(&self, tag: &[u8; 4]) -> Option<&SnftTableEntry> {
        self.tables.iter().find(|t| &t.tag == tag)
    }

    /// Lists all table tags in the SNFT file.
    pub fn list_table_tags(&self) -> Vec<String> {
        self.tables
            .iter()
            .map(|t| String::from_utf8_lossy(&t.tag).to_string())
            .collect()
    }

    /// Returns the number of tables in the SNFT file.
    pub fn table_count(&self) -> usize {
        self.tables.len()
    }

    /// Checks if a table with the specified tag exists.
    pub fn has_table(&self, tag: &[u8; 4]) -> bool {
        self.tables.iter().any(|t| &t.tag == tag)
    }

    /// Validates all table checksums against the provided data stream.
    pub fn validate_checksums(&self, stream: &mut FontDataStream) -> Result<(), Error> {
        validate_snft_tables(self.tables.clone(), stream)
    }

    /// Returns the SNFT file version.
    pub fn version(&self) -> u32 {
        self.header.version
    }

    /// Returns the number of tables in the SNFT file.
    pub fn num_tables(&self) -> u16 {
        self.header.num_tables
    }

    /// Returns the search range for table entries.
    pub fn search_range(&self) -> u16 {
        self.header.search_range
    }

    /// Returns the entry selector for table entries.
    pub fn entry_selector(&self) -> u16 {
        self.header.entry_selector
    }

    /// Returns the range shift for table entries.
    pub fn range_shift(&self) -> u16 {
        self.header.range_shift
    }

    /// Returns a reference to all table entries.
    pub fn all_tables(&self) -> &Vec<SnftTableEntry> {
        &self.tables
    }

    /// Returns a mutable reference to all table entries.
    pub fn all_tables_mut(&mut self) -> &mut Vec<SnftTableEntry> {
        &mut self.tables
    }

    /// Checks if there are any tables in the SNFT file.
    pub fn is_empty(&self) -> bool {
        self.tables.is_empty()
    }
}

/// SNFT table header representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SnftTableHeader {
    /// Version of the SNFT file format.
    pub version: u32,
    /// Number of tables in the SNFT file.
    pub num_tables: u16,
    /// Search range for table entries.
    pub search_range: u16,
    /// Entry selector for table entries.
    pub entry_selector: u16,
    /// Range shift for table entries.
    pub range_shift: u16,
}

/// SNFT table entry representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SnftTableEntry {
    /// 4-byte tag identifying the table.
    pub tag: [u8; 4],
    /// Checksum of the table data.
    pub checksum: u32,
    /// Offset of the table data in the font file.
    pub offset: u32,
    /// Length of the table data in bytes.
    pub length: u32,
}

impl SnftTableEntry {
    /// Creates a new SNFT table entry.
    pub fn new(tag: [u8; 4], checksum: u32, offset: u32, length: u32) -> Self {
        SnftTableEntry {
            tag,
            checksum,
            offset,
            length,
        }
    }

    /// Returns the tag as a string.
    pub fn tag_as_string(&self) -> String {
        String::from_utf8_lossy(&self.tag).to_string()
    }

    /// Checks if the table has the specified tag.
    pub fn is_tag(&self, tag: &[u8; 4]) -> bool {
        &self.tag == tag
    }

    /// Checks if the table has the specified tag string.
    pub fn is_tag_str(&self, tag: &str) -> bool {
        self.tag_as_string() == tag
    }

    /// Returns the checksum of the table.
    pub fn checksum(&self) -> u32 {
        self.checksum
    }

    /// Returns the offset of the table data.
    pub fn offset(&self) -> u32 {
        self.offset
    }

    /// Returns the length of the table data.
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Validates the checksum of the table against the provided checksum.
    pub fn matches_checksum(&self, checksum: u32) -> bool {
        self.checksum == checksum
    }

    /// Computes the checksum of the given data according to SNFT specification.
    pub fn compute_checksum(&self, data: &[u8]) -> u32 {
        let mut sum: u32 = 0;
        let mut chunks = data.chunks(4);
        for chunk in &mut chunks {
            let mut buf = [0u8; 4];
            for i in 0..chunk.len() {
                buf[i] = chunk[i];
            }
            sum = sum.wrapping_add(u32::from_be_bytes(buf));
        }
        sum
    }
}

/// SNFT file tag constant.
pub const SNFT_TAG: u32 = u32::from_be_bytes([0x53, 0x4E, 0x46, 0x54]);

/// Extracts SNFT table entries from the provided data stream.
/// 
/// # Arguments
/// * `data` - A mutable reference to the font data stream.
/// 
/// # Returns
/// * `Result<SnftTable, Error>` - On success, returns the extracted SNFT table representation; on failure, returns an error.
/// 
/// # Errors
/// * Returns `Error` if reading from the data stream fails or if the SNFT format is invalid.
pub fn read_snft(
    data: &mut FontDataStream,
) -> Result<SnftTable, Error> {
    let mut tables = Vec::new();

    // Read SNFT header
    let header = read_snft_header(data)?;

    // Read table records
    for _ in 0..header.num_tables {
        tables.push(read_snft_table_entry(data)?);
    }
    
    Ok(SnftTable {
        header,
        tables,
    })
}

/// Extracts the SNFT table header from the provided data stream.
pub fn read_snft_header(
    data: &mut FontDataStream,
) -> Result<SnftTableHeader, Error> {
    let sfnt_version = data.read_u32()?;
    let num_tables = data.read_u16()?;
    let search_range = data.read_u16()?;
    let entry_selector = data.read_u16()?;
    let range_shift = data.read_u16()?;

    Ok(SnftTableHeader {
        version: sfnt_version,
        num_tables,
        search_range,
        entry_selector,
        range_shift,
    })
}

/// Extract an SNFT table entry from the data stream.
pub fn read_snft_table_entry(
    data: &mut FontDataStream,
) -> Result<SnftTableEntry, Error> {
    let tag_bytes = data.read_bytes(4)?;
    let tag = [tag_bytes[0], tag_bytes[1], tag_bytes[2], tag_bytes[3]];
    let checksum = data.read_u32()?;
    let offset = data.read_u32()?;
    let length = data.read_u32()?;

    Ok(SnftTableEntry {
        tag,
        checksum,
        offset,
        length,
    })
}

/// Validates the checksums of SNFT tables against the data in the stream.
pub fn validate_snft_tables(
    tables: Vec<SnftTableEntry>,
    stream: &mut FontDataStream,
) -> Result<(), Error> {
    // Pre-extract slices sequentially so we can validate in parallel safely.
    let mut table_slices: Vec<(&SnftTableEntry, &[u8])> = Vec::with_capacity(tables.len());
    for t in &tables {
        match stream.slice_range(t.offset as usize..(t.offset + t.length) as usize) {
            Ok(slice) => table_slices.push((t, slice)),
            Err(e) => return Err(e),
        }
    }

    // Parallel path (requires `parallel` feature / rayon)
    #[cfg(feature = "parallel")]
    {
        use rayon::prelude::*;
        let mismatches: Vec<_> = table_slices
            .par_iter()
            .filter_map(|(table, data)| {
                let computed = compute_table_checksum(table, data);
                if computed != table.checksum {
                    Some(Err(Error::Io(IoError::ChecksumMismatch {
                        table: table.tag,
                        offset: table.offset as usize,
                        expected: table.checksum,
                        found: computed,
                    })))
                } else {
                    None
                }
            })
            .collect();

        for res in mismatches {
            if let Err(e) = res {
                return Err(e);
            }
        }

        return Ok(());
    }

    // Sequential fallback
    #[cfg(not(feature = "parallel"))]
    {
        for (table, data) in table_slices.into_iter() {
            let computed = compute_table_checksum(table, data);
            if computed != table.checksum {
                return Err(Error::Io(IoError::ChecksumMismatch {
                    table: table.tag,
                    offset: table.offset as usize,
                    expected: table.checksum,
                    found: computed,
                }));
            }
        }

        Ok(())
    }
}

/// Compute checksum for a table, special-casing the `head` table to treat
/// bytes 8..12 as zero without allocating.
fn compute_table_checksum(table: &SnftTableEntry, data: &[u8]) -> u32 {
    if table.tag == [b'h', b'e', b'a', b'd'] {
        // sum 4-byte big-endian words, treating bytes 8..12 as zero
        let mut sum: u32 = 0;
        let len = data.len();
        let mut i = 0usize;
        while i < len {
            // collect 4 bytes for this word
            let mut buf = [0u8; 4];
            for j in 0..4 {
                let idx = i + j;
                if idx < len {
                    // if idx lies within checksumAdjustment (8..12) treat as 0
                    if idx >= 8 && idx < 12 {
                        buf[j] = 0;
                    } else {
                        buf[j] = data[idx];
                    }
                } else {
                    buf[j] = 0;
                }
            }
            sum = sum.wrapping_add(u32::from_be_bytes(buf));
            i += 4;
        }
        sum
    } else {
        // default path: fast iteration
        let mut sum: u32 = 0;
        let mut chunks = data.chunks(4);
        for chunk in &mut chunks {
            let mut buf = [0u8; 4];
            for i in 0..chunk.len() {
                buf[i] = chunk[i];
            }
            sum = sum.wrapping_add(u32::from_be_bytes(buf));
        }
        sum
    }
}
