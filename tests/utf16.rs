use textcode::{
    Utf16,
    decode,
    encode,
};

#[test]
fn test_utf16_basic() {
    let u = "Hello";
    let c: &[u8] = &[
        0x00, 0x48, // H
        0x00, 0x65, // e
        0x00, 0x6c, // l
        0x00, 0x6c, // l
        0x00, 0x6f, // o
    ];

    let dec = decode::<Utf16>(c);
    assert_eq!(u, dec.as_str());

    let enc = encode::<Utf16>(u);
    assert_eq!(c, enc.as_slice());
}

#[test]
fn test_utf16be_surrogate_pair() {
    // Emoji 😀 (U+1F600) requires surrogate pair
    let u = "😀";
    let c: &[u8] = &[
        0xd8, 0x3d, // high surrogate
        0xde, 0x00, // low surrogate
    ];

    let enc = encode::<Utf16>(u);
    assert_eq!(c, enc.as_slice());

    let dec = decode::<Utf16>(c);
    assert_eq!(u, dec.as_str());
}
