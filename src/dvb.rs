//! DVB text encoding and decoding module.
//!
//! Implements text encoding and decoding according to ETSI EN 300 468 specification. The first byte
//! of the text determines the character encoding.
//!
//! Character code table selection:
//! - `0x01`-`0x0B`: ISO-8859-5 to ISO-8859-15
//! - `0x10`: ISO-8859-X (next 2 bytes specify the part number)
//! - `0x11`: UTF-16BE
//! - `0x13`: GB2312
//! - `0x15`: UTF-8
//! - `0x1E`: proprietary Georgian mapping (not part of EN 300 468)
//! - `0x00` or `0x20`-`0xFF`: ISO-6937 (default)

use std::io::Write;

use crate::{
    Gb2312,
    Geo,
    Iso6937,
    Iso8859_1,
    Iso8859_2,
    Iso8859_3,
    Iso8859_4,
    Iso8859_5,
    Iso8859_6,
    Iso8859_7,
    Iso8859_8,
    Iso8859_9,
    Iso8859_10,
    Iso8859_11,
    Iso8859_13,
    Iso8859_14,
    Iso8859_15,
    Iso8859_16,
    Textcode,
    Utf8,
    Utf16,
};

/// Character set selection for DVB text encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Charset {
    /// Default table, encoded without a header; text must not start with a character below U+0020
    /// (such bytes are selector codes on the wire)
    Iso6937,
    Iso8859_1,
    Iso8859_2,
    Iso8859_3,
    Iso8859_4,
    Iso8859_5,
    Iso8859_6,
    Iso8859_7,
    Iso8859_8,
    Iso8859_9,
    Iso8859_10,
    Iso8859_11,
    Iso8859_13,
    Iso8859_14,
    Iso8859_15,
    /// Selector `0x10 0x00 0x10` is beyond the EN 300 468 table A.4 assigned range; spec-strict
    /// third-party decoders may reject it
    Iso8859_16,
    Utf16,
    Gb2312,
    Utf8,
    /// Proprietary Georgian mapping, selector `0x1E` (not part of EN 300 468)
    Geo,
}

/// Decodes DVB text to a UTF-8 String.
///
/// The encoding is determined by the first byte according to ETSI EN 300 468.
pub fn decode(src: impl AsRef<[u8]>) -> String {
    let mut result = String::new();
    // SAFE: writes valid UTF-8 sequences or replacement character
    let dst = unsafe { result.as_mut_vec() };
    let _ = decode_inner(src.as_ref(), dst);
    result
}

/// Decodes DVB text to a UTF-8 slice.
///
/// Returns the number of bytes written to the destination slice.
pub fn decode_to_slice(src: impl AsRef<[u8]>, dst: &mut [u8]) -> usize {
    let mut cursor = std::io::Cursor::new(dst);
    decode_inner(src.as_ref(), &mut cursor).unwrap_or(0)
}

/// Encodes a UTF-8 string to DVB text with the charset header.
///
/// The header is emitted according to ETSI EN 300 468 Annex A;
/// see [`Charset`] for the non-standard exceptions. Empty input
/// produces no bytes at all.
pub fn encode(src: impl AsRef<str>, charset: Charset) -> Vec<u8> {
    let mut ret = Vec::new();
    let _ = encode_inner(src.as_ref(), charset, &mut ret);
    ret
}

/// Encodes a UTF-8 string to a DVB text slice with the charset header.
///
/// Returns the number of bytes written including the header.
pub fn encode_to_slice(src: impl AsRef<str>, charset: Charset, dst: &mut [u8]) -> usize {
    let mut cursor = std::io::Cursor::new(dst);
    encode_inner(src.as_ref(), charset, &mut cursor).unwrap_or(0)
}

fn encode_with_header<T: Textcode, W: Write>(
    src: &str,
    header: &[u8],
    dst: &mut W,
) -> std::io::Result<usize> {
    dst.write_all(header)?;
    Ok(header.len() + T::encode(src, dst)?)
}

