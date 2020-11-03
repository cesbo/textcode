//! Latin superset of ISO/IEC 6937 with addition of the Euro symbol


use super::data::iso6937::{
    DECODE_MAP,
    HI_MAP,
    ENCODE_MAP,
};


#[inline]
pub fn encode(src: &str, dst: &mut Vec<u8>) {
    for c in src.chars() {
        let c = u32::from(c) as u16;

        if c <= 0x7F {
            dst.push(c as u8);
        } else if c >= 0xA0 {
            let hi = usize::from(c >> 8);
            let lo = usize::from(c & 0xFF);

            let pos = HI_MAP[hi] * 0x100 + lo;
            let code = ENCODE_MAP[pos];

            if code > 0xFF {
                dst.push((code >> 8) as u8);
                dst.push((code & 0xFF) as u8);
            } else if code > 0 {
                dst.push(code as u8);
            } else {
                match c {
                    /* LEFT/RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK */
                    0x00AB | 0x00BB => dst.push(b'"'),
                    /* LEFT/RIGHT SINGLE QUOTATION MARK */
                    0x2018 | 0x2019 => dst.push(b'\''),
                    /* LEFT/RIGHT DOUBLE QUOTATION MARK */
                    0x201C | 0x201D => dst.push(b'"'),
                    /* HORIZONTAL ELLIPSIS */
                    0x2026 => dst.extend_from_slice(b"..."),
                    /* UNKNOWN SYMBOL */
                    _ => dst.push(b'?'),
                };
            }
        }
    }
}


#[inline]
pub fn decode(src: &[u8], dst: &mut String) {
    let mut skip = 0;
    let size = src.len();

    let get_map = |code: usize| -> char {
        match DECODE_MAP[code] {
            0x0000 => '�',
            u => unsafe { std::char::from_u32_unchecked(u32::from(u)) },
        }
    };

    while skip < size {
        let c = src[skip];

        if c <= 0x7F {
            dst.push(c as char);
            skip += 1;
            continue;
        }

        let m;

        if c >= 0xC1 && c <= 0xCF {
            // diactrics
            skip += 1;
            if skip >= size {
                dst.push('�');
                break;
            }

            let map_skip =
                usize::from(c - 0xC1) *
                usize::from(b'z' - b'A' + 1) +
                (0x0100 - 0x00A0);

            let c = src[skip];
            if c >= b'A' && c <= b'z' {
                m = get_map(map_skip + usize::from(c - b'A'));
            } else {
                m = '�';
            }

            skip += 1;
        } else {
            m = get_map(usize::from(c) - 0xA0);
            skip += 1;
        }

        dst.push(m);
    }
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
