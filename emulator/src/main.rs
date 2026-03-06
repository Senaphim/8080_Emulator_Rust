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

enum Flags {
    Z,
    Nz,
    S,
    Ns,
    P,
    Np,
    Cy,
    Ncy,
    // Ac,
    None,
}

#[derive(Default)]
struct ConditionCodes {
    z: bool,
    s: bool,
    p: bool,
    cy: bool,
    ac: bool,
    // pad: u8,
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
    // int_enable: u8,
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
            // int_enable: 0x00,
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

            0x06 => self.op_mvi(Registers::B),

            0x0e => self.op_mvi(Registers::C),

            0x11 => self.op_lxi(Registers::D),

            0x16 => self.op_mvi(Registers::D),

            0x1e => self.op_mvi(Registers::E),

            0x21 => self.op_lxi(Registers::H),

            0x26 => self.op_lxi(Registers::H),

            0x2e => self.op_lxi(Registers::L),

            0x31 => self.op_lxi(Registers::Sp),

            0x36 => self.op_lxi(Registers::M),

            0x80 => self.op_add(Registers::B),
            0x81 => self.op_add(Registers::C),
            0x82 => self.op_add(Registers::D),
            0x83 => self.op_add(Registers::E),
            0x84 => self.op_add(Registers::H),
            0x85 => self.op_add(Registers::L),
            0x86 => self.op_add(Registers::M),
            0x87 => self.op_add(Registers::A),
            0x88 => self.op_adc(Registers::B),
            0x89 => self.op_adc(Registers::C),
            0x8a => self.op_adc(Registers::D),
            0x8b => self.op_adc(Registers::E),
            0x8c => self.op_adc(Registers::H),
            0x8d => self.op_adc(Registers::L),
            0x8e => self.op_adc(Registers::M),
            0x8f => self.op_adc(Registers::A),

            0xc2 => self.op_jmp(Flags::Nz),
            0xc3 => self.op_jmp(Flags::None),
            0xc4 => self.op_call(Flags::Nz),

            0xc6 => self.op_adi(),

            0xca => self.op_jmp(Flags::Z),

            0xcc => self.op_call(Flags::Z),
            0xcd => self.op_call(Flags::None),
            0xce => self.op_aci(),

            0xd2 => self.op_jmp(Flags::Ncy),

            0xd4 => self.op_call(Flags::Ncy),

            0xda => self.op_jmp(Flags::Cy),

            0xdc => self.op_call(Flags::Cy),

            0xe2 => self.op_jmp(Flags::Np),

            0xe4 => self.op_call(Flags::Np),

            0xea => self.op_jmp(Flags::P),

            0xec => self.op_call(Flags::P),

            0xf2 => self.op_jmp(Flags::S),

            0xf4 => self.op_call(Flags::S),

            0xfa => self.op_jmp(Flags::Ns),

            0xfc => self.op_call(Flags::Ns),

