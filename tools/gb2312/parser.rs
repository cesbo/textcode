//! GB2312 Parser
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


fn read_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let hi_limit = 0x77;
    let hi_size = hi_limit - 0x21 + 1;
    let lo_size = 0x7F - 0x21;
    let mut arr: Vec<u16> = vec![0; hi_size * lo_size];

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

        let code = ((hi - 0x21) * lo_size) + (lo - 0x21);
        arr[code as usize] = unicode;
    }

    println!("const HI_LIMIT: usize = {};", hi_limit);
    println!("const LO_SIZE: usize = {};", lo_size);
    println!("");
    println!("const DATA: [u16; {}] = [", arr.len());
    for (n, &unicode) in arr.iter().enumerate() {
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

    Ok(())
}

fn main() -> io::Result<()> {
    let base_path = std::env::current_exe()?;
    let base_path = base_path.parent().unwrap();
    let base_path = base_path.join("data");


    println!("/// Simplified Chinese");
    read_file(base_path.join("GB2312.TXT"))?;

    Ok(())
}
