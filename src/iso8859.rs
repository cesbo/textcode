use std::io::Write;

use crate::{
    ENCODE_FALLBACK,
    Textcode,
    write_ascii,
    write_utf8,
};

fn decode_inner<W: Write>(src: &[u8], dst: &mut W, map: &[u16]) -> std::io::Result<usize> {
    let mut written = 0;

    for &c in src {
        if c <= 0x7F {
            written += write_ascii(dst, c)?;
        } else if c >= 0xA0 {
            let offset = c as usize - 0xA0;
            let u = map[offset] as u32;
            written += write_utf8(dst, u)?;
        }
    }

    Ok(written)
}

fn encode_inner<W: Write>(
    src: &str,
    dst: &mut W,
    hi_map: &[usize],
    map: &[u8],
) -> std::io::Result<usize> {
    let mut written = 0;

    for ch in src.chars() {
        let u = u32::from(ch) as u16;
        let c;

        if u <= 0x7F {
            c = u as u8;
        } else if u >= 0xA0 {
            let hi = usize::from(u >> 8);
            let lo = usize::from(u & 0xFF);

            let pos = hi_map[hi] * 0x100 + lo;
            let code = map[pos];

            if code != 0x0000 {
                c = code;
            } else {
                c = ENCODE_FALLBACK;
            }
        } else {
            c = ENCODE_FALLBACK;
        }

        dst.write_all(&[c])?;
        written += 1;
    }

    Ok(written)
}

macro_rules! iso8859 {
    ( $($struct_name: ident, $decode_map: ident, $hi_map: ident, $encode_map: ident),* ) => {
        $(
            /// ISO-8859 encoding implementation.
            pub struct $struct_name;

            impl Textcode for $struct_name {
                fn decode<W: Write, R: AsRef<[u8]>>(src: R, dst: &mut W) -> std::io::Result<usize> {
                    decode_inner(src.as_ref(), dst, &crate::data::$decode_map)
                }

                fn encode<W: Write, R: AsRef<str>>(src: R, dst: &mut W) -> std::io::Result<usize> {
                    encode_inner(src.as_ref(), dst, &crate::data::$hi_map, &crate::data::$encode_map)
                }
            }
        )*
    }
}

iso8859!(
    // Western European
    Iso8859_1,
    DECODE_MAP_1,
    HI_MAP_1,
    ENCODE_MAP_1,
    // Central European
    Iso8859_2,
    DECODE_MAP_2,
    HI_MAP_2,
    ENCODE_MAP_2,
    // South European
    Iso8859_3,
    DECODE_MAP_3,
    HI_MAP_3,
    ENCODE_MAP_3,
    // North European
    Iso8859_4,
    DECODE_MAP_4,
    HI_MAP_4,
    ENCODE_MAP_4,
    // Cyrillic
    Iso8859_5,
    DECODE_MAP_5,
    HI_MAP_5,
    ENCODE_MAP_5,
    // Arabic
    Iso8859_6,
    DECODE_MAP_6,
    HI_MAP_6,
    ENCODE_MAP_6,
    // Greek
    Iso8859_7,
    DECODE_MAP_7,
    HI_MAP_7,
    ENCODE_MAP_7,
    // Hebrew
    Iso8859_8,
    DECODE_MAP_8,
    HI_MAP_8,
    ENCODE_MAP_8,
    // Turkish
    Iso8859_9,
    DECODE_MAP_9,
    HI_MAP_9,
    ENCODE_MAP_9,
    // Nordic
    Iso8859_10,
    DECODE_MAP_10,
    HI_MAP_10,
    ENCODE_MAP_10,
    // Thai
    Iso8859_11,
    DECODE_MAP_11,
    HI_MAP_11,
    ENCODE_MAP_11,
    // Baltic Rim
    Iso8859_13,
    DECODE_MAP_13,
    HI_MAP_13,
    ENCODE_MAP_13,
    // Celtic
    Iso8859_14,
    DECODE_MAP_14,
    HI_MAP_14,
    ENCODE_MAP_14,
    // Western European
    Iso8859_15,
    DECODE_MAP_15,
    HI_MAP_15,
    ENCODE_MAP_15,
    // South-Eastern European
    Iso8859_16,
    DECODE_MAP_16,
    HI_MAP_16,
    ENCODE_MAP_16
);
