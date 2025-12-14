//! UTF-8

use std::io::Write;

use crate::{
    TextcodeError,
    write_decode_fallback,
};

fn decode_impl<W: Write, R: AsRef<[u8]>>(src: R, dst: &mut W) -> Result<usize, TextcodeError> {
    let src = src.as_ref();

    match std::str::from_utf8(src) {
        Ok(v) => {
            dst.write_all(v.as_bytes()).map_err(|_| TextcodeError::Io)?;
            Ok(v.len())
        }
        Err(_) => write_decode_fallback(dst),
    }
}

fn encode_impl<W: Write, R: AsRef<str>>(src: R, dst: &mut W) -> Result<usize, TextcodeError> {
    let src = src.as_ref();
    dst.write_all(src.as_bytes())
        .map_err(|_| TextcodeError::Io)?;
    Ok(src.len())
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
