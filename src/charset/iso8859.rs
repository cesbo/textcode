
fn singlechar_encode(src: &str, dst: &mut Vec<u8>, hi_map: &[usize], map: &[u8]) {
    for c in src.chars() {
        let c = u32::from(c) as u16;
        if c <= 0x7F {
            dst.push(c as u8);
        } else if c >= 0xA0 {
            let hi = usize::from(c >> 8);
            let lo = usize::from(c & 0xFF);

            let pos = hi_map[hi] * 0x100 + lo;
            let code = map[pos];

            if code != 0x0000 {
                dst.push(code);
            } else {
                match c {
                    /* LEFT/RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK */
                    0x00AB | 0x00BB => dst.push(b'"'),
                    /* LEFT/RIGHT SINGLE QUOTATION MARK */
                    0x2018 | 0x2019 => dst.push(b'\''),
                    /* SINGLE HIGH-REVERSED-9 QUOTATION MARK */
                    0x201B => dst.push(b'\''),
                    /* LEFT/RIGHT DOUBLE QUOTATION MARK */
                    0x201C | 0x201D => dst.push(b'"'),
                    /* DOUBLE HIGH-REVERSED-9 QUOTATION MARK */
                    0x201F => dst.push(b'"'),
                    /* HORIZONTAL ELLIPSIS */
                    0x2026 => dst.extend_from_slice(b"..."),
                    /* REPLACEMENT CHARACTER */
                    0xFFFD => dst.push(b'?'),
                    /* UNKNOWN SYMBOL */
                    _ => dst.push(b'?'),
                };
            }
        }
    }
}


fn singlechar_decode(src: &[u8], dst: &mut String, map: &[u16]) {
    for &c in src {
        if c <= 0x7F {
            dst.push(c as char);
        } else if c >= 0xA0 {
            match map[c as usize - 0xA0] {
                0 => dst.push('�'),
                u => dst.push(unsafe { std::char::from_u32_unchecked(u32::from(u)) }),
            };
        }
    }
}


macro_rules! iso8859 {
    ( $($name: ident, $decode_map: ident, $hi_map: ident, $encode_map: ident),* ) => {
        $(
            pub mod $name {
                use crate::charset::{
                    data::iso8859::{
                        $decode_map,
                        $hi_map,
                        $encode_map,
                    },
                };

                #[inline]
                pub fn encode(src: &str, dst: &mut Vec<u8>) {
                    super::singlechar_encode(src, dst, &$hi_map, &$encode_map)
                }

                #[inline]
                pub fn decode(src: &[u8], dst: &mut String) {
                    super::singlechar_decode(src, dst, &$decode_map)
                }

                #[inline]
                pub fn encode_to_vec(src: &str) -> Vec<u8> {
                    let mut ret = Vec::new();
                    encode(src, &mut ret);
                    ret
                }

                #[inline]
                pub fn decode_to_string(src: &[u8]) -> String {
                    let mut ret = String::new();
                    decode(src, &mut ret);
                    ret
                }
            }
        )*
    }
}


iso8859!(
    iso8859_1, DECODE_MAP_1, HI_MAP_1, ENCODE_MAP_1, // Western European
    iso8859_2, DECODE_MAP_2, HI_MAP_2, ENCODE_MAP_2, // Central European
    iso8859_3, DECODE_MAP_3, HI_MAP_3, ENCODE_MAP_3, // South European
    iso8859_4, DECODE_MAP_4, HI_MAP_4, ENCODE_MAP_4, // North European
    iso8859_5, DECODE_MAP_5, HI_MAP_5, ENCODE_MAP_5, // Cyrillic
    iso8859_6, DECODE_MAP_6, HI_MAP_6, ENCODE_MAP_6, // Arabic
    iso8859_7, DECODE_MAP_7, HI_MAP_7, ENCODE_MAP_7, // Greek
    iso8859_8, DECODE_MAP_8, HI_MAP_8, ENCODE_MAP_8, // Hebrew
    iso8859_9, DECODE_MAP_9, HI_MAP_9, ENCODE_MAP_9, // Turkish
    iso8859_10, DECODE_MAP_10, HI_MAP_10, ENCODE_MAP_10, // Nordic
    iso8859_11, DECODE_MAP_11, HI_MAP_11, ENCODE_MAP_11, // Thai
    iso8859_13, DECODE_MAP_13, HI_MAP_13, ENCODE_MAP_13, // Baltic Rim
    iso8859_14, DECODE_MAP_14, HI_MAP_14, ENCODE_MAP_14, // Celtic
    iso8859_15, DECODE_MAP_15, HI_MAP_15, ENCODE_MAP_15, // Western European
    iso8859_16, DECODE_MAP_16, HI_MAP_16, ENCODE_MAP_16 // South-Eastern European
);
