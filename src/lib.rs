//! Aurora Font Library
//! 
//! Author: Colton McGraw <github.com/ColtMcG1>
//! License: Apache-2.0
//! Date: January 2026
//! 
//! This library provides functionality for parsing and manipulating font files.

pub mod common;
pub mod error;
pub mod io;

/// Interface for high-level font data conversion and manipulation (opt-in).
#[cfg(feature = "full")]
pub mod font;