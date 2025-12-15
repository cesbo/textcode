//! GB2312 Parser
//!
//! 1. Build generator: rustc parser.rs
//! 2. Launch: ./parser >../../src/charset/data/gb2312.rs

use std::{
    fs::File,
    io::{
        self,
        BufRead,
        BufReader,
    },
    path::Path,
};

use super::push_unicode;

fn read_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let hi_limit = 0x77;
    let hi_size = hi_limit - 0x21 + 1;
    let lo_size = 0x7F - 0x21;

    let mut arr_decode: Vec<u16> = vec![0; hi_size * lo_size];
    let mut arr_encode: Vec<(u16, u16)> = Vec::new();

    let file = File::open(path)?;

    for line in BufReader::new(file).lines() {
        let line = line?;
        let line = line.trim_start();
        if line.starts_with("#") {
            continue;
        }
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        let mut split = line.split_whitespace();
        let code = split.next().unwrap();
        let code = usize::from_str_radix(&code[2 ..], 16).unwrap();
        let hi = code >> 8;
        let lo = code & 0xFF;
        if hi < 0x21 || hi > 0x77 || lo < 0x21 || lo > 0x7F {
            continue;
        }

        let unicode = split.next().unwrap();
        let unicode = u16::from_str_radix(&unicode[2 ..], 16).unwrap();

        arr_encode.push((unicode, code as u16));

        let code = ((hi - 0x21) * lo_size) + (lo - 0x21);
        arr_decode[code as usize] = unicode;
    }

    println!("");
    println!("#[rustfmt::skip]");
    println!(
        "pub static DECODE_MAP_GB2312: [u16; {}] = [",
        arr_decode.len()
    );
    for (n, &unicode) in arr_decode.iter().enumerate() {
        if (n % 8) == 0 {
            if n > 0 {
                println!("");
            }
            print!("    ");
        } else {
            print!(" ");
        }

        print!("0x{:04x},", unicode);
    }
    println!("");
    println!("];");

    // encode

    push_unicode(&mut arr_encode);

    arr_encode.sort_by(|a, b| (a.0).cmp(&b.0));

    let mut code_map: Vec<u16> = vec![0; 0x100];
    let mut hi_map: Vec<usize> = vec![0; 0x100];

    let mut hi_byte = 0u8;
    let mut hi_skip = 0usize;

    for (unicode, code) in arr_encode.iter() {
        if *unicode == 0 {
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
        code_map[pos] = *code;
    }

    println!("");
    println!("#[rustfmt::skip]");
    println!("pub static HI_MAP_GB2312: [usize; {}] = [", hi_map.len());
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
    println!("#[rustfmt::skip]");
    println!(
        "pub static ENCODE_MAP_GB2312: [u16; {}] = [",
        code_map.len()
    );
    for (n, &code) in code_map.iter().enumerate() {
        if (n % 8) == 0 {
            if n > 0 {
                println!("");
            }
            print!("    ");
        } else {
            print!(" ");
        }

        let code = if code != 0 { code | 0x8080 } else { code };

        print!("0x{:04x},", code);
    }
    println!("");
    println!("];");

    Ok(())
}

pub fn build() -> io::Result<()> {
    let base_path = std::env::current_exe()?;
    let base_path = base_path.parent().unwrap();
    let base_path = base_path.join("data");

    println!("// Simplified Chinese. File generated with build/gb2312.rs");
    println!("");
    read_file(base_path.join("GB2312.TXT"))?;

    Ok(())
}
