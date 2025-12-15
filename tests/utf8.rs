use textcode::*;

#[test]
fn test_utf8() {
    let u = "Тест 😀";
    let c: &[u8] = &[
        0xd0, 0xa2, 0xd0, 0xb5, 0xd1, 0x81, 0xd1, 0x82, 0x20, 0xf0, 0x9f, 0x98, 0x80,
    ];

    let enc = utf8::encode(u);
    assert_eq!(c, enc.as_slice());

    let dec = utf8::decode(c);
    assert_eq!(u, dec.as_str());
}

#[test]
fn test_utf8_n_bytes() {
    let u = "n тест😹x";
    let c: &[u8] = &[
        // 1: 'n'
        0x6e, // 2: ' '
        0x20, // 3..4: 'т'
        0xd1, 0x82, // 5..6: 'е'
        0xd0, 0xb5, // 7..8: 'с'
        0xd1, 0x81, // 9..10: 'т'
        0xd1, 0x82, // 11..14: '😹'
        0xf0, 0x9f, 0x98, 0xb9, // 15: x
        0x78,
    ];

    let enc = utf8::encode(u);
    assert_eq!(c, enc.as_slice());

    let dec = utf8::decode(c);
    assert_eq!(u, dec.as_str());
}
