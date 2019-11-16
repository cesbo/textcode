use textcode::*;


#[test]
fn test_gb2312() {
    let u = "天地玄黄 宇宙洪荒 日月盈昃 辰宿列张 寒来暑往 秋收冬藏";
    let c: &[u8] = &[
        0xcc, 0xec, 0xb5, 0xd8, 0xd0, 0xfe, 0xbb, 0xc6, 0x20, 0xd3, 0xee, 0xd6,
        0xe6, 0xba, 0xe9, 0xbb, 0xc4, 0x20, 0xc8, 0xd5, 0xd4, 0xc2, 0xd3, 0xaf,
        0xea, 0xbe, 0x20, 0xb3, 0xbd, 0xcb, 0xde, 0xc1, 0xd0, 0xd5, 0xc5, 0x20,
        0xba, 0xae, 0xc0, 0xb4, 0xca, 0xee, 0xcd, 0xf9, 0x20, 0xc7, 0xef, 0xca,
        0xd5, 0xb6, 0xac, 0xb2, 0xd8,
    ];

    let mut dst: Vec<u8> = Vec::new();
    gb2312::encode(u, &mut dst);
    assert_eq!(dst.as_slice(), c);

    let mut dst = String::new();
    gb2312::decode(c, &mut dst);
    assert_eq!(u, dst.as_str());
}
