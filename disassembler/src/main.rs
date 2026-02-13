use std::env;
use std::fs;

fn main() {
    let argv: Vec<String> = env::args().collect();

    let opt = &argv[1];
    let filepath = &argv[argv.len() - 1];

    let mut dest: &String = &"".to_string();
    let mut to_file = false;

    if opt == "-f" {
        to_file = true;
        dest = &argv[2]
    }

    let raw_bytes = fs::read(filepath).expect("Should have been able to read the file");
    let out = disassemble(raw_bytes);

    if to_file {
        fs::write(dest, out).expect("Should have been able to write to file");
    } else {
        println!("{out}");
    }
}

fn disassemble(raw_bytes: Vec<u8>) -> String {
    "success".to_string()
}
