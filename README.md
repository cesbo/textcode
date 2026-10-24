# textcode

[![docs](https://docs.rs/textcode/badge.svg)](https://docs.rs/textcode)

Text encoding and decoding between legacy charsets and UTF-8, plus DVB text
(ETSI EN 300 468 Annex A) with charset selectors.

Conversion is non-strict: bytes without a mapping decode to U+FFFD, characters
without a mapping encode to `?`.

## Charsets

- `Utf8`
- `Utf16` - decodes BE and LE with a BOM, encodes BE without a BOM
- `Iso6937` - Latin superset of ISO/IEC 6937 with the Euro sign
- `Iso8859_1` - Western European
- `Iso8859_2` - Central European
- `Iso8859_3` - South European
- `Iso8859_4` - North European
- `Iso8859_5` - Cyrillic
- `Iso8859_6` - Arabic
- `Iso8859_7` - Greek
- `Iso8859_8` - Hebrew
- `Iso8859_9` - Turkish
- `Iso8859_10` - Nordic
- `Iso8859_11` - Thai
- `Iso8859_13` - Baltic Rim
- `Iso8859_14` - Celtic
- `Iso8859_15` - Western European
- `Iso8859_16` - South-Eastern European
- `Gb2312` - Simplified Chinese
- `Geo` - DVB single-byte Georgian encoding (not part of EN 300 468)

## Usage

Every charset is a type implementing the `Textcode` trait; the generic
functions pick the codec by type parameter:

```rust
use textcode::{Iso8859_5, decode, encode};

let text = decode::<Iso8859_5>(b"\xbf\xe0\xd8\xd2\xd5\xe2!");
assert_eq!(text, "Привет!");

let bytes = encode::<Iso8859_5>("Привет!");
assert_eq!(bytes, b"\xbf\xe0\xd8\xd2\xd5\xe2!");
```

`decode_to_slice` and `encode_to_slice` write into a caller-provided buffer
and return the number of bytes written:

```rust
use textcode::{Utf8, decode_to_slice};

let mut buf = [0u8; 16];
let len = decode_to_slice::<Utf8>(b"abc", &mut buf);
assert_eq!(&buf[.. len], b"abc");
```

## DVB text

The `dvb` module handles EN 300 468 Annex A text, where the first bytes select
the charset:

| Selector                       | Charset                    |
|--------------------------------|----------------------------|
| `0x20`-`0xFF` (none) or `0x00` | `Iso6937`                  |
| `0x01`-`0x0B`                  | `Iso8859_5`-`Iso8859_15`   |
| `0x10 0x00 NN`                 | `Iso8859_NN` (`NN` = 1-16) |
| `0x11`                         | `Utf16` (big endian)       |
| `0x13`                         | `Gb2312`                   |
| `0x15`                         | `Utf8`                     |
| `0x1E`                         | `Geo`                      |

Decoding drops the Annex A.1 control codes of the single-byte tables (`0x00`-`0x1F`,
`0x80`-`0x9F`); `0x8A` (CR/LF) and a bare `0x0A` become `\n`. The multi-byte tables
carry the same codes at `U+E080`-`U+E09F` and pass them through unchanged.

```rust
use textcode::dvb::{self, Charset};

// decoding follows the selector
assert_eq!(dvb::decode(b"\x01\xbf\xe0\xd8\xd2\xd5\xe2!"), "Привет!");

// encoding emits the selector for the chosen charset
assert_eq!(dvb::encode("Привет!", Charset::Iso8859_5), b"\x01\xbf\xe0\xd8\xd2\xd5\xe2!");
```

`DvbText` owns raw DVB bytes as they come off the wire and is never validated
on construction; `DvbTextRef` is a borrowed view with a verified selector.
Both decode through `Display`:

```rust
use textcode::dvb::{Charset, DvbText, DvbTextRef};

let text = DvbText::from(&b"\x15Menu"[..]);
assert_eq!(text.to_string(), "Menu");

let text_ref = DvbTextRef::try_from(text.as_bytes()).unwrap();
assert_eq!(text_ref.charset(), Charset::Utf8);
assert_eq!(text_ref.text(), b"Menu");

// an explicit charset overrides the selector
assert_eq!(text_ref.decode_with(Charset::Iso8859_1), "Menu");

// an undecodable selector displays as empty, the bytes stay reachable
let bad = DvbText::from(&b"\x1f?"[..]);
assert_eq!(bad.to_string(), "");
assert_eq!(bad.as_bytes(), b"\x1f?");
```

## License

MIT
