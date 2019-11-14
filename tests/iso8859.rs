use textcode::*;


#[test]
fn test_iso8859_5() {
    let u = "Привет!";
    let c: &[u8] = &[0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];

    let mut dst: Vec<u8> = Vec::new();
    iso8859_5::encode(u, &mut dst);
    assert_eq!(dst.as_slice(), c);

    let mut dst = String::new();
    iso8859_5::decode(c, &mut dst);
    assert_eq!(u, dst.as_str());
}
