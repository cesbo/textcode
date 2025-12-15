use textcode::dvb::{
    decode,
    decode_to_slice,
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
