use textcode::{
    Iso8859_5,
    Iso8859_6,
    Iso8859_11,
    decode,
    decode_to_slice,
    encode,
    encode_to_slice,
};

#[test]
fn test_iso8859_5() {
    let u = "Привет!";
    let c: &[u8] = &[0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];
    let mut buf = [0u8; 512];

    let enc = encode::<Iso8859_5>(u);
    assert_eq!(enc.as_slice(), c);

    let enc_len = encode_to_slice::<Iso8859_5>(u, &mut buf);
    assert_eq!(enc, &buf[.. enc_len]);

    let dec = decode::<Iso8859_5>(c);
    assert_eq!(u, dec.as_str());

    let dec_len = decode_to_slice::<Iso8859_5>(c, &mut buf);
    assert_eq!(dec.as_bytes(), &buf[.. dec_len]);
}

#[test]
fn test_iso8859_6() {
    let u = "مرحبا";
    let c: &[u8] = &[0xe5, 0xd1, 0xcd, 0xc8, 0xc7];
    let mut buf = [0u8; 512];

    let enc = encode::<Iso8859_6>(u);
    assert_eq!(enc.as_slice(), c);

    let enc_len = encode_to_slice::<Iso8859_6>(u, &mut buf);
    assert_eq!(enc, &buf[.. enc_len]);

    let dec = decode::<Iso8859_6>(c);
    assert_eq!(u, dec.as_str());

    let dec_len = decode_to_slice::<Iso8859_6>(c, &mut buf);
    assert_eq!(dec.as_bytes(), &buf[.. dec_len]);
}

#[test]
fn test_iso8859_11() {
    let u = "มีวันที่ดี!";
    let c: &[u8] = &[
        0xc1, 0xd5, 0xc7, 0xd1, 0xb9, 0xb7, 0xd5, 0xe8, 0xb4, 0xd5, 0x21,
    ];
    let mut buf = [0u8; 512];

    let enc = encode::<Iso8859_11>(u);
    assert_eq!(enc.as_slice(), c);

    let enc_len = encode_to_slice::<Iso8859_11>(u, &mut buf);
    assert_eq!(enc, &buf[.. enc_len]);

    let dec = decode::<Iso8859_11>(c);
    assert_eq!(u, dec.as_str());

    let dec_len = decode_to_slice::<Iso8859_11>(c, &mut buf);
    assert_eq!(dec.as_bytes(), &buf[.. dec_len]);
}
