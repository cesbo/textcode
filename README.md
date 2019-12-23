# textcode

[![docs](https://docs.rs/textcode/badge.svg)](https://docs.rs/textcode)

## Intro

textcode is a library for text encoding/decoding. Supports next charsets:

- `UTF-8`
- `iso-6937` - Latin superset of ISO/IEC 6937 with addition of the Euro symbol
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


Example:

```rust
use textcode::iso8859_5;

const UTF8: &str = "Привет!";
const ISO8859_5: &[u8] = &[0xbf, 0xe0, 0xd8, 0xd2, 0xd5, 0xe2, 0x21];

let mut dst: Vec<u8> = Vec::new();
iso8859_5::encode(UTF8, &mut dst);
assert_eq!(dst.as_slice(), ISO8859_5);

let mut dst = String::new();
iso8859_5::decode(ISO8859_5, &mut dst);
assert_eq!(UTF8, dst.as_str());
```

## Bound

Method `bound` - calculates bound for defined limit.
For example UTF-8 string with 2 four-bytes symbols.
The bound limit is 6 bytes. Method returns 4, because second symbol out of bound.

Example:

```rust
use textcode::utf8;

// "🦀🦀"
const UTF8_DATA: &[u8] = &[0xF0, 0x9F, 0xA6, 0x80, 0xF0, 0x9F, 0xA6, 0x80];

assert_eq!(utf8::bound(UTF8_DATA, 6), 4);
assert_eq!(utf8::bound(UTF8_DATA, 1000), UTF8_DATA.len());
```
