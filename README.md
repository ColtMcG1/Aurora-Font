# Aurora Font Library

This is a Rust library for parsing and manipulating common font formats, including TrueType (TTF) and OpenType (OTF). It provides a basic API for working with font files, allowing developers to read font metadata, glyph data, and perform various font-related operations.

See the [development](https://github.com/ColtMcG1/Aurora-Font/branches/development) branch for the latest updates and features.

## Design Goals

- Support for common font formats (TTF, OTF, ATT, WOFF, WOFF2, SVG, COLRv2).
  - TrueType and OpenType support.
  - Apple TrueType support.
  - Web Open Font Format support.
  - Scalable Vector Graphics fonts.
  - Color Vector fonts (COLRv2).
  - Type 1 font support (deprecated in 2023).
- Easy-to-use API for font manipulation.
  - Support for reading font metadata and glyph data.
  - Basic font editing capabilities.
  - Font subsetting and optimization.
- Modular design for extensibility.
  - Support for both low-level and high-level font operations.
  - Pluggable architecture for font format conversion.
- Robust error handling and reporting.
  - Clear error messages for common font parsing issues.
  - Graceful handling of malformed font files.
- Comprehensive documentation and examples.
  - Detailed API documentation.
  - Example code snippets for common use cases.
  - Tutorials for getting started with the library.
- Very few dependencies to keep the library lightweight.
  - Minimal external crates.
  - Focus on core Rust features and standard library.
  - Optimized for performance and memory usage.
  - Support for no_std environments. (planned for future releases)
- Cross-platform compatibility.
  - Support for major operating systems (Windows, macOS, Linux).
  - Consistent behavior across different platforms.
- Performance optimizations for handling large font files.
  - Efficient parsing algorithms.
  - Caching mechanisms for frequently accessed data.
  - Parallel processing support for performance improvements. (optional feature)
  - Lazy loading of font data. (planned for future releases)

## Cargo Features

- `full`: Enables all optional features for maximum functionality.
- `parrallel`: Enables parallel processing for performance improvements.

## License

This project is licensed under the Apache-2.0 License. See the [LICENSE](LICENSE) file for details.
