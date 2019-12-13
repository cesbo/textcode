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

    let bound = utf8::bound(c, 9);
    assert_eq!(bound, 9);
}

#[test]
fn test_utf8_n_bytes() { 
    let u = "n тест😹x";
    //              | n  |     | т         | е         | с         | т         | 😹                    | x   |
    //              | 1  | 2   | 3     4   | 5     6   | 7     8   | 9     10  | 11    12    13    14  | 15  |
    let c: &[u8] = &[0x6e, 0x20, 0xd1, 0x82, 0xd0, 0xb5, 0xd1, 0x81, 0xd1, 0x82, 0xf0, 0x9f, 0x98, 0xb9, 0x78];
    
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
