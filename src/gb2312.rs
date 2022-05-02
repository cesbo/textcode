//! Simplified Chinese


use crate::data::{
    DECODE_MAP_GB2312,
    HI_MAP_GB2312,
    ENCODE_MAP_GB2312,
};


const LO_SIZE: usize = 0x7F - 0x21;


pub fn encode(src: &str, dst: &mut Vec<u8>) {
    for c in src.chars() {
        let c = u32::from(c) as u16;
        if c <= 0x7F {
            dst.push(c as u8);
        } else if c >= 0xA0 {
            let hi = usize::from(c >> 8);
            let lo = usize::from(c & 0xFF);

            let pos = HI_MAP_GB2312[hi] * 0x100 + lo;
            let code = ENCODE_MAP_GB2312[pos];
            if code != 0x0000 {
                dst.push((code >> 8) as u8);
                dst.push((code & 0xFF) as u8);
            } else {
                dst.push(b'?');
            }
        }
    }
}


pub fn decode(src: &[u8], dst: &mut String) {
    let mut skip = 0;
    let size = src.len();

    while skip < size {
        let c = src[skip];

        if c <= 0x7F {
            dst.push(c as char);
            skip += 1;
            continue;
        }

        skip += 1;
        if skip >= size {
            dst.push('�');
            break;
        }

        let hi = usize::from(c) & 0x7F;
        let lo = usize::from(src[skip]) & 0x7F;
        skip += 1;

        if lo < 0x21 || hi < 0x21 {
            dst.push('�');
            continue;
        }

        let shift = (hi - 0x21) * LO_SIZE + (lo - 0x21);

        if shift >= DECODE_MAP_GB2312.len() {
            dst.push('�');
            continue;
        }

        let c = u32::from(DECODE_MAP_GB2312[shift]);

        if c == 0 {
            dst.push('�');
            continue;
        }

        dst.push(unsafe { std::char::from_u32_unchecked(c) })
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
