//! ISO 6937 parser
//!
//! 1. Build generator: rustc parser.rs
//! 2. Launch: ./parser >../../src/charset/data/iso6937.rs


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


const DATA_DIACTRICS: &[(&str, &str)] = &[
    (   /* 0xC1 */
        "AEIOUaeiou",
        "ÀÈÌÒÙàèìòù"
    ),
    (   /* 0xC2 */
        "ACEILNORSUYZacegilnorsuyz",
        "ÁĆÉÍĹŃÓŔŚÚÝŹáćéģíĺńóŕśúýź"
    ),
    (   /* 0xC3 */
        "ACEGHIJOSUWYaceghijosuwy",
        "ÂĈÊĜĤÎĴÔŜÛŴŶâĉêĝĥîĵôŝûŵŷ"
    ),
    (   /* 0xC4 */
        "AINOUainou",
        "ÃĨÑÕŨãĩñõũ"
    ),
    (   /* 0xC5 */
        "AEIOUaeiou",
        "ĀĒĪŌŪāēīōū"
    ),
    (   /* 0xC6 */
        "AGUagu",
        "ĂĞŬăğŭ"
    ),
    (   /* 0xC7 */
        "CEGIZcegz",
        "ĊĖĠİŻċėġż"
    ),
    (   /* 0xC8 */
        "AEIOUYaeiouy",
        "ÄËÏÖÜŸäëïöüÿ"
    ),
    (   /* 0xC9 */
        "",
        ""
    ),
    (   /* 0xCA */
        "AUau",
        "ÅŮåů"
    ),
    (   /* 0xCB */
        "CGKLNRSTcklnrst",
        "ÇĢĶĻŅŖŞŢçķļņŗşţ"
    ),
    (   /* 0xCC */
        "",
        ""
    ),
    (   /* 0xCD */
        "OUou",
        "ŐŰőű"
    ),
    (   /* 0xCE */
        "AEIUaeiu",
        "ĄĘĮŲąęįų"
    ),
    (   /* 0xCF */
        "CDELNRSTZcdelnrstz",
        "ČĎĚĽŇŘŠŤŽčďěľňřšťž"
    ),
];



fn main() {
    println!("//! Latin superset of ISO/IEC 6937 with addition of the Euro symbol");
    println!("//! File generated with tools/iso6937/parser.rs");
    println!("");

    let mut arr_encode: Vec<(u16, u16)> = Vec::new();

    println!(
        "pub const DECODE_MAP: [u16; {}] = [",
        DATA.len() + (DATA_DIACTRICS.len() * usize::from(b'z' - b'A' + 1))
    );

    let print_code = |count, unicode: u16| {
        if (count % 8) == 0 {
            if count > 0 {
                println!("");
            }
            print!("    ");
        } else {
            print!(" ");
        }

        print!("0x{:04x},", unicode);
    };

    for (count, &unicode) in DATA.iter().enumerate() {
        print_code(count, unicode);

        if unicode != 0 {
            arr_encode.push((unicode, (count as u16) + 0x00A0));
        }
    }
    println!("");

    for (cn, pair) in DATA_DIACTRICS.iter().enumerate() {
        let mut skip = 0;

        let cn: u16 = 0x00C1 + cn as u16;
        let bytes = pair.0.as_bytes();
        let chars: Vec<u16> = pair.1.chars().map(|v| v as u32 as u16).collect();

        println!("    /* 0x{:02x} */", cn);

        for (count, n) in (b'A' ..= b'z').enumerate() {
            let unicode;

            if skip < bytes.len() && n == bytes[skip] {
                unicode = chars[skip] as u16;
                arr_encode.push((unicode, (cn << 8) | (n as u16)));
                skip += 1;
            } else {
                unicode = 0u16;
                arr_encode.push((unicode, 0));
            }

            print_code(count, unicode);
        }
        println!("");
    }

    println!("];");

    // encode

    arr_encode.sort_by(|a, b| {
        (a.0).cmp(&b.0)
    });

    let mut code_map: Vec<u16> = vec![0; 0x100];
    let mut hi_map: Vec<usize> = vec![0; 0x100];

    let mut hi_byte = 0u8;
    let mut hi_skip = 0usize;

    for &(unicode, code) in arr_encode.iter() {
        if unicode == 0 {
            continue;
        }

        let hi = (unicode >> 8) as u8;
        let lo = (unicode & 0xFF) as u8;

        if hi_byte != hi {
            hi_byte = hi;
            hi_skip += 1;
            hi_map[usize::from(hi)] = hi_skip;

            for _ in 0 .. 0x100 {
                code_map.push(0)
            }
        }

        let pos = hi_skip * 0x100 + usize::from(lo);
        code_map[pos] = code;
    }

    println!("");
    println!("pub const HI_MAP: [usize; {}] = [", hi_map.len());
    for (n, &pos) in hi_map.iter().enumerate() {
        if (n % 8) == 0 {
            if n > 0 {
                println!("");
            }
            print!("    ");
        } else {
            print!(" ");
        }

        print!("{},", pos);
    }
    println!("");
    println!("];");

    println!("");
    println!("pub const ENCODE_MAP: [u16; {}] = [", code_map.len());
    for (n, &code) in code_map.iter().enumerate() {
        if (n % 8) == 0 {
            if n > 0 {
                println!("");
            }
            print!("    ");
        } else {
            print!(" ");
        }

        print!("0x{:04x},", code);
    }
    println!("");
    println!("];");
}
