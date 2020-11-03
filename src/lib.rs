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


mod data;

pub mod iso6937;

mod iso8859;
pub use iso8859::*;

pub mod gb2312;

pub mod utf8;
