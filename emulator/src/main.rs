use std::env;
use std::fs;

enum Registers {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    M,
    Sp,
}

#[derive(Default)]
struct ConditionCodes {
    z: bool,
    s: bool,
    p: bool,
    cy: bool,
    ac: bool,
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

        let status = match opcode {
            0x00 => Ok(0),
            0x01 => self.op_lxi(Registers::B),

            0x11 => self.op_lxi(Registers::D),

            0x21 => self.op_lxi(Registers::H),

            0x31 => self.op_lxi(Registers::Sp),

            0x80 => self.op_add(Registers::B),
            0x81 => self.op_add(Registers::C),
            0x82 => self.op_add(Registers::D),
            0x83 => self.op_add(Registers::E),
            0x84 => self.op_add(Registers::H),
            0x85 => self.op_add(Registers::L),
            0x86 => self.op_add(Registers::M),
            0x87 => self.op_add(Registers::A),

            0xc6 => self.op_adi(),

            0xce => self.op_aci(),

            _ => Err("Unimplemented Opcode".to_string()),
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

    // Missing ac (Auxillary Carry) flag, but this is only used for one operation (DAA) so will be
    // manually set there
    fn flags(&mut self, val: u16) {
        // Zero flag
        self.cc.z = val & 0xff == 0;
        // Sign flag
        self.cc.s = val & 0x80 != 0;
        // Carry flag
        self.cc.cy = val > 0xff;
        // Parity flag
        self.cc.p = parity(val & 0xff);
    }

    fn op_add(&mut self, reg: Registers) -> Result<u8, String> {
        // Going to use 16 bit maths to emulate 8 bit maths. Makes it easier to tell when we spill
        // to a number bigger than 8 bits. Wll involve a lot of casting back and forth.
        let mut err_flag = false;
        let mut total: u16 = self.a as u16;

        match reg {
            Registers::B => total += self.b as u16,
            Registers::C => total += self.c as u16,
            Registers::D => total += self.d as u16,
            Registers::E => total += self.e as u16,
            Registers::H => total += self.h as u16,
            Registers::L => total += self.l as u16,
            Registers::M => {
                // M references a specific memory addr - treat H/L as a 16 bit addr
                let ptr = (self.h as u16) << 8 | self.l as u16;
                total += self.memory[ptr as usize] as u16;
            }
            Registers::A => total += self.a as u16,
            _ => err_flag = true,
        }

        // flags function modifies relevant flags based off of total
        self.flags(total);

        // Cast to u8 truncates first 8 bits - good because we don't care, the carry flag handles
        // if we went over 31
        self.a = total as u8;

        if err_flag {
            Err("Bad register passed to ADD".to_string())
        } else {
            Ok(0)
        }
    }

    fn op_adi(&mut self) -> Result<u8, String> {
        self.pc += 1;

        let total: u16 = (self.a as u16) + (self.memory[self.pc as usize] as u16);

        // Pass to flags to check output
        self.flags(total);
        self.a = total as u8;

        Ok(0)
    }

    fn op_aci(&mut self) -> Result<u8, String> {
        self.pc += 1;

        let total = if self.cc.cy {
            (self.a as u16) + (self.memory[self.pc as usize] as u16) + 1
        } else {
            (self.a as u16) + (self.memory[self.pc as usize] as u16)
        };

        // Pass to flags to check ouput
        self.flags(total);
        self.a = total as u8;

        Ok(0)
    }
}

fn parity(num: u16) -> bool {
    num.count_ones().is_multiple_of(2)
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
