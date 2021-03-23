use std::fs::File;
use std::env;
use std::io::Read;

use std::{fmt::Write, num::ParseIntError};

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}


fn main()
{
	let args = env::args().collect::<Vec<_>>();
	let wasm_file = match args.len() {
		2 => (&args[1]),
		_ => {
			println!("Usage: {} gen <project_path> <contract_name> <lib.rs path>", args[0]);
			return;
		},
	};    
    let mut file=File::open(format!("{}",wasm_file)).unwrap();
    let mut buf=Vec::new();
    file.read_to_end(&mut buf).unwrap();
    println!("0x{}",encode_hex(&buf));
    // use file
}