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
//!
//! Types:
//! - [`Charset`]: character code table, [`Charset::from_header`] splits the selector from the text
//! - [`DvbTextRef`]: borrowed DVB text with a valid selector
//! - [`DvbText`]: owned raw DVB text, never validated on construction
//! - [`Error`]: selector errors

use std::fmt;
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

/// Character code table selector error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Selector `0x10` without the two part number bytes
    InvalidLength,
    /// Undefined selector byte or unsupported ISO-8859-X part number
    InvalidCodepage,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidLength => f.write_str("Invalid length of input data"),
            Error::InvalidCodepage => f.write_str("Invalid or unsupported codepage"),
        }
    }
}

impl std::error::Error for Error {}

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

impl Charset {
    /// Reads the character code table selector and returns it with the text that follows.
    ///
    /// Empty input and a first byte of `0x20` or above select ISO-6937 with the whole
    /// input as text.
    pub fn from_header(src: &[u8]) -> Result<(Charset, &[u8]), Error> {
        let Some(&first) = src.first() else {
            return Ok((Charset::Iso6937, src));
        };

        let charset = match first {
            0x00 => Charset::Iso6937,
            0x20 ..= 0xFF => return Ok((Charset::Iso6937, src)),

            0x01 => Charset::Iso8859_5,
            0x02 => Charset::Iso8859_6,
            0x03 => Charset::Iso8859_7,
            0x04 => Charset::Iso8859_8,
            0x05 => Charset::Iso8859_9,
            0x06 => Charset::Iso8859_10,
            0x07 => Charset::Iso8859_11,
            0x09 => Charset::Iso8859_13,
            0x0A => Charset::Iso8859_14,
            0x0B => Charset::Iso8859_15,

            0x10 => {
                if src.len() < 3 {
                    return Err(Error::InvalidLength);
                }
                let part = u16::from_be_bytes([src[1], src[2]]);
                let charset = match part {
                    0x01 => Charset::Iso8859_1,
                    0x02 => Charset::Iso8859_2,
                    0x03 => Charset::Iso8859_3,
                    0x04 => Charset::Iso8859_4,
                    0x05 => Charset::Iso8859_5,
                    0x06 => Charset::Iso8859_6,
                    0x07 => Charset::Iso8859_7,
                    0x08 => Charset::Iso8859_8,
                    0x09 => Charset::Iso8859_9,
                    0x0A => Charset::Iso8859_10,
                    0x0B => Charset::Iso8859_11,
                    0x0D => Charset::Iso8859_13,
                    0x0E => Charset::Iso8859_14,
                    0x0F => Charset::Iso8859_15,
                    0x10 => Charset::Iso8859_16,
                    _ => return Err(Error::InvalidCodepage),
                };
                return Ok((charset, &src[3 ..]));
            }

            0x11 => Charset::Utf16,
            0x13 => Charset::Gb2312,
            0x15 => Charset::Utf8,
            0x1E => Charset::Geo,

            _ => return Err(Error::InvalidCodepage),
        };

        Ok((charset, &src[1 ..]))
    }
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

/// Decodes text without a selector header using the given charset.
pub fn decode_with(src: impl AsRef<[u8]>, charset: Charset) -> String {
    let mut result = String::new();
    // SAFE: writes valid UTF-8 sequences or replacement character
    let dst = unsafe { result.as_mut_vec() };
    let _ = decode_charset(src.as_ref(), charset, dst);
    result
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

fn decode_charset<W: Write>(src: &[u8], charset: Charset, dst: &mut W) -> std::io::Result<usize> {
    match charset {
        Charset::Iso6937 => Iso6937::decode(src, dst),
        Charset::Iso8859_1 => Iso8859_1::decode(src, dst),
        Charset::Iso8859_2 => Iso8859_2::decode(src, dst),
        Charset::Iso8859_3 => Iso8859_3::decode(src, dst),
        Charset::Iso8859_4 => Iso8859_4::decode(src, dst),
        Charset::Iso8859_5 => Iso8859_5::decode(src, dst),
        Charset::Iso8859_6 => Iso8859_6::decode(src, dst),
        Charset::Iso8859_7 => Iso8859_7::decode(src, dst),
        Charset::Iso8859_8 => Iso8859_8::decode(src, dst),
        Charset::Iso8859_9 => Iso8859_9::decode(src, dst),
        Charset::Iso8859_10 => Iso8859_10::decode(src, dst),
        Charset::Iso8859_11 => Iso8859_11::decode(src, dst),
        Charset::Iso8859_13 => Iso8859_13::decode(src, dst),
        Charset::Iso8859_14 => Iso8859_14::decode(src, dst),
        Charset::Iso8859_15 => Iso8859_15::decode(src, dst),
        Charset::Iso8859_16 => Iso8859_16::decode(src, dst),
        Charset::Utf16 => Utf16::decode(src, dst),
        Charset::Gb2312 => Gb2312::decode(src, dst),
        Charset::Utf8 => Utf8::decode(src, dst),
        Charset::Geo => Geo::decode(src, dst),
    }
}

fn decode_inner<W: Write>(src: &[u8], dst: &mut W) -> std::io::Result<usize> {
    let (charset, text) = Charset::from_header(src)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    decode_charset(text, charset, dst)
}

/// Borrowed DVB text with a valid character code table selector.
///
/// [`Display`](fmt::Display) decodes the text.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DvbTextRef<'a> {
    charset: Charset,
    bytes: &'a [u8],
}

impl<'a> DvbTextRef<'a> {
    /// Character code table read from the selector
    pub fn charset(&self) -> Charset {
        self.charset
    }

