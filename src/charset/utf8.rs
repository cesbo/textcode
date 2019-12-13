pub fn encode(src: &str, dst: &mut Vec<u8>) {
    *dst = src.to_string().into_bytes();
}


pub fn decode(src: &[u8], dst: &mut String) {
    *dst = String::from_utf8_lossy(src).to_string();
}


pub fn bound(src: &[u8], limit: usize) -> usize {
    if limit == 0 || src.len() < limit { return limit; }
    let mut iter = 1;
    let mut delta_len = 0;

    for byte in src.iter() {
        if *byte > 0xC0 || *byte < 0x80 {
            delta_len = 0;
        }

        if iter > limit { break; }

        if *byte > 0x80  {
            delta_len += 1;
        }

        iter += 1;
    }

    if delta_len >= limit { return 0; }

    limit - delta_len
}
