use textcode::*;


#[test]
fn test_iso6937() {
    let u = "iso6937 with € sign. ♪";
    let c: &[u8] = &[
        0x69, 0x73, 0x6f, 0x36, 0x39, 0x33, 0x37, 0x20,
        0x77, 0x69, 0x74, 0x68, 0x20, 0xa4, 0x20, 0x73,
        0x69, 0x67, 0x6e, 0x2e, 0x20, 0xd5,
    ];

    let bound = iso6937::bound(c, 10000);
    assert_eq!(bound, c.len());

    let bound = iso6937::bound(c, 5);
    assert_eq!(bound, 5);

    let mut dst: Vec<u8> = Vec::new();
    iso6937::encode(u, &mut dst);
    assert_eq!(dst.as_slice(), c);

    let enc = iso6937::encode_to_vec(u);
    assert_eq!(enc, dst);

    let mut dst = String::new();
    iso6937::decode(c, &mut dst);
    assert_eq!(u, dst.as_str());

    let dec = iso6937::decode_to_string(c);
    assert_eq!(dst, dec);
}
