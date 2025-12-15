use textcode::{
    Gb2312,
    decode,
    decode_to_slice,
    encode,
    encode_to_slice,
};

#[test]
fn test_gb2312() {
    let u = "天地玄黄 宇宙洪荒 日月盈昃 辰宿列张 寒来暑往 秋收冬藏";
    let c: &[u8] = &[
        0xcc, 0xec, 0xb5, 0xd8, 0xd0, 0xfe, 0xbb, 0xc6, 0x20, 0xd3, 0xee, 0xd6, 0xe6, 0xba, 0xe9,
        0xbb, 0xc4, 0x20, 0xc8, 0xd5, 0xd4, 0xc2, 0xd3, 0xaf, 0xea, 0xbe, 0x20, 0xb3, 0xbd, 0xcb,
        0xde, 0xc1, 0xd0, 0xd5, 0xc5, 0x20, 0xba, 0xae, 0xc0, 0xb4, 0xca, 0xee, 0xcd, 0xf9, 0x20,
        0xc7, 0xef, 0xca, 0xd5, 0xb6, 0xac, 0xb2, 0xd8,
    ];

    let mut buf = [0u8; 512];

    let enc = encode::<Gb2312>(u);
    assert_eq!(enc.as_slice(), c);

    let enc_len = encode_to_slice::<Gb2312>(u, &mut buf);
    assert_eq!(enc, &buf[.. enc_len]);

    let dec = decode::<Gb2312>(c);
    assert_eq!(u, dec.as_str());

    let dec_len = decode_to_slice::<Gb2312>(c, &mut buf);
    assert_eq!(dec.as_bytes(), &buf[.. dec_len]);
}

#[test]
fn test_gb2312_issue6() {
    let expected: &str = "�";
    let c: &[u8] = &[0xf7, 0xff];

    let dec = decode::<Gb2312>(c);
    assert_eq!(expected, dec.as_str());
}