    /// Whole DVB-coded text including the selector
    pub fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Text bytes after the selector
    pub fn text(&self) -> &'a [u8] {
        // The selector was validated on construction
        Charset::from_header(self.bytes).map_or(&[], |(_, text)| text)
    }

    /// Decodes the text with the charset read from the selector
    pub fn decode(&self) -> String {
        decode_with(self.text(), self.charset)
    }

    /// Decodes the text with the given charset, ignoring the selector
    pub fn decode_with(&self, charset: Charset) -> String {
        decode_with(self.text(), charset)
    }

    /// Owned copy of the whole DVB-coded text
    pub fn to_text(&self) -> DvbText {
        DvbText::from(self.bytes)
    }
}

impl<'a> TryFrom<&'a [u8]> for DvbTextRef<'a> {
    type Error = Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let (charset, _) = Charset::from_header(bytes)?;
        Ok(DvbTextRef { charset, bytes })
    }
}

impl fmt::Display for DvbTextRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.decode().fmt(f)
    }
}

impl fmt::Debug for DvbTextRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.decode())
    }
}

/// Owned DVB text (EN 300 468 annex A), kept as raw bytes.
///
/// The bytes are never validated on construction, so wire text survives even
/// if it cannot be decoded. [`Display`](fmt::Display) decodes the text; an
/// undecodable text displays as empty.
#[derive(Clone, PartialEq, Eq, Default, Hash)]
pub struct DvbText(Vec<u8>);

impl DvbText {
    /// Encodes a UTF-8 string with the charset header
    pub fn encode(text: &str, charset: Charset) -> Self {
        DvbText(encode(text, charset))
    }

    /// Raw DVB-coded bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Raw DVB-coded bytes, consuming the text
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Whether the text has no bytes at all
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Borrowed view with the selector validated
    pub fn as_text(&self) -> Result<DvbTextRef<'_>, Error> {
        DvbTextRef::try_from(self.0.as_slice())
    }
}

impl From<Vec<u8>> for DvbText {
    fn from(data: Vec<u8>) -> Self {
        DvbText(data)
    }
}

impl From<&[u8]> for DvbText {
    fn from(data: &[u8]) -> Self {
        DvbText(data.to_vec())
    }
}

impl fmt::Display for DvbText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_text() {
            Ok(text) => text.fmt(f),
            Err(_) => Ok(()),
        }
    }
}

impl fmt::Debug for DvbText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_text() {
            Ok(text) => write!(f, "{:?}", text.decode()),
            Err(_) => write!(f, "DvbText({:02X?})", self.0),
        }
    }
}
