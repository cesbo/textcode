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
    let mut last_char_len = 0;

    while cnt_limit > 0 {
        cnt_limit -= 1;
        if src[cnt_limit] <= 0x7F {
            last_char_len = 1; 
            break;
        } else if (src[cnt_limit] & 0xE0) == 0xC0 {
            last_char_len = 2;
            break;
        } else if (src[cnt_limit] & 0xF0) == 0xE0 {
            last_char_len = 3;
            break;
        } else if (src[cnt_limit] & 0xF8) == 0xF0 {
            last_char_len = 4;
            break;
        }
    }

    if last_char_len + cnt_limit > limit {
        cnt_limit
    } else {
        limit
    }
}
