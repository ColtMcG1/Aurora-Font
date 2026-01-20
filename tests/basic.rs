//! Basic tests for Aurora Font Library

#[test]
fn test_endianness_detection() {
    use aurora_font::io::ByteOrder;

    let data_be: [u8; 4] = [0x77, 0x1B, 0xA2, 0x4F];
    let data_le: [u8; 4] = [0x4F, 0xA2, 0x1B, 0x77];

    assert_eq!(ByteOrder::detect_from_u32_data(&data_be), Some(ByteOrder::BigEndian));
    assert_eq!(ByteOrder::detect_from_u32_data(&data_le), Some(ByteOrder::LittleEndian));

    let bom_be: [u8; 2] = [0xFE, 0xFF];
    let bom_le: [u8; 2] = [0xFF, 0xFE];
    assert_eq!(ByteOrder::detect_from_bom(&bom_be), Some(ByteOrder::BigEndian));
    assert_eq!(ByteOrder::detect_from_bom(&bom_le), Some(ByteOrder::LittleEndian));

    let num_be = ByteOrder::BigEndian.u32_from([0x00, 0x00, 0x01, 0x00]);
    let num_le = ByteOrder::LittleEndian.u32_from([0x00, 0x00, 0x01, 0x00]);
    assert_eq!(num_be, 256);
    assert_eq!(num_le, 65536);

    let slice_be: [u8; 4] = [0x00, 0x00, 0x01, 0x00];
    let slice_le: [u8; 4] = [0x00, 0x00, 0x01, 0x00];
    assert_eq!(ByteOrder::BigEndian.read_u32_from_slice(&slice_be), Some(256));
    assert_eq!(ByteOrder::LittleEndian.read_u32_from_slice(&slice_le), Some(65536));
}

#[test]
fn test_io_stream() {
    use aurora_font::io::FontDataStream;
    use aurora_font::error::{Error, IoError};

    let data: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04];
    let mut stream = FontDataStream::new(&data);
    assert_eq!(stream.position(), 0);
    assert_eq!(stream.read_bytes(2).unwrap(), &[0x00, 0x01]);
    assert_eq!(stream.position(), 2);

    stream.seek(1);
    assert_eq!(stream.position(), 1);
    assert_eq!(stream.read_bytes(3).unwrap(), &[0x01, 0x02, 0x03]);
    assert_eq!(stream.position(), 4);

    let result = stream.read_bytes(2);
    assert!(matches!(result, Err(Error::Io(IoError::OutOfBounds { requested: 2, available: 1 }))));

}

#[test]
fn test_decode_font_type() {
    use aurora_font::io::FontDataStream;
    use aurora_font::common::{decode_font_type, FontFileHeader};

    let sfnt_data: Vec<u8> = vec![0x00, 0x01, 0x00, 0x00]; // SFNT signature
    let mut sfnt_stream = FontDataStream::new(&sfnt_data);
    let header = decode_font_type(&mut sfnt_stream).unwrap();
    assert_eq!(header, FontFileHeader::SFNT);

    let otto_data: Vec<u8> = vec![0x4F, 0x54, 0x54, 0x4F]; // 'OTTO' signature
    let mut otto_stream = FontDataStream::new(&otto_data);
    let header = decode_font_type(&mut otto_stream).unwrap();
    assert_eq!(header, FontFileHeader::OTTO);

    let invalid_data: Vec<u8> = vec![0xFF, 0xFF, 0xFF, 0xFF]; // Invalid signature
    let mut invalid_stream = FontDataStream::new(&invalid_data);
    let header = decode_font_type(&mut invalid_stream).unwrap();
    assert_eq!(header, FontFileHeader::Unknown);
}

#[test]
fn test_snft_checksum_validation() {
    use aurora_font::io::FontDataStream;
    use aurora_font::common::snft::{validate_snft_tables, read_snft};

    const FONT: &[u8] = include_bytes!("../assets/test-fonts/NinetyNine.otf");
    let mut stream = FontDataStream::new(FONT);
    let table = read_snft(&mut stream).expect("Unable to parse snft");
    let result = validate_snft_tables(table.tables, &mut stream);

    assert!(matches!(result, Ok(())), "SNFT checksum validation failed: {:?}", result.err());
}

#[test]
#[cfg(feature = "full")]
fn test_decode_into() {
    use aurora_font::io::FontDataStream;
    use aurora_font::optional::builtin::Font;

    const FONT: &[u8] = include_bytes!("../assets/test-fonts/NinetyNine.otf");

    let mut sfnt_stream = FontDataStream::new(FONT);
    let sink = Font::new();
    let result = decode_into(&mut sfnt_stream, sink);
    assert!(result.is_ok(), "Decoding into Font sink failed: {:?}", result.err());
}