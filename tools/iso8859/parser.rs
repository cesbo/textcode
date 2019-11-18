//! ISO 8859 Parser
//!
//! 1. Build generator: rustc parser.rs
//! 2. Launch: ./parser


use std::{
    fs::File,
    io::{
        self,
        BufReader,
        BufRead,
    },
    path::Path,
};


const ARR_SIZE: usize = 0xFF - 0xA0 + 1;


fn read_file<P: AsRef<Path>>(path: P, part: usize) -> io::Result<()> {
    let mut arr_decode: Vec<u16> = vec![0x0000; ARR_SIZE];
    let mut arr_encode: Vec<(u16, u8)> = Vec::new();

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
        let code = u8::from_str_radix(&code[2 ..], 16).unwrap();
        if code < 0xA0 {
            continue;
        }

        let unicode = split.next().unwrap();
        let unicode = u16::from_str_radix(&unicode[2 ..], 16).unwrap();
        arr_encode.push((unicode, code));
        arr_decode[(code - 0xA0) as usize] = unicode;
    }

    println!("pub const DECODE_MAP_{}: [u16; {}] = [", part, ARR_SIZE);
    for (n, unicode) in arr_decode.iter().enumerate() {
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

    arr_encode.sort_by(|a, b| {
        (a.0).cmp(&b.0)
    });

    let mut code_map: Vec<u8> = vec![0; 0x100];
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

        let pos = hi_skip * 0xFF + usize::from(lo);
        code_map[pos] = *code;
    }

    println!("");
    println!("pub const HI_MAP_{}: [usize; {}] = [", part, hi_map.len());
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
    println!("pub const ENCODE_MAP_{}: [u8; {}] = [", part, code_map.len());
    for (n, &code) in code_map.iter().enumerate() {
        if (n % 8) == 0 {
            if n > 0 {
                println!("");
            }
            print!("    ");
        } else {
            print!(" ");
        }

        print!("0x{:02x},", code);
    }
    println!("");
    println!("];");

    Ok(())
}

fn main() -> io::Result<()> {
    let base_path = std::env::current_exe()?;
    let base_path = base_path.parent().unwrap();
    let base_path = base_path.join("data");

    let map = vec![
        (1, "Western European"),
        (2, "Central European"),
        (3, "South European"),
        (4, "North European"),
        (5, "Cyrillic"),
        (6, "Arabic"),
        (7, "Greek"),
        (8, "Hebrew"),
        (9, "Turkish"),
        (10, "Nordic"),
        (11, "Thai"),
        (13, "Baltic Rim"),
        (14, "Celtic"),
        (15, "Western European"),
        (16, "South-Eastern European"),
    ];

    println!("//! File generated with tools/iso8859/parser.rs");

    for i in &map {
        println!("");
        println!("/// {}", i.1);
        read_file(base_path.join(format!("8859-{}.TXT", i.0)), i.0)?;
    }

    Ok(())
}
