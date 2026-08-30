use textcode::dvb::{
    Charset,
    DvbText,
    DvbTextRef,
    Error,
    decode,
    decode_to_slice,
    decode_with,
    encode,
    encode_to_slice,
};

#[test]
fn test_dvb_iso6937_default() {
    // No prefix byte - ISO-6937
    let data = b"Hello";
    let text = decode(data);
    assert_eq!(text, "Hello");
}

#[test]
fn test_dvb_iso6937_explicit() {
    // Explicit 0x00 prefix
    let data = [0x00, b'H', b'e', b'l', b'l', b'o'];
    let text = decode(&data);
    assert_eq!(text, "Hello");
}

#[test]
fn test_dvb_iso8859_5() {
    // ISO-8859-5 (Cyrillic)
    let data = [0x01, 0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];
    let text = decode(&data);
    assert_eq!(text, "Привет!");
}

#[test]
fn test_dvb_iso8859_extended() {
    // ISO-8859-11 (Thai) via extended selection (0x10 0x00 0x0B)
    let data = [0x10, 0x00, 0x0B, 0xca, 0xc7, 0xd1, 0xca, 0xb4, 0xd5];
    let text = decode(&data);
    assert_eq!(text, "สวัสดี");
}

#[test]
fn test_dvb_utf8() {
    // UTF-8 prefix
    let data = [
        0x15, 0xe3, 0x81, 0x93, 0xe3, 0x82, 0x93, 0xe3, 0x81, 0xab, 0xe3, 0x81, 0xa1, 0xe3, 0x81,
        0xaf, 0xe4, 0xb8, 0x96, 0xe7, 0x95, 0x8c,
    ];
    let text = decode(&data);
    assert_eq!(text, "こんにちは世界");
}

#[test]
fn test_dvb_utf16be() {
    // UTF-16BE prefix
    let data = [0x11, 0x00, 0x48, 0x00, 0x69]; // "Hi"
    let text = decode(&data);
    assert_eq!(text, "Hi");
}

#[test]
fn test_dvb_decode_to_slice() {
    let data = [0x01, 0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];
    let mut buf = [0u8; 64];
    let len = decode_to_slice(&data, &mut buf);
    assert_eq!(&buf[.. len], "Привет!".as_bytes());
}

#[test]
fn test_dvb_empty() {
    let data: &[u8] = &[];
    let text = decode(data);
    assert_eq!(text, "");
}

#[test]
fn test_dvb_geo() {
    // Proprietary Georgian prefix (0x1E)
    let data = [
        0x1E, 0xE1, 0xD0, 0xE5, 0xD0, 0xE0, 0xD7, 0xD5, 0xD4, 0xDA, 0xDD,
    ];
    let text = decode(data);
    assert_eq!(text, "საქართველო");
}

#[test]
fn test_dvb_encode_iso6937() {
    // ISO-6937 - no header
    let enc = encode("Hello", Charset::Iso6937);
    assert_eq!(enc.as_slice(), b"Hello");
}

#[test]
fn test_dvb_encode_iso8859_5() {
    // Single-byte header 0x01
    let enc = encode("Привет!", Charset::Iso8859_5);
    let expected = [0x01, 0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];
    assert_eq!(enc.as_slice(), &expected);
}

#[test]
fn test_dvb_encode_iso8859_2() {
    // Extended selection header 0x10 0x00 0x02
    let enc = encode("Příliš žluťoučký", Charset::Iso8859_2);
    let expected = [
        0x10, 0x00, 0x02, 0x50, 0xf8, 0xed, 0x6c, 0x69, 0xb9, 0x20, 0xbe, 0x6c, 0x75, 0xbb, 0x6f,
        0x75, 0xe8, 0x6b, 0xfd,
    ];
    assert_eq!(enc.as_slice(), &expected);
}

#[test]
fn test_dvb_encode_utf8() {
    let enc = encode("こんにちは世界", Charset::Utf8);
    let expected = [
        0x15, 0xe3, 0x81, 0x93, 0xe3, 0x82, 0x93, 0xe3, 0x81, 0xab, 0xe3, 0x81, 0xa1, 0xe3, 0x81,
        0xaf, 0xe4, 0xb8, 0x96, 0xe7, 0x95, 0x8c,
    ];
    assert_eq!(enc.as_slice(), &expected);
}

