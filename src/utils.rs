use std::io::Write;

pub const ENCODE_FALLBACK: u8 = b'?';
const DECODE_FALLBACK: &[u8] = b"\xEF\xBF\xBD"; // �

#[inline]
pub fn write_decode_fallback<W: Write>(dst: &mut W) -> std::io::Result<usize> {
    dst.write_all(DECODE_FALLBACK)?;
    Ok(DECODE_FALLBACK.len())
}

#[inline]
pub fn write_ascii<W: Write>(dst: &mut W, byte: u8) -> std::io::Result<usize> {
    dst.write_all(&[byte])?;
    Ok(1)
}

#[inline]
pub fn write_utf8<W: Write>(dst: &mut W, u: u16) -> std::io::Result<usize> {
    if u == 0 {
        write_decode_fallback(dst)
    } else if let Some(ch) = char::from_u32(u as u32) {
        let mut buf = [0u8; 4];
        let s = ch.encode_utf8(&mut buf);
        dst.write_all(s.as_bytes())?;
        Ok(s.len())
    } else {
        write_decode_fallback(dst)
    }
}
