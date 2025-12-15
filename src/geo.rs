//! Proprietary DVB single-byte Georgian character encoding.
//! ASCII (0x00–0x7F), control codes skipped (0x80–0x9F),
//! bytes 0xA0–0xFF mapped to Unicode Georgian block (U+10A0–U+10FF).

use std::io::Write;

use crate::{
    ENCODE_FALLBACK,
    Textcode,
    write_ascii,
    write_decode_fallback,
    write_utf8,
};

pub struct Geo;

impl Textcode for Geo {
    fn decode<W: Write, R: AsRef<[u8]>>(src: R, dst: &mut W) -> std::io::Result<usize> {
        let src = src.as_ref();
        let mut skip = 0;
        let size = src.len();
        let mut written = 0;

        while skip < size {
            let c = src[skip];

            if c == 0 {
                break;
            } else if c <= 0x7F {
                written += write_ascii(dst, c)?;
            } else if c >= 0xA0 {
                let u = u32::from(c) | 0x1000;
                written += write_utf8(dst, u)?;
            } else {
                written += write_decode_fallback(dst)?;
            }

            skip += 1;
        }

        Ok(written)
    }

    fn encode<W: Write, R: AsRef<str>>(src: R, dst: &mut W) -> std::io::Result<usize> {
        let src = src.as_ref();
        let mut written = 0;

        for ch in src.chars() {
            let u = ch as u32;
            let buf: [u8; 1];

            if u <= 0x7F {
                buf = [u as u8];
            } else if (0x10A0 ..= 0x10FF).contains(&u) {
                buf = [(u & 0xFF) as u8];
            } else {
                buf = [ENCODE_FALLBACK];
            }

            dst.write_all(&buf)?;
            written += 1;
        }

        Ok(written)
    }
}
