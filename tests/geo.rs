use textcode::{
    Geo,
    decode,
    decode_to_slice,
    encode,
    encode_to_slice,
};

#[test]
fn test_geo_roundtrip() {
    // Mix of ASCII and Georgian characters
    let u = "Magti TV საქართველო";
    let c: &[u8] = &[
        0x4D, 0x61, 0x67, 0x74, 0x69, 0x20, 0x54, 0x56, 0x20, 0xE1, 0xD0, 0xE5, 0xD0, 0xE0, 0xD7,
        0xD5, 0xD4, 0xDA, 0xDD,
    ];

    let enc = encode::<Geo>(u);
    assert_eq!(enc.as_slice(), c);

    let dec = decode::<Geo>(c);
    assert_eq!(u, dec.as_str());
}

#[test]
fn test_geo_roundtrip_to_slice() {
    // Mix of ASCII and Georgian characters
    let u = "Magti TV საქართველო";
    let c: &[u8] = &[
        0x4D, 0x61, 0x67, 0x74, 0x69, 0x20, 0x54, 0x56, 0x20, 0xE1, 0xD0, 0xE5, 0xD0, 0xE0, 0xD7,
        0xD5, 0xD4, 0xDA, 0xDD,
    ];

    let mut enc_buf = vec![0u8; 32];
    let enc_size = encode_to_slice::<Geo>(u, &mut enc_buf);
    assert_eq!(&enc_buf[.. enc_size], c);

    let mut dec_buf = vec![0u8; 64];
    let dec_size = decode_to_slice::<Geo>(c, &mut dec_buf);
    assert_eq!(u.as_bytes(), &dec_buf[.. dec_size]);
}

#[test]
fn test_geo_capital_letters() {
    // Georgian capital letters Ⴀ-Ⴥ (U+10A0-U+10C5) map from 0xA0-0xC5
    let c: &[u8] = &[0xA0, 0xA1, 0xA2];
    let u = "ႠႡႢ"; // U+10A0, U+10A1, U+10A2

    let dec = decode::<Geo>(c);
    assert_eq!(u, dec.as_str());

    let enc = encode::<Geo>(u);
    assert_eq!(enc.as_slice(), c);
}

#[test]
fn test_geo_decode_null_terminator() {
    // Decoding should stop at null byte
    let c: &[u8] = &[0x48, 0x69, 0x00, 0xD2, 0xD0];
    let u = "Hi";

    let dec = decode::<Geo>(c);
    assert_eq!(u, dec.as_str());
}

#[test]
fn test_geo_decode_control_chars() {
    // Control characters 0x80-0x9F should produce fallback character
    let c: &[u8] = &[0x48, 0x80, 0x69]; // "H" + control + "i"

    let dec = decode::<Geo>(c);
    assert_eq!("H\u{FFFD}i", dec.as_str()); // Should contain replacement character
}

#[test]
fn test_geo_empty() {
    let c: &[u8] = &[];

    let dec = decode::<Geo>(c);
    assert_eq!(dec, "");

    let enc = encode::<Geo>("");
    assert!(enc.is_empty());
}

#[test]
fn test_geo_encode_unsupported() {
    // Characters outside ASCII and Georgian range should produce fallback
    let u = "Тест"; // Cyrillic text
    let enc = encode::<Geo>(u);
    assert_eq!(b"????", enc.as_slice());
}
