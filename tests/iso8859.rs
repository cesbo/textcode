use textcode::*;


#[test]
fn test_iso8859_5() {
    let u = "Привет!";
    let c: &[u8] = &[0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];

    let mut dst: Vec<u8> = Vec::new();
    iso8859_5::encode(u, &mut dst);
    assert_eq!(dst.as_slice(), c);

    let enc = iso8859_5::encode_to_vec(u);
    assert_eq!(enc, dst);

    let mut dst = String::new();
    iso8859_5::decode(c, &mut dst);
    assert_eq!(u, dst.as_str());

    let dec = iso8859_5::decode_to_string(c);
    assert_eq!(dec, dst);
}


#[test]
fn test_iso8859_6() {
    let u = "مرحبا";
    let c: &[u8] = &[0xe5, 0xd1, 0xcd, 0xc8, 0xc7];

    let mut dst: Vec<u8> = Vec::new();
    iso8859_6::encode(u, &mut dst);
    assert_eq!(dst.as_slice(), c);

    let enc = iso8859_6::encode_to_vec(u);
    assert_eq!(enc, dst);

    let mut dst = String::new();
    iso8859_6::decode(c, &mut dst);
    assert_eq!(u, dst.as_str());

    let dec = iso8859_6::decode_to_string(c);
    assert_eq!(dec, dst);
}


#[test]
fn test_iso8859_11() {
    let u = "มีวันที่ดี!";
    let c: &[u8] = &[0xc1, 0xd5, 0xc7, 0xd1, 0xb9, 0xb7, 0xd5, 0xe8, 0xb4, 0xd5, 0x21];

    let mut dst: Vec<u8> = Vec::new();
    iso8859_11::encode(u, &mut dst);
    assert_eq!(dst.as_slice(), c);

    let enc = iso8859_11::encode_to_vec(u);
    assert_eq!(enc, dst);

    let mut dst = String::new();
    iso8859_11::decode(c, &mut dst);
    assert_eq!(u, dst.as_str());

    let dec = iso8859_11::decode_to_string(c);
    assert_eq!(dec, dst);
}
