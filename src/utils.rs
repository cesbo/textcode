use std::io::Write;

use crate::TextcodeError;

pub const ENCODE_FALLBACK: u8 = b'?';
pub const DECODE_FALLBACK: char = '\u{FFFD}'; // �

#[inline]
pub fn write_decode_fallback<W: Write>(dst: &mut W) -> Result<usize, TextcodeError> {
    let mut buf = [0u8; 4];
    let s = DECODE_FALLBACK.encode_utf8(&mut buf);

    dst.write_all(s.as_bytes()).map_err(|_| TextcodeError::Io)?;
    Ok(s.len())
}

#[inline]
pub fn write_ascii<W: Write>(dst: &mut W, byte: u8) -> Result<usize, TextcodeError> {
    dst.write_all(&[byte]).map_err(|_| TextcodeError::Io)?;
    Ok(1)
}

#[inline]
pub fn write_utf8<W: Write>(dst: &mut W, u: u16) -> Result<usize, TextcodeError> {
    let ch = if u == 0 {
        DECODE_FALLBACK
    } else {
        char::from_u32(u as u32).unwrap_or(DECODE_FALLBACK)
    };

    let mut buf = [0u8; 4];
    let s = ch.encode_utf8(&mut buf);
    dst.write_all(s.as_bytes()).map_err(|_| TextcodeError::Io)?;
    Ok(s.len())
}
