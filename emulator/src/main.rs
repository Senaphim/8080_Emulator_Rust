pub mod state;
mod test;

use std::env;
use std::fs;

use crate::state::State8080;

fn main() {
    let argv: Vec<String> = env::args().collect();

    // let opt = &argv[1];
    let filepath = &argv[argv.len() - 1];

    // let mut dest: &String = &"".to_string();
    // let mut to_file = false;
    //
    // if opt == "-f" {
    //     to_file = true;
    //     dest = &argv[2];
    // }

    let mut processor = State8080::default();

    let rom: Vec<u8> = fs::read(filepath).expect("Should have been able to read the file");

    processor.memory[..rom.len()].clone_from_slice(&rom);

    loop {
        let ticks = processor.emulate_8080();

        match ticks {
            Ok(_) => (),
            Err(e) => {
                println!("8080 STATE");
                println!(
                    "--------------------------------------------------------------------------------"
                );
                println!("Registers:");
                println!("B: {:02x}", processor.b);
                println!("C: {:02x}", processor.c);
                println!("D: {:02x}", processor.d);
                println!("E: {:02x}", processor.e);
                println!("H: {:02x}", processor.h);
                println!("L: {:02x}", processor.l);
                println!("A: {:02x}", processor.a);

                println!("\nPointers:");
                println!("Stack Pointer:   {:04x}", processor.sp);
                println!("Program Counter: {:04x}", processor.pc);

                println!("\nFlags:");
                println!("Zero:   {}", processor.cc.z);
                println!("Sign:   {}", processor.cc.s);
                println!("Parity: {}", processor.cc.p);
                println!("Carry:  {}", processor.cc.cy);
                println!("Carry2: {}", processor.cc.ac);

                panic!("Error: {}", e)
            }
        }
    }
}
