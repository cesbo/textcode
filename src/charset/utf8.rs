//! UTF-8


#[inline]
pub fn encode(src: &str, dst: &mut Vec<u8>) {
    dst.extend_from_slice(src.as_bytes());
}


#[inline]
pub fn decode(src: &[u8], dst: &mut String) {
    match std::str::from_utf8(src) {
        Ok(v) => dst.push_str(v),
        _ => dst.push('�'),
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

pub fn bound(src: &[u8], limit: usize) -> usize {
    if limit == 0 {
        return limit;
    }

    if src.len() <= limit {
        return src.len();
    }

    let mut cnt_limit = limit;

    while cnt_limit > 0 {
        if src[cnt_limit] <= 0x7F || src[cnt_limit] >= 0xC0 {
            break;
        }
        cnt_limit -= 1;
    }

    cnt_limit
}
