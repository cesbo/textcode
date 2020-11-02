use textcode::*;


#[test]
fn test_utf8() {
    let u = "тест";
    let c: &[u8] = &[0xd1, 0x82, 0xd0, 0xb5, 0xd1, 0x81, 0xd1, 0x82];

    let mut dst = Vec::new();
    utf8::encode(u, &mut dst);
    assert_eq!(c, dst.as_slice());

    let enc = utf8::encode_to_vec(u);
    assert_eq!(enc, dst);

    let mut dst = String::new();
    utf8::decode(c, &mut dst);
    assert_eq!(u, dst.as_str());

    let dec = utf8::decode_to_string(c);
    assert_eq!(dec, dst);
}

#[test]
fn test_utf8_n_bytes() {
    let u = "n тест😹x";
    let c: &[u8] = &[
        // 1: 'n'
        0x6e,
        // 2: ' '
        0x20,
        // 3..4: 'т'
        0xd1, 0x82,
        // 5..6: 'е'
        0xd0, 0xb5,
        // 7..8: 'с'
        0xd1, 0x81,
        // 9..10: 'т'
        0xd1, 0x82,
        // 11..14: '😹'
        0xf0, 0x9f, 0x98, 0xb9,
        // 15: x
        0x78]
    ;

    let mut dst = Vec::new();
    utf8::encode(u, &mut dst);
    assert_eq!(c, dst.as_slice());

    let enc = utf8::encode_to_vec(u);
    assert_eq!(enc, dst);

    let mut dst = String::new();
    utf8::decode(c, &mut dst);
    assert_eq!(u, dst.as_str());

    let dec = utf8::decode_to_string(c);
    assert_eq!(dec, dst);
}
