use textcode::dvb::{
    Charset,
    decode,
    decode_to_slice,
    encode,
    encode_to_slice,
};

#[test]
fn test_dvb_iso6937_default() {
    // No prefix byte - ISO-6937
    let data = b"Hello";
    let text = decode(data);
    assert_eq!(text, "Hello");
}

#[test]
fn test_dvb_iso6937_explicit() {
    // Explicit 0x00 prefix
    let data = [0x00, b'H', b'e', b'l', b'l', b'o'];
    let text = decode(&data);
    assert_eq!(text, "Hello");
}

#[test]
fn test_dvb_iso8859_5() {
    // ISO-8859-5 (Cyrillic)
    let data = [0x01, 0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];
    let text = decode(&data);
    assert_eq!(text, "Привет!");
}

#[test]
fn test_dvb_iso8859_extended() {
    // ISO-8859-11 (Thai) via extended selection (0x10 0x00 0x0B)
    let data = [0x10, 0x00, 0x0B, 0xca, 0xc7, 0xd1, 0xca, 0xb4, 0xd5];
    let text = decode(&data);
    assert_eq!(text, "สวัสดี");
}

#[test]
fn test_dvb_utf8() {
    // UTF-8 prefix
    let data = [
        0x15, 0xe3, 0x81, 0x93, 0xe3, 0x82, 0x93, 0xe3, 0x81, 0xab, 0xe3, 0x81, 0xa1, 0xe3, 0x81,
        0xaf, 0xe4, 0xb8, 0x96, 0xe7, 0x95, 0x8c,
    ];
    let text = decode(&data);
    assert_eq!(text, "こんにちは世界");
}

#[test]
fn test_dvb_utf16be() {
    // UTF-16BE prefix
    let data = [0x11, 0x00, 0x48, 0x00, 0x69]; // "Hi"
    let text = decode(&data);
    assert_eq!(text, "Hi");
}

#[test]
fn test_dvb_decode_to_slice() {
    let data = [0x01, 0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];
    let mut buf = [0u8; 64];
    let len = decode_to_slice(&data, &mut buf);
    assert_eq!(&buf[.. len], "Привет!".as_bytes());
}

#[test]
fn test_dvb_empty() {
    let data: &[u8] = &[];
    let text = decode(data);
    assert_eq!(text, "");
}

#[test]
fn test_dvb_geo() {
    // Proprietary Georgian prefix (0x1E)
    let data = [
        0x1E, 0xE1, 0xD0, 0xE5, 0xD0, 0xE0, 0xD7, 0xD5, 0xD4, 0xDA, 0xDD,
    ];
    let text = decode(data);
    assert_eq!(text, "საქართველო");
}

#[test]
fn test_dvb_encode_iso6937() {
    // ISO-6937 - no header
    let enc = encode("Hello", Charset::Iso6937);
    assert_eq!(enc.as_slice(), b"Hello");
}

#[test]
fn test_dvb_encode_iso8859_5() {
    // Single-byte header 0x01
    let enc = encode("Привет!", Charset::Iso8859_5);
    let expected = [0x01, 0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];
    assert_eq!(enc.as_slice(), &expected);
}

#[test]
fn test_dvb_encode_iso8859_2() {
    // Extended selection header 0x10 0x00 0x02
    let enc = encode("Příliš žluťoučký", Charset::Iso8859_2);
    let expected = [
        0x10, 0x00, 0x02, 0x50, 0xf8, 0xed, 0x6c, 0x69, 0xb9, 0x20, 0xbe, 0x6c, 0x75, 0xbb, 0x6f,
        0x75, 0xe8, 0x6b, 0xfd,
    ];
    assert_eq!(enc.as_slice(), &expected);
}

#[test]
fn test_dvb_encode_utf8() {
    let enc = encode("こんにちは世界", Charset::Utf8);
    let expected = [
        0x15, 0xe3, 0x81, 0x93, 0xe3, 0x82, 0x93, 0xe3, 0x81, 0xab, 0xe3, 0x81, 0xa1, 0xe3, 0x81,
        0xaf, 0xe4, 0xb8, 0x96, 0xe7, 0x95, 0x8c,
    ];
    assert_eq!(enc.as_slice(), &expected);
}

#[test]
fn test_dvb_encode_geo() {
    let enc = encode("Magti TV საქართველო", Charset::Geo);
    let expected = [
        0x1E, 0x4D, 0x61, 0x67, 0x74, 0x69, 0x20, 0x54, 0x56, 0x20, 0xE1, 0xD0, 0xE5, 0xD0, 0xE0,
        0xD7, 0xD5, 0xD4, 0xDA, 0xDD,
    ];
    assert_eq!(enc.as_slice(), &expected);
}

#[test]
fn test_dvb_encode_roundtrip() {
    let samples: &[(Charset, &str)] = &[
        (Charset::Iso6937, "Hello, World!"),
        (Charset::Iso8859_1, "Café"),
        (Charset::Iso8859_2, "Příliš žluťoučký"),
        (Charset::Iso8859_3, "Ħabib"),
        (Charset::Iso8859_4, "Läti"),
        (Charset::Iso8859_5, "Привет!"),
        (Charset::Iso8859_6, "مرحبا"),
        (Charset::Iso8859_7, "Γειά σου"),
        (Charset::Iso8859_8, "שלום"),
        (Charset::Iso8859_9, "Merhaba dünya"),
        (Charset::Iso8859_10, "Halló"),
        (Charset::Iso8859_11, "สวัสดี"),
        (Charset::Iso8859_13, "Rīga"),
        (Charset::Iso8859_14, "Dŵr"),
        (Charset::Iso8859_15, "€uro"),
        (Charset::Iso8859_16, "România"),
        (Charset::Utf16, "こんにちは"),
        (Charset::Gb2312, "你好"),
        (Charset::Utf8, "こんにちは世界"),
        (Charset::Geo, "Magti TV საქართველო"),
    ];

    for (charset, text) in samples {
        let enc = encode(text, *charset);
        assert_eq!(decode(&enc), *text, "{:?}", charset);
    }
}

#[test]
fn test_dvb_encode_empty() {
    // Empty source writes nothing, not even the header
    let enc = encode("", Charset::Iso8859_5);
    assert!(enc.is_empty());

    let mut buf = [0u8; 8];
    let len = encode_to_slice("", Charset::Utf8, &mut buf);
    assert_eq!(len, 0);
}

#[test]
fn test_dvb_encode_to_slice() {
    let expected = [0x01, 0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];

    // Exact-size buffer: header + payload
    let mut buf = [0u8; 8];
    let len = encode_to_slice("Привет!", Charset::Iso8859_5, &mut buf);
    assert_eq!(len, 8);
    assert_eq!(&buf[.. len], &expected);

    // Buffer too small for result
    let mut small_buf = [0u8; 7];
    let len = encode_to_slice("Привет!", Charset::Iso8859_5, &mut small_buf);
    assert_eq!(len, 0);
}
