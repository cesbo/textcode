//! Latin superset of ISO/IEC 6937 with addition of the Euro symbol

use std::io::Write;

use crate::{
    ENCODE_FALLBACK,
    data::{
        DECODE_MAP_ISO6937,
        ENCODE_MAP_ISO6937,
        HI_MAP_ISO6937,
    },
    write_ascii,
    write_decode_fallback,
    write_utf8,
};

fn decode_impl<W: Write, R: AsRef<[u8]>>(src: R, dst: &mut W) -> std::io::Result<usize> {
    let mut skip = 0;
    let src = src.as_ref();
    let size = src.len();
    let mut written = 0;

    while skip < size {
        let c = src[skip];

        if c <= 0x7F {
            written += write_ascii(dst, c)?;
        } else if (0xC1 ..= 0xCF).contains(&c) {
            // diactrics
            skip += 1;

            if skip >= size {
                written += write_decode_fallback(dst)?;
                break;
            }

            let map_skip = usize::from(c - 0xC1) * usize::from(b'z' - b'A' + 1) + (0x0100 - 0x00A0);
            let c = src[skip];

            if (b'A' ..= b'z').contains(&c) {
                let offset = map_skip + usize::from(c - b'A');
                let u = DECODE_MAP_ISO6937[offset];
                written += write_utf8(dst, u)?;
            } else {
                written += write_decode_fallback(dst)?;
            }
        } else if c >= 0xA0 {
            let offset = usize::from(c) - 0xA0;
            let u = DECODE_MAP_ISO6937[offset];
            written += write_utf8(dst, u)?;
        } else {
            written += write_decode_fallback(dst)?;
        }

        skip += 1;
    }

    Ok(written)
}

fn encode_impl<W: Write, R: AsRef<str>>(src: R, dst: &mut W) -> std::io::Result<usize> {
    let src = src.as_ref();
    let mut written = 0;

    for ch in src.chars() {
        let u = ch as u32;
        let mut buf = [0u8; 2];
        let n: usize;

        if u <= 0x7F {
            buf[0] = u as u8;
            n = 1;
        } else if u >= 0x00A0 && u <= 0xFFFF {
            let c = u as u16;
            let hi = (c >> 8) as usize;
            let lo = (c & 0xFF) as usize;

            let pos = HI_MAP_ISO6937[hi] * 0x100 + lo;
            let code = ENCODE_MAP_ISO6937[pos];

            if code > 0xFF {
                buf[0] = (code >> 8) as u8;
                buf[1] = (code & 0xFF) as u8;
                n = 2;
            } else if code > 0 {
                buf[0] = code as u8;
                n = 1;
            } else {
                buf[0] = ENCODE_FALLBACK;
                n = 1;
            }
        } else {
            buf[0] = ENCODE_FALLBACK;
            n = 1;
        }

        dst.write_all(&buf[.. n])?;
        written += n;
    }

    Ok(written)
}

pub fn encode(src: &str) -> Vec<u8> {
    let mut ret = Vec::new();
    let _ = encode_impl(src, &mut ret);
    ret
}

pub fn encode_to_slice(src: &str, dst: &mut [u8]) -> usize {
    let mut cursor = std::io::Cursor::new(dst);
    encode_impl(src, &mut cursor).unwrap_or(0)
}

pub fn decode(src: &[u8]) -> String {
    let mut result = String::new();
    // SAFE: writes valid UTF-8 sequences or DECODE_FALLBACK
    let dst = unsafe { result.as_mut_vec() };
    let _ = decode_impl(src, dst);
    result
}

pub fn decode_to_slice(src: &[u8], dst: &mut [u8]) -> usize {
    let mut cursor = std::io::Cursor::new(dst);
    decode_impl(src, &mut cursor).unwrap_or(0)
}
