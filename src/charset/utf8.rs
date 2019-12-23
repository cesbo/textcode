#[inline]
pub fn encode(src: &str, dst: &mut Vec<u8>) {
    let src_slice = src.to_string().into_bytes();
    dst.extend_from_slice(&src_slice);
}


#[inline]
pub fn decode(src: &[u8], dst: &mut String) {
    dst.push_str(&String::from_utf8_lossy(src));
}


pub fn bound(src: &[u8], limit: usize) -> usize {
    if limit == 0 || src.len() <= limit {
        return limit;
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