fn encode_inner<W: Write>(src: &str, charset: Charset, dst: &mut W) -> std::io::Result<usize> {
    if src.is_empty() {
        return Ok(0);
    }

    match charset {
        Charset::Iso6937 => Iso6937::encode(src, dst),

        Charset::Iso8859_1 => encode_with_header::<Iso8859_1, W>(src, &[0x10, 0x00, 0x01], dst),
        Charset::Iso8859_2 => encode_with_header::<Iso8859_2, W>(src, &[0x10, 0x00, 0x02], dst),
        Charset::Iso8859_3 => encode_with_header::<Iso8859_3, W>(src, &[0x10, 0x00, 0x03], dst),
        Charset::Iso8859_4 => encode_with_header::<Iso8859_4, W>(src, &[0x10, 0x00, 0x04], dst),

        Charset::Iso8859_5 => encode_with_header::<Iso8859_5, W>(src, &[0x01], dst),
        Charset::Iso8859_6 => encode_with_header::<Iso8859_6, W>(src, &[0x02], dst),
        Charset::Iso8859_7 => encode_with_header::<Iso8859_7, W>(src, &[0x03], dst),
        Charset::Iso8859_8 => encode_with_header::<Iso8859_8, W>(src, &[0x04], dst),
        Charset::Iso8859_9 => encode_with_header::<Iso8859_9, W>(src, &[0x05], dst),
        Charset::Iso8859_10 => encode_with_header::<Iso8859_10, W>(src, &[0x06], dst),
        Charset::Iso8859_11 => encode_with_header::<Iso8859_11, W>(src, &[0x07], dst),
        Charset::Iso8859_13 => encode_with_header::<Iso8859_13, W>(src, &[0x09], dst),
        Charset::Iso8859_14 => encode_with_header::<Iso8859_14, W>(src, &[0x0A], dst),
        Charset::Iso8859_15 => encode_with_header::<Iso8859_15, W>(src, &[0x0B], dst),

        Charset::Iso8859_16 => encode_with_header::<Iso8859_16, W>(src, &[0x10, 0x00, 0x10], dst),

        Charset::Utf16 => encode_with_header::<Utf16, W>(src, &[0x11], dst),
        Charset::Gb2312 => encode_with_header::<Gb2312, W>(src, &[0x13], dst),
        Charset::Utf8 => encode_with_header::<Utf8, W>(src, &[0x15], dst),
        Charset::Geo => encode_with_header::<Geo, W>(src, &[0x1E], dst),
    }
}

fn decode_inner<W: Write>(src: &[u8], dst: &mut W) -> std::io::Result<usize> {
    if src.is_empty() {
        return Ok(0);
    }

    let first = src[0];

    match first {
        // ISO-8859-5 (Cyrillic)
        0x01 => Iso8859_5::decode(&src[1 ..], dst),
        // ISO-8859-6 (Arabic)
        0x02 => Iso8859_6::decode(&src[1 ..], dst),
        // ISO-8859-7 (Greek)
        0x03 => Iso8859_7::decode(&src[1 ..], dst),
        // ISO-8859-8 (Hebrew)
        0x04 => Iso8859_8::decode(&src[1 ..], dst),
        // ISO-8859-9 (Turkish)
        0x05 => Iso8859_9::decode(&src[1 ..], dst),
        // ISO-8859-10 (Nordic)
        0x06 => Iso8859_10::decode(&src[1 ..], dst),
        // ISO-8859-11 (Thai)
        0x07 => Iso8859_11::decode(&src[1 ..], dst),
        // ISO-8859-13 (Baltic Rim)
        0x09 => Iso8859_13::decode(&src[1 ..], dst),
        // ISO-8859-14 (Celtic)
        0x0A => Iso8859_14::decode(&src[1 ..], dst),
        // ISO-8859-15 (Western European)
        0x0B => Iso8859_15::decode(&src[1 ..], dst),

        // ISO-8859-X (extended selection)
        0x10 => {
            if src.len() < 3 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "Insufficient data for ISO-8859-X part number",
                ));
            }
            let part = u16::from_be_bytes([src[1], src[2]]);
            let data = &src[3 ..];

            match part {
                0x01 => Iso8859_1::decode(data, dst),
                0x02 => Iso8859_2::decode(data, dst),
                0x03 => Iso8859_3::decode(data, dst),
                0x04 => Iso8859_4::decode(data, dst),
                0x05 => Iso8859_5::decode(data, dst),
                0x06 => Iso8859_6::decode(data, dst),
                0x07 => Iso8859_7::decode(data, dst),
                0x08 => Iso8859_8::decode(data, dst),
                0x09 => Iso8859_9::decode(data, dst),
                0x0A => Iso8859_10::decode(data, dst),
                0x0B => Iso8859_11::decode(data, dst),
                0x0D => Iso8859_13::decode(data, dst),
                0x0E => Iso8859_14::decode(data, dst),
                0x0F => Iso8859_15::decode(data, dst),
                0x10 => Iso8859_16::decode(data, dst),
                // Unknown part number - return empty
                _ => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Unknown ISO-8859-X part number",
                )),
            }
        }

        // UTF-16BE
        0x11 => Utf16::decode(&src[1 ..], dst),

        // GB2312 (Simplified Chinese)
        0x13 => Gb2312::decode(&src[1 ..], dst),

        // UTF-8
        0x15 => Utf8::decode(&src[1 ..], dst),

        // Proprietary Georgian mapping (not part of EN 300 468)
        0x1E => Geo::decode(&src[1 ..], dst),

        // ISO-6937 with explicit prefix
        0x00 => Iso6937::decode(&src[1 ..], dst),

        // ISO-6937 (first byte is part of the text)
        0x20 ..= 0xFF => Iso6937::decode(src, dst),

        // Undefined codes
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Undefined DVB text encoding",
        )),
    }
}
