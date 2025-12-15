//! UTF-8

use std::io::Write;

use crate::{
    Textcode,
    write_decode_fallback,
};

/// UTF-8 encoding.
pub struct Utf8;

impl Textcode for Utf8 {
    fn decode<W: Write, R: AsRef<[u8]>>(src: R, dst: &mut W) -> std::io::Result<usize> {
        let src = src.as_ref();
        match std::str::from_utf8(src) {
            Ok(v) => {
                dst.write_all(v.as_bytes())?;
                Ok(v.len())
            }
            Err(_) => write_decode_fallback(dst),
        }
    }

    fn encode<W: Write, R: AsRef<str>>(src: R, dst: &mut W) -> std::io::Result<usize> {
        let src = src.as_ref();
        dst.write_all(src.as_bytes())?;
        Ok(src.len())
    }
}
