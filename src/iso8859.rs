use std::io::Write;

use crate::{
    ENCODE_FALLBACK,
    TextcodeError,
    write_ascii,
    write_utf8,
};

fn iso8859_decode_impl<W: Write, R: AsRef<[u8]>>(
    src: R,
    dst: &mut W,
    map: &[u16],
) -> Result<usize, TextcodeError> {
    let src = src.as_ref();
    let mut written = 0;

    for &c in src {
        if c <= 0x7F {
            written += write_ascii(dst, c)?;
        } else if c >= 0xA0 {
            let offset = c as usize - 0xA0;
            let u = map[offset];
            written += write_utf8(dst, u)?;
        }
    }

    Ok(written)
}

fn iso8859_encode_impl<W: Write, R: AsRef<str>>(
    src: R,
    dst: &mut W,
    hi_map: &[usize],
    map: &[u8],
) -> Result<usize, TextcodeError> {
    let src = src.as_ref();
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

        dst.write_all(&[c]).map_err(|_| TextcodeError::Io)?;
        written += 1;
    }

    Ok(written)
}

macro_rules! iso8859 {
    ( $($name: ident, $decode_map: ident, $hi_map: ident, $encode_map: ident),* ) => {
        $(
            pub mod $name {
                use std::io::Write;
                use crate::TextcodeError;
                use crate::data::{
                    $decode_map,
                    $hi_map,
                    $encode_map,
                };

                fn decode_impl<W: Write, R: AsRef<[u8]>>(
                    src: R,
                    dst: &mut W,
                ) -> Result<usize, TextcodeError> {
                    crate::iso8859::iso8859_decode_impl(src, dst, &$decode_map)
                }

                fn encode_impl<W: Write, R: AsRef<str>>(
                    src: R,
                    dst: &mut W,
                ) -> Result<usize, TextcodeError> {
                    crate::iso8859::iso8859_encode_impl(src, dst, &$hi_map, &$encode_map)
                }

                pub fn decode(src: &[u8]) -> Result<String, crate::TextcodeError> {
                    let mut result = String::new();
                    // SAFE: writes valid UTF-8 sequences or DECODE_FALLBACK
                    let dst = unsafe { result.as_mut_vec() };
                    decode_impl(src, dst)?;
                    Ok(result)
                }

                pub fn decode_to_slice(src: &[u8], dst: &mut [u8]) -> usize {
                    let mut cursor = std::io::Cursor::new(dst);
                    decode_impl(src, &mut cursor).unwrap_or(0)
                }

                pub fn encode(src: &str) -> Result<Vec<u8>, crate::TextcodeError> {
                    let mut ret = Vec::new();
                    encode_impl(src, &mut ret)?;
                    Ok(ret)
                }

                pub fn encode_to_slice(src: &str, dst: &mut [u8]) -> usize {
                    let mut cursor = std::io::Cursor::new(dst);
                    encode_impl(src, &mut cursor).unwrap_or(0)
                }
            }
        )*
    }
}

iso8859!(
    // Western European
    iso8859_1,
    DECODE_MAP_1,
    HI_MAP_1,
    ENCODE_MAP_1,
    // Central European
    iso8859_2,
    DECODE_MAP_2,
    HI_MAP_2,
    ENCODE_MAP_2,
    // South European
    iso8859_3,
    DECODE_MAP_3,
    HI_MAP_3,
    ENCODE_MAP_3,
    // North European
    iso8859_4,
    DECODE_MAP_4,
    HI_MAP_4,
    ENCODE_MAP_4,
    // Cyrillic
    iso8859_5,
    DECODE_MAP_5,
    HI_MAP_5,
    ENCODE_MAP_5,
    // Arabic
    iso8859_6,
    DECODE_MAP_6,
    HI_MAP_6,
    ENCODE_MAP_6,
    // Greek
    iso8859_7,
    DECODE_MAP_7,
    HI_MAP_7,
    ENCODE_MAP_7,
    // Hebrew
    iso8859_8,
    DECODE_MAP_8,
    HI_MAP_8,
    ENCODE_MAP_8,
    // Turkish
    iso8859_9,
    DECODE_MAP_9,
    HI_MAP_9,
    ENCODE_MAP_9,
    // Nordic
    iso8859_10,
    DECODE_MAP_10,
    HI_MAP_10,
    ENCODE_MAP_10,
    // Thai
    iso8859_11,
    DECODE_MAP_11,
    HI_MAP_11,
    ENCODE_MAP_11,
    // Baltic Rim
    iso8859_13,
    DECODE_MAP_13,
    HI_MAP_13,
    ENCODE_MAP_13,
    // Celtic
    iso8859_14,
    DECODE_MAP_14,
    HI_MAP_14,
    ENCODE_MAP_14,
    // Western European
    iso8859_15,
    DECODE_MAP_15,
    HI_MAP_15,
    ENCODE_MAP_15,
    // South-Eastern European
    iso8859_16,
    DECODE_MAP_16,
    HI_MAP_16,
    ENCODE_MAP_16
);