#[test]
fn test_dvb_encode_geo() {
    let enc = encode("Magti TV საქართველო", Charset::Geo);
    let expected = [
        0x1E, 0x4D, 0x61, 0x67, 0x74, 0x69, 0x20, 0x54, 0x56, 0x20, 0xE1, 0xD0, 0xE5, 0xD0, 0xE0,
        0xD7, 0xD5, 0xD4, 0xDA, 0xDD,
    ];
    assert_eq!(enc.as_slice(), &expected);
}

#[test]
fn test_dvb_encode_roundtrip() {
    let samples: &[(Charset, &str)] = &[
        (Charset::Iso6937, "Hello, World!"),
        (Charset::Iso8859_1, "Café"),
        (Charset::Iso8859_2, "Příliš žluťoučký"),
        (Charset::Iso8859_3, "Ħabib"),
        (Charset::Iso8859_4, "Läti"),
        (Charset::Iso8859_5, "Привет!"),
        (Charset::Iso8859_6, "مرحبا"),
        (Charset::Iso8859_7, "Γειά σου"),
        (Charset::Iso8859_8, "שלום"),
        (Charset::Iso8859_9, "Merhaba dünya"),
        (Charset::Iso8859_10, "Halló"),
        (Charset::Iso8859_11, "สวัสดี"),
        (Charset::Iso8859_13, "Rīga"),
        (Charset::Iso8859_14, "Dŵr"),
        (Charset::Iso8859_15, "€uro"),
        (Charset::Iso8859_16, "România"),
        (Charset::Utf16, "こんにちは"),
        (Charset::Gb2312, "你好"),
        (Charset::Utf8, "こんにちは世界"),
        (Charset::Geo, "Magti TV საქართველო"),
    ];

    for (charset, text) in samples {
        let enc = encode(text, *charset);
        assert_eq!(decode(&enc), *text, "{:?}", charset);
    }
}

#[test]
fn test_dvb_encode_empty() {
    // Empty source writes nothing, not even the header
    let enc = encode("", Charset::Iso8859_5);
    assert!(enc.is_empty());

    let mut buf = [0u8; 8];
    let len = encode_to_slice("", Charset::Utf8, &mut buf);
    assert_eq!(len, 0);
}

#[test]
fn test_dvb_encode_to_slice() {
    let expected = [0x01, 0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];

    // Exact-size buffer: header + payload
    let mut buf = [0u8; 8];
    let len = encode_to_slice("Привет!", Charset::Iso8859_5, &mut buf);
    assert_eq!(len, 8);
    assert_eq!(&buf[.. len], &expected);

    // Buffer too small for result
    let mut small_buf = [0u8; 7];
    let len = encode_to_slice("Привет!", Charset::Iso8859_5, &mut small_buf);
    assert_eq!(len, 0);
}

#[test]
fn test_dvb_from_header_table() {
    // Single-byte selectors
    let rows: &[(u8, Charset)] = &[
        (0x01, Charset::Iso8859_5),
        (0x02, Charset::Iso8859_6),
        (0x03, Charset::Iso8859_7),
        (0x04, Charset::Iso8859_8),
        (0x05, Charset::Iso8859_9),
        (0x06, Charset::Iso8859_10),
        (0x07, Charset::Iso8859_11),
        (0x09, Charset::Iso8859_13),
        (0x0A, Charset::Iso8859_14),
        (0x0B, Charset::Iso8859_15),
        (0x11, Charset::Utf16),
        (0x13, Charset::Gb2312),
        (0x15, Charset::Utf8),
        (0x1E, Charset::Geo),
    ];
    for &(sel, charset) in rows {
        let data = [sel, 0x41, 0x42];
        assert_eq!(
            Charset::from_header(&data),
            Ok((charset, &data[1 ..])),
            "selector {:#04X}",
            sel
        );
    }

    // Three-byte ISO-8859-X selectors
    let rows: &[(u8, Charset)] = &[
        (0x01, Charset::Iso8859_1),
        (0x02, Charset::Iso8859_2),
        (0x03, Charset::Iso8859_3),
        (0x04, Charset::Iso8859_4),
        (0x05, Charset::Iso8859_5),
        (0x06, Charset::Iso8859_6),
        (0x07, Charset::Iso8859_7),
        (0x08, Charset::Iso8859_8),
        (0x09, Charset::Iso8859_9),
        (0x0A, Charset::Iso8859_10),
        (0x0B, Charset::Iso8859_11),
        (0x0D, Charset::Iso8859_13),
        (0x0E, Charset::Iso8859_14),
        (0x0F, Charset::Iso8859_15),
        (0x10, Charset::Iso8859_16),
    ];
    for &(part, charset) in rows {
        let data = [0x10, 0x00, part, 0x41, 0x42];
        assert_eq!(
            Charset::from_header(&data),
            Ok((charset, &data[3 ..])),
            "part {:#04X}",
            part
        );
    }
}

