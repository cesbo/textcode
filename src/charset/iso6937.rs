//! Latin superset of ISO/IEC 6937 with addition of the Euro symbol


use super::data::iso6937::{
    DECODE_MAP,
    HI_MAP,
    ENCODE_MAP,
};


pub (crate) fn singlechar_encode(src: &str, dst: &mut Vec<u8>, hi_map: &[usize], map: &[u8]) {
    for c in src.chars() {
        let c = u32::from(c) as u16;
        if c <= 0x7F {
            dst.push(c as u8);
        } else if c >= 0xA0 {
            let hi = usize::from(c >> 8);
            let lo = usize::from(c & 0xFF);

            let pos = hi_map[hi] * 0xFF + lo;
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


pub (crate) fn singlechar_decode(src: &[u8], dst: &mut String, map: &[u16]) {
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


#[inline]
pub fn encode(src: &str, dst: &mut Vec<u8>) {
    singlechar_encode(src, dst, &HI_MAP, &ENCODE_MAP)
}


#[inline]
pub fn decode(src: &[u8], dst: &mut String) {
    singlechar_decode(src, dst, &DECODE_MAP)
}


#[inline]
pub fn encode_to_vec(src: &str) -> Vec<u8> {
    let mut ret = Vec::new();
    singlechar_encode(src, &mut ret, &HI_MAP, &ENCODE_MAP);
    ret
}


#[inline]
pub fn decode_to_string(src: &[u8]) -> String {
    let mut ret = String::new();
    singlechar_decode(src, &mut ret, &DECODE_MAP);
    ret
}


#[inline]
pub fn bound(src: &[u8], limit: usize) -> usize {
    std::cmp::min(src.len(), limit)
}
