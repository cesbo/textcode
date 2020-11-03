//! Encoding/Decoding map generator
//! Build: rustc -o build main.rs
//! Launch: ./build >../src/data.rs

mod iso6937;
mod iso8859;
mod gb2312;


pub fn push_unicode_check(dst: &Vec<(u16, u16)>, unicode: u16) -> bool {
    for pair in dst.iter() {
        if pair.0 == unicode {
            return true;
        }
    }

    false
}


pub fn push_unicode(dst: &mut Vec<(u16, u16)>) {
    /* LEFT/RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK */
    if ! push_unicode_check(dst, 0x00AB) {
        dst.push((0x00AB, u16::from(b'"')))
    }
    if ! push_unicode_check(dst, 0x00BB) {
        dst.push((0x00BB, u16::from(b'"')))
    }

    /* LEFT/RIGHT SINGLE QUOTATION MARK */
    if ! push_unicode_check(dst, 0x2018) {
        dst.push((0x2018, u16::from(b'\'')))
    }
    if ! push_unicode_check(dst, 0x2019) {
        dst.push((0x2019, u16::from(b'\'')))
    }

    /* LEFT/RIGHT DOUBLE QUOTATION MARK */
    if ! push_unicode_check(dst, 0x201C) {
        dst.push((0x201C, u16::from(b'"')))
    }
    if ! push_unicode_check(dst, 0x201D) {
        dst.push((0x201D, u16::from(b'"')))
    }

    /* HORIZONTAL ELLIPSIS */
    if ! push_unicode_check(dst, 0x2026) {
        dst.push((0x2026, u16::from(b'.')))
    }
}


fn main() {
    iso6937::build().unwrap();
    iso8859::build().unwrap();
    gb2312::build().unwrap();
}
