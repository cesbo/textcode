pub (crate) fn iso6937_encode(src: &str, dst: &mut Vec<u8>, map: &[u16; 96]) {
    for c in src.chars() {
        let c = c as u16;
        if c <= 0x007F {
            dst.push(c as u8);
        } else if c >= 0x00A0 {
            if let Some(v) = map.iter().position(|&u| u == c) {
                dst.push((v as u8) + 0xA0);
            } else {
                match c as u16 {
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
                    /* UNKNOWN SYMBOL */
                    _ => dst.push(b'?'),
                };
            }
        }
    }
}


pub (crate) fn iso6937_decode(src: &[u8], dst: &mut String, map: &[u16; 96]) {
    for &c in src {
        if c <= 0x7F {
            dst.push(c as char);
        } else if c >= 0xA0 {
            match map[c as usize - 0xA0] {
                0 => dst.push('?'),
                u => dst.push(unsafe { std::char::from_u32_unchecked(u32::from(u)) }),
            };
        }
    }
}


/// Latin superset of ISO/IEC 6937 with addition of the Euro symbol
const DATA: [u16; 96] = [
    0x00a0, 0x00a1, 0x00a2, 0x00a3, 0x20ac, 0x00a5, 0x0000, 0x00a7,
    0x00a4, 0x2018, 0x201c, 0x00ab, 0x2190, 0x2191, 0x2192, 0x2193,
    0x00b0, 0x00b1, 0x00b2, 0x00b3, 0x00d7, 0x00b5, 0x00b6, 0x00b7,
    0x00f7, 0x2019, 0x201d, 0x00bb, 0x00bc, 0x00bd, 0x00be, 0x00bf,
    0x0000, 0x0300, 0x0301, 0x0302, 0x0303, 0x0304, 0x0306, 0x0307,
    0x0308, 0x0000, 0x030a, 0x0327, 0x0000, 0x030b, 0x0328, 0x030c,
    0x2015, 0x00b9, 0x00ae, 0x00a9, 0x2122, 0x266a, 0x00ac, 0x00a6,
    0x0000, 0x0000, 0x0000, 0x0000, 0x215b, 0x215c, 0x215d, 0x215e,
    0x2126, 0x00c6, 0x0110, 0x00aa, 0x0126, 0x0000, 0x0132, 0x013f,
    0x0141, 0x00d8, 0x0152, 0x00ba, 0x00de, 0x0166, 0x014a, 0x0149,
    0x0138, 0x00e6, 0x0111, 0x00f0, 0x0127, 0x0131, 0x0133, 0x0140,
    0x0142, 0x00f8, 0x0153, 0x00df, 0x00fe, 0x0167, 0x014b, 0x00ad,
];

#[inline]
pub fn encode(src: &str, dst: &mut Vec<u8>) { iso6937_encode(src, dst, &DATA) }

#[inline]
pub fn decode(src: &[u8], dst: &mut String) { iso6937_decode(src, dst, &DATA) }
