# textcode

[![docs](https://docs.rs/textcode/badge.svg)](https://docs.rs/textcode)

## Intro

Textcode is a library for text encoding/decoding.

The library uses non-strict conversion: invalid or unmappable characters are replaced with `?`.

## ⚠️ Breaking change in v0.3.0

The library API has been completely redesigned:

**Old API (v0.2.x):** module-based functions

```rust
use textcode::iso8859_5;

let mut text = String::new();
iso8859_5::decode(b"\xbf\xe0\xd8\xd2\xd5\xe2!", &mut text);

let mut bytes = Vec::new();
iso8859_5::encode("Привет!", &mut bytes);
```

**New API (v0.3.x):** generic functions with codec types

```rust
use textcode::{Iso8859_5, decode, encode};

let text = decode::<Iso8859_5>(b"\xbf\xe0\xd8\xd2\xd5\xe2!");

let bytes = encode::<Iso8859_5>("Привет!");
```

## Charsets

- `UTF-8`
- `UTF-16`
- `iso-6937` - Latin superset of ISO/IEC 6937 with Euro and letters with diacritics
- `iso-8859-1` - Western European
- `iso-8859-2` - Central European
- `iso-8859-3` - South European
- `iso-8859-4` - North European
- `iso-8859-5` - Cyrillic
- `iso-8859-6` - Arabic
- `iso-8859-7` - Greek
- `iso-8859-8` - Hebrew
- `iso-8859-9` - Turkish
- `iso-8859-10` - Nordic
- `iso-8859-11` - Thai
- `iso-8859-13` - Baltic Rim
- `iso-8859-14` - Celtic
- `iso-8859-15` - Western European
- `iso-8859-16` - South-Eastern European
- `gb2312` - Simplified Chinese

## Example

```rust
use textcode::{Iso8859_5, decode, encode};

const UTF8: &str = "Привет!";
const ISO8859_5: &[u8] = &[0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];

let text = decode::<Iso8859_5>(ISO8859_5);
assert_eq!(text, UTF8);

let bytes = encode::<Iso8859_5>(UTF8);
assert_eq!(bytes, ISO8859_5);
```
