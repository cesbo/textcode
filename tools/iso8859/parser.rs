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


const ARR_SIZE: usize = 0xFF - 0x9F;


fn read_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let mut arr: Vec<u16> = vec![0x0000; ARR_SIZE];

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
        let code = code - 0xA0;
        let unicode = split.next().unwrap();
        let unicode = u16::from_str_radix(&unicode[2 ..], 16).unwrap();
        arr[code as usize] = unicode;
    }

    println!("    const DATA: [u16; {}] = [", ARR_SIZE);
    for (n, unicode) in arr.iter().enumerate() {
        if (n % 8) == 0 {
            if n > 0 {
                println!("");
            }
            print!("        ");
        } else {
            print!(" ");
        }
        print!("0x{:04x},", unicode);
    }
    println!("");
    println!("    ];");

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

    for i in &map {
        if i.0 > 1 {
            print!("\n\n");
        }

        println!("/// {}", i.1);
        println!("pub mod iso8859_{} {{", &i.0);
        println!("    use crate::charset::iso6937::{{");
        println!("        iso6937_encode,");
        println!("        iso6937_decode,");
        println!("    }};");
        println!("");

        read_file(base_path.join(format!("8859-{}.TXT", i.0)))?;

        println!("");
        println!("    #[inline]");
        println!("    pub fn encode(src: &str, dst: &mut Vec<u8>) {{ iso6937_encode(src, dst, &DATA) }}");
        println!("");
        println!("    #[inline]");
        println!("    pub fn decode(src: &[u8], dst: &mut String) {{ iso6937_decode(src, dst, &DATA) }}");
        println!("}}");
    }

    Ok(())
}
