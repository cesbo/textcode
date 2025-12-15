//! Simplified Chinese

use std::io::Write;

use crate::{
    ENCODE_FALLBACK,
    Textcode,
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

/// Simplified Chinese encoding.
pub struct Gb2312;

impl Textcode for Gb2312 {
    fn decode<W: Write, R: AsRef<[u8]>>(src: R, dst: &mut W) -> std::io::Result<usize> {
        let src = src.as_ref();
        let mut skip = 0;
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

            let u = DECODE_MAP_GB2312[offset] as u32;
            written += write_utf8(dst, u)?;
        }

        Ok(written)
    }

    fn encode<W: Write, R: AsRef<str>>(src: R, dst: &mut W) -> std::io::Result<usize> {
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

            dst.write_all(&buf[.. n])?;
            written += n;
        }

        Ok(written)
    }
}
