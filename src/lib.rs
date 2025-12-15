//! # textcode
//!
//! [![docs](https://docs.rs/textcode/badge.svg)](https://docs.rs/textcode)
//!
//! ## Intro
//!
//! textcode is a library for text encoding/decoding. Supports next charsets:
//!
//! - `UTF-8`
//! - `iso-6937` - Latin superset of ISO/IEC 6937 with addition of the Euro symbol
//! - `iso-8859-1` - Western European
//! - `iso-8859-2` - Central European
//! - `iso-8859-3` - South European
//! - `iso-8859-4` - North European
//! - `iso-8859-5` - Cyrillic
//! - `iso-8859-6` - Arabic
//! - `iso-8859-7` - Greek
//! - `iso-8859-8` - Hebrew
//! - `iso-8859-9` - Turkish
//! - `iso-8859-10` - Nordic
//! - `iso-8859-11` - Thai
//! - `iso-8859-13` - Baltic Rim
//! - `iso-8859-14` - Celtic
//! - `iso-8859-15` - Western European
//! - `iso-8859-16` - South-Eastern European
//! - `gb2312` - Simplified Chinese
//!
//! [Read more...](https://github.com/cesbo/textcode)
//!
//! ## Usage
//!
//! ```rust
//! use textcode::{Iso8859_5, decode, encode};
//!
//! let text = decode::<Iso8859_5>(b"\xbf\xe0\xd8\xd2\xd5\xe2!");
//! assert_eq!(text, "Привет!");
//!
//! let bytes = encode::<Iso8859_5>("Привет!");
//! assert_eq!(bytes, b"\xbf\xe0\xd8\xd2\xd5\xe2!");
//! ```

use std::io::Write;

mod data;
mod iso8859;
mod utils;

mod gb2312;
mod iso6937;
mod utf8;

pub use gb2312::Gb2312;
pub use iso6937::Iso6937;
pub use iso8859::*;
pub use utf8::Utf8;
pub(crate) use utils::*;

/// Trait for text encoding/decoding implementations.
///
/// Types implementing this trait provide conversion between
/// a specific character encoding and UTF-8.
pub trait Textcode {
    /// Decodes bytes from the source encoding to UTF-8.
    fn decode<W: Write, R: AsRef<[u8]>>(src: R, dst: &mut W) -> std::io::Result<usize>;

    /// Encodes UTF-8 string to the target encoding.
    fn encode<W: Write, R: AsRef<str>>(src: R, dst: &mut W) -> std::io::Result<usize>;
}

/// Decodes bytes from the source encoding to a UTF-8 String.
pub fn decode<T: Textcode>(src: impl AsRef<[u8]>) -> String {
    let mut result = String::new();
    // SAFE: writes valid UTF-8 sequences or DECODE_FALLBACK
    let dst = unsafe { result.as_mut_vec() };
    let _ = T::decode(src, dst);
    result
}

/// Decodes bytes from the source encoding to a UTF-8 slice.
/// Returns the number of bytes written.
pub fn decode_to_slice<T: Textcode>(src: impl AsRef<[u8]>, dst: &mut [u8]) -> usize {
    let mut cursor = std::io::Cursor::new(dst);
    T::decode(src, &mut cursor).unwrap_or(0)
}

/// Encodes UTF-8 string to the target encoding.
pub fn encode<T: Textcode>(src: impl AsRef<str>) -> Vec<u8> {
    let mut ret = Vec::new();
    let _ = T::encode(src, &mut ret);
    ret
}

/// Encodes UTF-8 string to the target encoding slice.
/// Returns the number of bytes written.
pub fn encode_to_slice<T: Textcode>(src: impl AsRef<str>, dst: &mut [u8]) -> usize {
    let mut cursor = std::io::Cursor::new(dst);
    T::encode(src, &mut cursor).unwrap_or(0)
}
