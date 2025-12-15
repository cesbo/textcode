//! UTF-16

use std::io::Write;

use crate::{
    Textcode,
    write_decode_fallback,
    write_utf8,
};

pub struct Utf16;

impl Textcode for Utf16 {
    fn decode<W: Write, R: AsRef<[u8]>>(src: R, dst: &mut W) -> std::io::Result<usize> {
        let src = src.as_ref();
        let mut written = 0;

        // Optional BOM
        let (be, mut i) = if src.len() >= 2 {
            match (src[0], src[1]) {
                (0xFE, 0xFF) => (true, 2),
                (0xFF, 0xFE) => (false, 2),
                _ => (true, 0), // default BE
            }
        } else {
            (true, 0)
        };

        while i < src.len() {
            if src.len() - i < 2 {
                written += write_decode_fallback(dst)?; // odd tail -> fallback
                break;
            }

            let u1 = if be {
                u16::from_be_bytes([src[i], src[i + 1]])
            } else {
                u16::from_le_bytes([src[i], src[i + 1]])
            };
            i += 2;

            let cp: u32;

            if (0xD800 ..= 0xDBFF).contains(&u1) {
                // high surrogate: need low surrogate
                if i + 2 > src.len() {
                    written += write_decode_fallback(dst)?;
                    break;
                }

                let u2 = if be {
                    u16::from_be_bytes([src[i], src[i + 1]])
                } else {
                    u16::from_le_bytes([src[i], src[i + 1]])
                };

                if (0xDC00 ..= 0xDFFF).contains(&u2) {
                    i += 2;
                    cp = 0x10000 + (((u1 as u32 - 0xD800) << 10) | (u2 as u32 - 0xDC00));
                } else {
                    written += write_decode_fallback(dst)?;
                    continue;
                }
            } else if (0xDC00 ..= 0xDFFF).contains(&u1) {
                // orphan low surrogate
                written += write_decode_fallback(dst)?;
                continue;
            } else {
                cp = u1 as u32;
            };

            written += write_utf8(dst, cp)?;
        }

        Ok(written)
    }

    fn encode<W: Write, R: AsRef<str>>(src: R, dst: &mut W) -> std::io::Result<usize> {
        // UTF-16BE, without BOM
        let src = src.as_ref();
        let mut written = 0usize;
        for u in src.encode_utf16() {
            dst.write_all(&u.to_be_bytes())?;
            written += 2;
        }
        Ok(written)
    }
}
