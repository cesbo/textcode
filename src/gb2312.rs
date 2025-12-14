//! Simplified Chinese

use std::io::Write;

use crate::{
    ENCODE_FALLBACK,
    TextcodeError,
    data::{
        DECODE_MAP_GB2312,
        ENCODE_MAP_GB2312,
        HI_MAP_GB2312,
    },
    write_ascii,
    write_decode_fallback,
    write_utf8,
};

const LO_SIZE: usize = 0x7F - 0x21;

fn decode_impl<W: Write, R: AsRef<[u8]>>(src: R, dst: &mut W) -> Result<usize, TextcodeError> {
    let mut skip = 0;
    let src = src.as_ref();
    let size = src.len();
    let mut written = 0;

    while skip < size {
        let c = src[skip];

        if c <= 0x7F {
            written += write_ascii(dst, c)?;
            skip += 1;
            continue;
        }

        skip += 1;
        if skip >= size {
            written += write_decode_fallback(dst)?;
            break;
        }

        let hi = usize::from(c) & 0x7F;
        let lo = usize::from(src[skip]) & 0x7F;
        skip += 1;

        if lo < 0x21 || hi < 0x21 {
            written += write_decode_fallback(dst)?;
            continue;
        }

        let offset = (hi - 0x21) * LO_SIZE + (lo - 0x21);

        if offset >= DECODE_MAP_GB2312.len() {
            written += write_decode_fallback(dst)?;
            continue;
        }

        let u = DECODE_MAP_GB2312[offset];
        written += write_utf8(dst, u)?;
    }

    Ok(written)
}

fn encode_impl<W: Write, R: AsRef<str>>(src: R, dst: &mut W) -> Result<usize, TextcodeError> {
    let src = src.as_ref();
    let mut written = 0;

    for ch in src.chars() {
        let u = ch as u32;
        let mut buf = [0u8; 2];
        let n: usize;

        if u <= 0x7F {
            buf[0] = u as u8;
            n = 1;
        } else if u >= 0xA0 {
            let hi = (u >> 8) as usize;
            let lo = (u & 0xFF) as usize;

            let pos = HI_MAP_GB2312[hi] * 0x100 + lo;
            let code = ENCODE_MAP_GB2312[pos];

            if code != 0x0000 {
                buf[0] = (code >> 8) as u8;
                buf[1] = (code & 0xFF) as u8;
                n = 2;
            } else {
                buf[0] = ENCODE_FALLBACK;
                n = 1;
            }
        } else {
            buf[0] = ENCODE_FALLBACK;
            n = 1;
        }

        dst.write_all(&buf[.. n]).map_err(|_| TextcodeError::Io)?;
        written += n;
    }

    Ok(written)
}

pub fn encode(src: &str) -> Result<Vec<u8>, TextcodeError> {
    let mut ret = Vec::new();
    encode_impl(src, &mut ret)?;
    Ok(ret)
}

pub fn encode_to_slice(src: &str, dst: &mut [u8]) -> usize {
    let mut cursor = std::io::Cursor::new(dst);
    encode_impl(src, &mut cursor).unwrap_or(0)
}

pub fn decode(src: &[u8]) -> Result<String, TextcodeError> {
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