#[test]
fn test_dvb_from_header_iso6937() {
    // Empty input
    assert_eq!(Charset::from_header(&[]), Ok((Charset::Iso6937, &[][..])));

    // Explicit 0x00 selector is stripped
    let data = [0x00, b'H', b'i'];
    assert_eq!(Charset::from_header(&data), Ok((Charset::Iso6937, &data[1 ..])));

    // 0x20 and above: the first byte is text
    let data = [0x20, b'H', b'i'];
    assert_eq!(Charset::from_header(&data), Ok((Charset::Iso6937, &data[..])));
    let data = [0xFF, b'H', b'i'];
    assert_eq!(Charset::from_header(&data), Ok((Charset::Iso6937, &data[..])));
}

#[test]
fn test_dvb_from_header_errors() {
    // 0x10 needs two more bytes
    assert_eq!(Charset::from_header(&[0x10]), Err(Error::InvalidLength));
    assert_eq!(Charset::from_header(&[0x10, 0x00]), Err(Error::InvalidLength));

    // Unassigned ISO-8859-X parts
    assert_eq!(Charset::from_header(&[0x10, 0x00, 0x0C]), Err(Error::InvalidCodepage));
    assert_eq!(Charset::from_header(&[0x10, 0x00, 0x00]), Err(Error::InvalidCodepage));
    assert_eq!(Charset::from_header(&[0x10, 0x00, 0x11]), Err(Error::InvalidCodepage));
    assert_eq!(Charset::from_header(&[0x10, 0x01, 0x01]), Err(Error::InvalidCodepage));

    // Undefined selectors
    for sel in [0x08, 0x0C, 0x0D, 0x0E, 0x0F, 0x12, 0x14, 0x16, 0x1D, 0x1F] {
        assert_eq!(
            Charset::from_header(&[sel, 0x41]),
            Err(Error::InvalidCodepage),
            "selector {:#04X}",
            sel
        );
    }

    // Selector errors make the whole decode empty
    assert_eq!(decode([0x1F, b'H', b'i']), "");
    let mut buf = [0u8; 8];
    assert_eq!(decode_to_slice([0x10, 0x00], &mut buf), 0);
}

#[test]
fn test_dvb_error_display() {
    assert_eq!(Error::InvalidLength.to_string(), "Invalid length of input data");
    assert_eq!(Error::InvalidCodepage.to_string(), "Invalid or unsupported codepage");
    let _: &dyn std::error::Error = &Error::InvalidLength;
}

#[test]
fn test_dvb_decode_with() {
    // Headerless bytes decoded with an explicit charset
    let cyr = [0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];
    assert_eq!(decode_with(cyr, Charset::Iso8859_5), "Привет!");
    assert_eq!(decode_with(b"Hello", Charset::Iso6937), "Hello");
    assert_eq!(decode_with([0x00, 0x48, 0x00, 0x69], Charset::Utf16), "Hi");
    assert_eq!(decode_with("こんにちは".as_bytes(), Charset::Utf8), "こんにちは");
    assert_eq!(decode_with([0xE1, 0xD0, 0xE5], Charset::Geo), "საქ");
    assert_eq!(decode_with(b"", Charset::Gb2312), "");

    // No header interpretation: a selector byte is treated as text
    assert_eq!(decode_with([0x15, 0x41], Charset::Iso8859_1), "\u{15}A");
}

#[test]
fn test_dvb_text_display() {
    assert_eq!(DvbText::from(&b"Menu"[..]).to_string(), "Menu");
    assert_eq!(DvbText::default().to_string(), "");
    // A truncated charset selector is undecodable and displays as empty
    assert_eq!(DvbText::from(&[0x10][..]).to_string(), "");
    assert_eq!(DvbText::from(vec![0x1F, b'X']).to_string(), "");
    assert_eq!(
        DvbText::from(vec![0x01, 0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21]).to_string(),
        "Привет!"
    );
}

#[test]
fn test_dvb_text_debug() {
    assert_eq!(format!("{:?}", DvbText::from(&b"Menu"[..])), "\"Menu\"");
    assert_eq!(format!("{:?}", DvbText::default()), "\"\"");
    assert_eq!(format!("{:?}", DvbText::from(&[0x10][..])), "DvbText([10])");
    assert_eq!(format!("{:?}", DvbText::from(&[0x1F, 0xAB][..])), "DvbText([1F, AB])");
}

