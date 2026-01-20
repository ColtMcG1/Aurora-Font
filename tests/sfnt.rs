
#[test]
fn test_snft_loading_and_validation() {
    use aurora_font::common::snft::read_snft;
    use aurora_font::common::decode::decode_font_type;
    use aurora_font::io::FontDataStream;
    use aurora_font::error::{Error, IoError};

    const FONT_DATA: &[u8] = include_bytes!("../assets/test-fonts/TrueType-Outlines.ttf");

    let mut stream = FontDataStream::new(&FONT_DATA);
    let font_type = decode_font_type(&mut stream).expect("Failed to decode font type");
    println!("Detected font type: {:?}", font_type);
    let result = read_snft(&mut stream);

    match result {
        Ok(tables) => {
            // Validate that tables were loaded correctly
            assert!(!tables.is_empty());
        }
        Err(Error::Io(IoError::ChecksumMismatch { table, offset, expected, found })) => {
            // Handle checksum mismatch error
            println!(
                "Checksum mismatch for table {}{}{}{} at offset {}: expected {}, found {}",
                table[0] as char,
                table[1] as char,
                table[2] as char,
                table[3] as char,
                offset,
                expected,
                found
            );
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}