//! UTF-8

use std::io::Write;

use crate::{
    Textcode,
    write_decode_fallback,
};

fn decode_inner<W: Write>(src: &[u8], dst: &mut W) -> std::io::Result<usize> {
    match std::str::from_utf8(src) {
        Ok(v) => {
            dst.write_all(v.as_bytes())?;
            Ok(v.len())
        }
        Err(_) => write_decode_fallback(dst),
    }
}

fn encode_inner<W: Write>(src: &str, dst: &mut W) -> std::io::Result<usize> {
    dst.write_all(src.as_bytes())?;
    Ok(src.len())
}

/// UTF-8 encoding.
pub struct Utf8;

impl Textcode for Utf8 {
    fn decode<W: Write, R: AsRef<[u8]>>(src: R, dst: &mut W) -> std::io::Result<usize> {
        decode_inner(src.as_ref(), dst)
    }

    fn encode<W: Write, R: AsRef<str>>(src: R, dst: &mut W) -> std::io::Result<usize> {
        encode_inner(src.as_ref(), dst)
    }
}