#[test]
fn test_dvb_text_bytes() {
    let text = DvbText::from(vec![0x15, b'a']);
    assert_eq!(text.as_bytes(), &[0x15, b'a']);
    assert!(!text.is_empty());
    assert_eq!(text.clone().into_bytes(), vec![0x15, b'a']);

    let empty = DvbText::default();
    assert!(empty.is_empty());
    assert_eq!(empty.as_bytes(), &[]);
    assert_eq!(empty, DvbText::from(Vec::new()));

    // Raw bytes are kept even when undecodable
    let raw = DvbText::from(&[0x1F, 0x00][..]);
    assert_eq!(raw.as_bytes(), &[0x1F, 0x00]);
    assert_eq!(raw.as_text(), Err(Error::InvalidCodepage));
}

#[test]
fn test_dvb_text_encode_roundtrip() {
    let text = DvbText::encode("Привет!", Charset::Iso8859_5);
    assert_eq!(text.as_bytes(), &[0x01, 0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21]);
    assert_eq!(text.to_string(), "Привет!");
    let text_ref = text.as_text().unwrap();
    assert_eq!(text_ref.charset(), Charset::Iso8859_5);
    assert_eq!(text_ref.to_text(), text);

    let text = DvbText::encode("Hello", Charset::Iso6937);
    assert_eq!(text.as_bytes(), b"Hello");
    assert_eq!(text.to_string(), "Hello");

    // Empty string encodes to no bytes at all
    let text = DvbText::encode("", Charset::Utf8);
    assert!(text.is_empty());
    assert_eq!(text, DvbText::default());
}

#[test]
fn test_dvb_text_ref_utf16() {
    let data = [0x11, 0x00, 0x48, 0x00, 0x69];
    let text = DvbTextRef::try_from(&data[..]).unwrap();
    assert_eq!(text.charset(), Charset::Utf16);
    assert_eq!(text.as_bytes(), &data);
    assert_eq!(text.text(), &data[1 ..]);
    assert_eq!(text.decode(), "Hi");
    assert_eq!(text.to_string(), "Hi");
    assert_eq!(format!("{:?}", text), "\"Hi\"");
    assert_eq!(text.to_text(), DvbText::from(&data[..]));
}

#[test]
fn test_dvb_text_ref_iso6937_no_header() {
    let data = b"Hello";
    let text = DvbTextRef::try_from(&data[..]).unwrap();
    assert_eq!(text.charset(), Charset::Iso6937);
    assert_eq!(text.as_bytes(), data);
    assert_eq!(text.text(), data);
    assert_eq!(text.decode(), "Hello");

    // Explicit 0x00 selector is stripped from text()
    let data = [0x00, b'H', b'i'];
    let text = DvbTextRef::try_from(&data[..]).unwrap();
    assert_eq!(text.charset(), Charset::Iso6937);
    assert_eq!(text.as_bytes(), &data);
    assert_eq!(text.text(), b"Hi");
    assert_eq!(text.decode(), "Hi");

    // Empty input
    let text = DvbTextRef::try_from(&[][..]).unwrap();
    assert_eq!(text.charset(), Charset::Iso6937);
    assert_eq!(text.text(), &[]);
    assert_eq!(text.decode(), "");
}

#[test]
fn test_dvb_text_ref_decode_with() {
    // Three-byte selector, payload re-read with another charset
    let data = [0x10, 0x00, 0x01, 0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];
    let text = DvbTextRef::try_from(&data[..]).unwrap();
    assert_eq!(text.charset(), Charset::Iso8859_1);
    assert_eq!(text.text(), &data[3 ..]);
    assert_eq!(text.decode_with(Charset::Iso8859_5), "Привет!");
    assert_ne!(text.decode(), "Привет!");

    // No-header text re-read with another charset keeps the first byte
    let data = [0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];
    let text = DvbTextRef::try_from(&data[..]).unwrap();
    assert_eq!(text.decode_with(Charset::Iso8859_5), "Привет!");
}

#[test]
fn test_dvb_text_ref_errors() {
    assert_eq!(DvbTextRef::try_from(&[0x10][..]).unwrap_err(), Error::InvalidLength);
    assert_eq!(DvbTextRef::try_from(&[0x10, 0x00, 0x0C][..]).unwrap_err(), Error::InvalidCodepage);
    assert_eq!(DvbTextRef::try_from(&[0x1F, b'A'][..]).unwrap_err(), Error::InvalidCodepage);
}
