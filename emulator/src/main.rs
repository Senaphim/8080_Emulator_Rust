use std::env;
use std::fs;

enum Registers {
    A,
    B,
    C,
    D,
    H,
    L,
    Sp,
}

#[derive(Default)]
struct ConditionCodes {
    z: u8,
    s: u8,
    p: u8,
    cy: u8,
    ac: u8,
    pad: u8,
}

struct State8080 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    int_enable: u8,
    cc: ConditionCodes,
    memory: Vec<u8>,
}

impl Default for State8080 {
    fn default() -> Self {
        State8080 {
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
            sp: 0x00,
            pc: 0x00,
            int_enable: 0x00,
            cc: ConditionCodes::default(),
            memory: vec![0x00],
        }
    }
}

impl State8080 {
    fn emulate_8080(&mut self) -> Result<u8, String> {
        let opcode = self.memory[self.pc as usize];
        let status: Result<u8, String>;

        match opcode {
            0x00 => status = Ok(0),
            0x01 => status = self.op_lxi(Registers::B),

            0x11 => status = self.op_lxi(Registers::D),

            0x21 => status = self.op_lxi(Registers::H),

            0x31 => status = self.op_lxi(Registers::Sp),

            _ => status = Err("Unimplemented Opcode".to_string()),
        };

        self.pc += 1;
        status
    }

    fn op_lxi(&mut self, reg: Registers) -> Result<u8, String> {
        let mut err_flag = false;

        match reg {
            Registers::B => {
                self.pc += 1;
                self.c = self.memory[self.pc as usize];
                self.pc += 1;
                self.b = self.memory[self.pc as usize];
            }
            Registers::D => {
                self.pc += 1;
                self.l = self.memory[self.pc as usize];
                self.pc += 1;
                self.d = self.memory[self.pc as usize];
            }
            Registers::H => {
                self.pc += 1;
                self.l = self.memory[self.pc as usize];
                self.pc += 1;
                self.h = self.memory[self.pc as usize];
            }
            Registers::Sp => {
                self.pc += 1;
                let upper: u16 = self.memory[self.pc as usize] as u16;
                self.pc += 1;
                let lower: u16 = self.memory[self.pc as usize] as u16;
                self.sp = upper << 8 | lower
            }
            _ => err_flag = true,
        }

        if err_flag {
            Err("Bad register passed to LXI".to_string())
        } else {
            Ok(0)
        }
    }
}

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

    processor.memory = fs::read(filepath).expect("Should have been able to read the file");

    loop {
        let ticks = processor.emulate_8080();

        match ticks {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
