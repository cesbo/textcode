//! Latin superset of ISO/IEC 6937 with addition of the Euro symbol

use crate::{
    DECODE_FALLBACK,
    ENCODE_FALLBACK,
    data::{
        DECODE_MAP_ISO6937,
        ENCODE_MAP_ISO6937,
        HI_MAP_ISO6937,
    },
};

pub fn encode(src: &str, dst: &mut Vec<u8>) {
    for c in src.chars() {
        let c = u32::from(c) as u16;

        if c <= 0x7F {
            dst.push(c as u8);
        } else if c >= 0xA0 {
            let hi = usize::from(c >> 8);
            let lo = usize::from(c & 0xFF);

            let pos = HI_MAP_ISO6937[hi] * 0x100 + lo;
            let code = ENCODE_MAP_ISO6937[pos];

            if code > 0xFF {
                dst.push((code >> 8) as u8);
                dst.push((code & 0xFF) as u8);
            } else if code > 0 {
                dst.push(code as u8);
            } else {
                dst.push(ENCODE_FALLBACK);
            }
        }
    }
}

pub fn decode(src: &[u8], dst: &mut String) {
    let mut skip = 0;
    let size = src.len();

    let get_map = |code: usize| -> char {
        match DECODE_MAP_ISO6937[code] {
            0x0000 => DECODE_FALLBACK,
            u => {
                let c = u32::from(u);
                std::char::from_u32(c).unwrap_or(DECODE_FALLBACK)
            }
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

        if (0xC1 ..= 0xCF).contains(&c) {
            // diactrics
            skip += 1;
            if skip >= size {
                dst.push(DECODE_FALLBACK);
                break;
            }

            let map_skip = usize::from(c - 0xC1) * usize::from(b'z' - b'A' + 1) + (0x0100 - 0x00A0);

            let c = src[skip];
            if (b'A' ..= b'z').contains(&c) {
                m = get_map(map_skip + usize::from(c - b'A'));
            } else {
                m = DECODE_FALLBACK;
            }

            skip += 1;
        } else if c >= 0xA0 {
            m = get_map(usize::from(c) - 0xA0);
            skip += 1;
        } else {
            m = DECODE_FALLBACK;
            skip += 1;
        }

        dst.push(m);
    }
}

pub fn encode_to_vec(src: &str) -> Vec<u8> {
    let mut ret = Vec::new();
    encode(src, &mut ret);
    ret
}

pub fn decode_to_string(src: &[u8]) -> String {
    let mut ret = String::new();
    decode(src, &mut ret);
    ret
}