            _ => Err(format!("Unimplemented opcode: {opcode:04x}")),
        };

        self.pc += 1;
        println!("{:004x}", self.pc);
        status
    }

    // MEMORY LOADS
    // --------------------------------------------------------------------------------------------
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
                let lower: u16 = self.memory[self.pc as usize] as u16;
                self.pc += 1;
                let upper: u16 = self.memory[self.pc as usize] as u16;
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

    fn op_mvi(&mut self, reg: Registers) -> Result<u8, String> {
        let mut err_flag = false;

        match reg {
            Registers::B => {
                self.pc += 1;
                self.b = self.memory[self.pc as usize]
            }
            Registers::C => {
                self.pc += 1;
                self.c = self.memory[self.pc as usize]
            }
            Registers::D => {
                self.pc += 1;
                self.d = self.memory[self.pc as usize]
            }
            Registers::E => {
                self.pc += 1;
                self.e = self.memory[self.pc as usize]
            }
            Registers::H => {
                self.pc += 1;
                self.h = self.memory[self.pc as usize]
            }
            Registers::L => {
                self.pc += 1;
                self.l = self.memory[self.pc as usize]
            }
            Registers::M => {
                // M references a specific memory addr - treat H/L as a 16 bit addr
                let ptr = (self.h as u16) << 8 | self.l as u16;
                self.pc += 1;
                self.memory[ptr as usize] = self.memory[self.pc as usize]
            }
            Registers::A => {
                self.pc += 1;
                self.a = self.memory[self.pc as usize]
            }
            _ => err_flag = true,
        }

        if err_flag {
            Err("Bad register passed to MVI".to_string())
        } else {
            Ok(0)
        }
    }

    // PROGRAM CONTROL
    // --------------------------------------------------------------------------------------------
    fn op_jmp(&mut self, flg: Flags) -> Result<u8, String> {
        let mut err_flag = false;

        self.pc += 1;
        let lower = self.memory[self.pc as usize] as u16;
        self.pc += 1;
        let upper = self.memory[self.pc as usize] as u16;
        // Minus 1 here as we are going to add one as part of the parent calling function. Need to
        // make sure that the pointer ends up at the right address
        let ptr = (upper << 8 | lower) - 1;

        match flg {
            Flags::Z => {
                if self.cc.z {
                    self.pc = ptr
                }
            }
            Flags::Nz => {
                if !self.cc.z {
                    self.pc = ptr
                }
            }
            Flags::S => {
                if self.cc.s {
                    self.pc = ptr
                }
            }
            Flags::Ns => {
                if !self.cc.s {
                    self.pc = ptr
                }
            }
            Flags::P => {
                if self.cc.p {
                    self.pc = ptr
                }
            }
            Flags::Np => {
                if !self.cc.p {
                    self.pc = ptr
                }
            }
            Flags::Cy => {
                if self.cc.cy {
                    self.pc = ptr
                }
            }
            Flags::Ncy => {
                if !self.cc.cy {
                    self.pc = ptr
                }
            }
            Flags::None => self.pc = ptr,
            _ => err_flag = true,
        }

        if err_flag {
            Err("Bad flag passed to jump".to_string())
        } else {
            Ok(0)
        }
    }

    fn op_call(&mut self, flg: Flags) -> Result<u8, String> {
        let mut err_flag = false;

        let pc_upper = ((self.pc & 0xff00) >> 8) as u8;
        let pc_lower = (self.pc & 0x00ff) as u8;

        self.pc += 1;
        let lower = self.memory[self.pc as usize] as u16;
        self.pc += 1;
        let upper = self.memory[self.pc as usize] as u16;
        // Minus 1 here as we are going to add one as part of the parent calling function. Need to
        // make sure that the pointer ends up at the right address
        let ptr = (upper << 8 | lower) - 1;

        match flg {
            Flags::Z => {
                if self.cc.z {
                    self.call(pc_upper, pc_lower, ptr)
                }
            }
            Flags::Nz => {
                if !self.cc.z {
                    self.call(pc_upper, pc_lower, ptr)
                }
            }
            Flags::S => {
                if self.cc.s {
                    self.call(pc_upper, pc_lower, ptr)
                }
            }
            Flags::Ns => {
                if !self.cc.s {
                    self.call(pc_upper, pc_lower, ptr)
                }
            }
            Flags::P => {
                if self.cc.p {
                    self.call(pc_upper, pc_lower, ptr)
                }
            }
            Flags::Np => {
                if !self.cc.p {
                    self.call(pc_upper, pc_lower, ptr)
                }
            }
            Flags::Cy => {
                if self.cc.cy {
                    self.call(pc_upper, pc_lower, ptr)
                }
            }
            Flags::Ncy => {
                if !self.cc.cy {
                    self.call(pc_upper, pc_lower, ptr)
                }
            }
            Flags::None => self.call(pc_upper, pc_lower, ptr),
            _ => err_flag = true,
        }

        if err_flag {
            Err("Bad flag passed to call".to_string())
        } else {
            Ok(0)
        }
    }

    fn call(&mut self, pc_u: u8, pc_l: u8, ptr: u16) {
        self.memory[(self.sp as usize) - 1] = pc_u;
        self.memory[(self.sp as usize) - 2] = pc_l;
        self.sp -= 2;
        self.pc = ptr;
    }

    // ARITHMETIC
    // --------------------------------------------------------------------------------------------
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

    // Function for calculating ac flag. Need to bit mask bits 4-7 (this also masks bits 7-15 as
    // well as we are using 16 bit maths to emulate 8 bit) to work out if ac it activated
    fn ac_flag(&mut self, val: u16) {
        let accumulator = (self.a as u16) & 0xf;
        let adder = val & 0xf;
        let total = accumulator + adder;

        self.cc.ac = total > 0xf;
    }

    fn op_add(&mut self, reg: Registers) -> Result<u8, String> {
        // Going to use 16 bit maths to emulate 8 bit maths. Makes it easier to tell when we spill
        // to a number bigger than 8 bits. Wll involve a lot of casting back and forth.
        let mut err_flag = false;
        let mut total: u16 = self.a as u16;

        let adder = match reg {
            Registers::B => self.b as u16,
            Registers::C => self.c as u16,
            Registers::D => self.d as u16,
            Registers::E => self.e as u16,
            Registers::H => self.h as u16,
            Registers::L => self.l as u16,
            Registers::M => {
                // M references a specific memory addr - treat H/L as a 16 bit addr
                let ptr = (self.h as u16) << 8 | self.l as u16;
                self.memory[ptr as usize] as u16
            }
            Registers::A => self.a as u16,
            _ => {
                err_flag = true;
                0
            }
        };

        total += adder;

        // Check ac flag. This is changed by every arithmetic operation but only affects one
        // instruction...
        self.ac_flag(adder);

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

    fn op_adc(&mut self, reg: Registers) -> Result<u8, String> {
        // As above, 16 bit maths used to emulate 8 bit
        let mut err_flag = false;
        let mut total: u16 = self.a as u16;
        let carry = if self.cc.cy { 1 } else { 0 };

        let adder = match reg {
            Registers::B => (self.b as u16) + carry,
            Registers::C => (self.c as u16) + carry,
            Registers::D => (self.d as u16) + carry,
            Registers::E => (self.e as u16) + carry,
            Registers::H => (self.h as u16) + carry,
            Registers::L => (self.l as u16) + carry,
            Registers::M => {
                // M references a specific memory addr - treat H/L as a 16 bit addr
                let ptr = (self.h as u16) << 8 | self.l as u16;
                (self.memory[ptr as usize] as u16) + carry
            }
            Registers::A => (self.a as u16) + carry,
            _ => {
                err_flag = true;
                0
            }
        };

        total += adder;

        // Check ac flag
        self.ac_flag(adder);

        // Pass to flags for check
        self.flags(total);

        // Cast to u8 and and assign back to a (accumulator) register
        self.a = total as u8;

        if err_flag {
            Err("Bad register passed to ADC".to_string())
        } else {
            Ok(0)
        }
    }

    fn op_adi(&mut self) -> Result<u8, String> {
        self.pc += 1;

        let adder = self.memory[self.pc as usize] as u16;
        let total: u16 = (self.a as u16) + adder;

        // Pass to flags to check output
        self.ac_flag(adder);
        self.flags(total);
        self.a = total as u8;

        Ok(0)
    }

    fn op_aci(&mut self) -> Result<u8, String> {
        self.pc += 1;

        let adder = if self.cc.cy {
            self.memory[self.pc as usize] as u16 + 1
        } else {
            self.memory[self.pc as usize] as u16
        };

        let total = (self.a as u16) + adder;

        // Pass to flags to check ouput
        self.ac_flag(adder);
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
