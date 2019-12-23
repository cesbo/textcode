use textcode::*;


#[test]
fn test_utf8() {
    let u = "тест";
    let c: &[u8] = &[0xd1, 0x82, 0xd0, 0xb5, 0xd1, 0x81, 0xd1, 0x82];

    let mut dst = String::new();
    utf8::decode(c, &mut dst);
    assert_eq!(u, dst.as_str());

    let mut dst = String::new();
    utf8::decode(c, &mut dst);
    assert_eq!(u, dst.as_str());

    let bound = utf8::bound(c, 5);
    assert_eq!(bound, 4);

    let bound = utf8::bound(c, 6);
    assert_eq!(bound, 6);

    let bound = utf8::bound(c, 1);
    assert_eq!(bound, 0);

    let bound = utf8::bound(c, 10000);
    assert_eq!(bound, c.len());
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

    let mut dst = String::new();
    utf8::decode(c, &mut dst);
    assert_eq!(u, dst.as_str());

    let mut dst = String::new();
    utf8::decode(c, &mut dst);
    assert_eq!(u, dst.as_str());

    let bound = utf8::bound(c, 5);
    assert_eq!(bound, 4);

    let bound = utf8::bound(c, 1);
    assert_eq!(bound, 1);

    let bound = utf8::bound(c, 10);
    assert_eq!(bound, 10);

    let bound = utf8::bound(c, 13);
    assert_eq!(bound, 10);

    let bound = utf8::bound(c, 14);
    assert_eq!(bound, 14);

    let bound = utf8::bound(c, 15);
    assert_eq!(bound, 15);
}

#[test]
fn test_readme() {
    // "🦀🦀"
    const UTF8_DATA: &[u8] = &[
        0xF0, 0x9F, 0xA6, 0x80, 0xF0, 0x9F, 0xA6, 0x80,
    ];

    use textcode::utf8;

    assert_eq!(utf8::bound(UTF8_DATA, 6), 4);
    assert_eq!(utf8::bound(UTF8_DATA, 1000), UTF8_DATA.len());
}
