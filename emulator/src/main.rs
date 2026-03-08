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
    Ac,
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
            memory: vec![0x00; 0xffff],
        }
    }
}

impl State8080 {
    fn emulate_8080(&mut self) -> Result<u8, String> {
        let opcode = self.memory[self.pc as usize];

        let status = match opcode {
            0x00 => Ok(0),
            0x01 => self.op_lxi(Registers::B),

            0x03 => self.op_inx(Registers::B),
            0x04 => self.op_inr(Registers::B),
            0x05 => self.op_dcr(Registers::B),
            0x06 => self.op_mvi(Registers::B),

            0x08 => Ok(0),

            0x0a => self.op_ldax(Registers::B),

            0x0c => self.op_inr(Registers::C),
            0x0d => self.op_dcr(Registers::C),
            0x0e => self.op_mvi(Registers::C),

            0x10 => Ok(0),
            0x11 => self.op_lxi(Registers::D),

            0x13 => self.op_inx(Registers::D),
            0x14 => self.op_inr(Registers::D),
            0x15 => self.op_dcr(Registers::D),
            0x16 => self.op_mvi(Registers::D),

            0x18 => Ok(0),

            0x1a => self.op_ldax(Registers::D),

            0x1c => self.op_inr(Registers::E),
            0x1d => self.op_dcr(Registers::E),
            0x1e => self.op_mvi(Registers::E),

            0x20 => Ok(0),
            0x21 => self.op_lxi(Registers::H),

            0x23 => self.op_inx(Registers::H),
            0x24 => self.op_inr(Registers::H),
            0x25 => self.op_dcr(Registers::H),
            0x26 => self.op_lxi(Registers::H),

            0x28 => Ok(0),

            0x2c => self.op_inr(Registers::L),
            0x2d => self.op_dcr(Registers::L),
            0x2e => self.op_lxi(Registers::L),

            0x30 => Ok(0),
            0x31 => self.op_lxi(Registers::Sp),

            0x33 => self.op_inx(Registers::Sp),
            0x34 => self.op_inr(Registers::M),
            0x35 => self.op_dcr(Registers::M),
            0x36 => self.op_lxi(Registers::M),

            0x38 => Ok(0),

            0x3c => self.op_inr(Registers::A),
            0x3d => self.op_dcr(Registers::A),

            0x40 => self.op_mov(Registers::B, Registers::B),
            0x41 => self.op_mov(Registers::B, Registers::C),
            0x42 => self.op_mov(Registers::B, Registers::D),
            0x43 => self.op_mov(Registers::B, Registers::E),
            0x44 => self.op_mov(Registers::B, Registers::H),
            0x45 => self.op_mov(Registers::B, Registers::L),
            0x46 => self.op_mov(Registers::B, Registers::M),
            0x47 => self.op_mov(Registers::B, Registers::A),
            0x48 => self.op_mov(Registers::C, Registers::B),
            0x49 => self.op_mov(Registers::C, Registers::C),
            0x4a => self.op_mov(Registers::C, Registers::D),
            0x4b => self.op_mov(Registers::C, Registers::E),
            0x4c => self.op_mov(Registers::C, Registers::H),
            0x4d => self.op_mov(Registers::C, Registers::L),
            0x4e => self.op_mov(Registers::C, Registers::M),
            0x4f => self.op_mov(Registers::C, Registers::A),
            0x50 => self.op_mov(Registers::D, Registers::B),
            0x51 => self.op_mov(Registers::D, Registers::C),
            0x52 => self.op_mov(Registers::D, Registers::D),
            0x53 => self.op_mov(Registers::D, Registers::E),
            0x54 => self.op_mov(Registers::D, Registers::H),
            0x55 => self.op_mov(Registers::D, Registers::L),
            0x56 => self.op_mov(Registers::D, Registers::M),
            0x57 => self.op_mov(Registers::D, Registers::A),
            0x58 => self.op_mov(Registers::E, Registers::B),
            0x59 => self.op_mov(Registers::E, Registers::C),
            0x5a => self.op_mov(Registers::E, Registers::D),
            0x5b => self.op_mov(Registers::E, Registers::E),
            0x5c => self.op_mov(Registers::E, Registers::H),
            0x5d => self.op_mov(Registers::E, Registers::L),
            0x5e => self.op_mov(Registers::E, Registers::M),
            0x5f => self.op_mov(Registers::E, Registers::A),
            0x60 => self.op_mov(Registers::H, Registers::B),
            0x61 => self.op_mov(Registers::H, Registers::C),
            0x62 => self.op_mov(Registers::H, Registers::D),
            0x63 => self.op_mov(Registers::H, Registers::E),
            0x64 => self.op_mov(Registers::H, Registers::H),
            0x65 => self.op_mov(Registers::H, Registers::L),
            0x66 => self.op_mov(Registers::H, Registers::M),
            0x67 => self.op_mov(Registers::H, Registers::A),
            0x68 => self.op_mov(Registers::L, Registers::B),
            0x69 => self.op_mov(Registers::L, Registers::C),
            0x6a => self.op_mov(Registers::L, Registers::D),
            0x6b => self.op_mov(Registers::L, Registers::E),
            0x6c => self.op_mov(Registers::L, Registers::H),
            0x6d => self.op_mov(Registers::L, Registers::L),
            0x6e => self.op_mov(Registers::L, Registers::M),
            0x6f => self.op_mov(Registers::L, Registers::A),
            0x70 => self.op_mov(Registers::M, Registers::B),
            0x71 => self.op_mov(Registers::M, Registers::C),
            0x72 => self.op_mov(Registers::M, Registers::D),
            0x73 => self.op_mov(Registers::M, Registers::E),
            0x74 => self.op_mov(Registers::M, Registers::H),
            0x75 => self.op_mov(Registers::M, Registers::L),

            0x77 => self.op_mov(Registers::M, Registers::A),
            0x78 => self.op_mov(Registers::A, Registers::B),
            0x79 => self.op_mov(Registers::A, Registers::C),
            0x7a => self.op_mov(Registers::A, Registers::D),
            0x7b => self.op_mov(Registers::A, Registers::E),
            0x7c => self.op_mov(Registers::A, Registers::H),
            0x7d => self.op_mov(Registers::A, Registers::L),
            0x7e => self.op_mov(Registers::A, Registers::M),
            0x7f => self.op_mov(Registers::A, Registers::A),
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

            0xc0 => self.op_ret(Flags::Nz),

            0xc2 => self.op_jmp(Flags::Nz),
            0xc3 => self.op_jmp(Flags::None),
            0xc4 => self.op_call(Flags::Nz),

            0xc6 => self.op_adi(),

            0xc8 => self.op_ret(Flags::Z),
            0xc9 => self.op_ret(Flags::None),
            0xca => self.op_jmp(Flags::Z),
            0xcb => Ok(0),
            0xcc => self.op_call(Flags::Z),
            0xcd => self.op_call(Flags::None),
            0xce => self.op_aci(),

            0xd0 => self.op_ret(Flags::Ncy),

            0xd2 => self.op_jmp(Flags::Ncy),

            0xd4 => self.op_call(Flags::Ncy),

            0xd8 => self.op_ret(Flags::Cy),
            0xd9 => Ok(0),
            0xda => self.op_jmp(Flags::Cy),

            0xdc => self.op_call(Flags::Cy),
            0xdd => Ok(0),

            0xe0 => self.op_jmp(Flags::Np),

            0xe2 => self.op_jmp(Flags::Np),

            0xe4 => self.op_call(Flags::Np),

            0xe8 => self.op_ret(Flags::P),

            0xea => self.op_jmp(Flags::P),

            0xec => self.op_call(Flags::P),

            0xf0 => self.op_ret(Flags::S),

            0xf2 => self.op_jmp(Flags::S),

            0xf4 => self.op_call(Flags::S),

            0xf8 => self.op_ret(Flags::Ns),

            0xfa => self.op_jmp(Flags::Ns),

            0xfc => self.op_call(Flags::Ns),

            _ => Err(format!("Unimplemented opcode: {opcode:04x}")),
        };

        self.pc += 1;
        status
    }

    // MEMORY LOADS AND MOVES
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
                self.e = self.memory[self.pc as usize];
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

    fn op_ldax(&mut self, reg: Registers) -> Result<u8, String> {
        let mut err_flag = false;

        match reg {
            Registers::D => {
                let upper = self.d as u16;
                let lower = self.e as u16;
                let ptr = (upper << 8) | lower;
                self.a = self.memory[ptr as usize];
            }
            Registers::B => {
                let upper = self.b as u16;
                let lower = self.c as u16;
                let ptr = (upper << 8) | lower;
                self.a = self.memory[ptr as usize];
            }
            _ => err_flag = true,
        }

        if err_flag {
            Err("Bad register passed to LDAX".to_string())
        } else {
            Ok(0)
        }
    }

    fn op_mov(&mut self, dest: Registers, from: Registers) -> Result<u8, String> {
        let mut err_flag = false;

        let from_val = match from {
            Registers::B => self.b,
            Registers::C => self.c,
            Registers::D => self.d,
            Registers::E => self.e,
            Registers::H => self.h,
            Registers::L => self.l,
            Registers::M => self.memory[((self.h as u16) << 8 | (self.l as u16)) as usize],
            Registers::A => self.a,
            _ => {
                err_flag = true;
                0
            }
        };

        match dest {
            Registers::B => self.b = from_val,
            Registers::C => self.c = from_val,
            Registers::D => self.d = from_val,
            Registers::E => self.e = from_val,
            Registers::H => self.h = from_val,
            Registers::L => self.l = from_val,
            Registers::M => {
                self.memory[((self.h as u16) << 8 | (self.l as u16)) as usize] = from_val
            }
            Registers::A => self.a = from_val,
            _ => err_flag = true,
        }

        if err_flag {
            Err("Bad flag passed to MOV".to_string())
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

    fn op_ret(&mut self, flg: Flags) -> Result<u8, String> {
        let mut err_flag = false;

        let lower = self.memory[self.sp as usize];
        let upper = self.memory[(self.sp as usize) + 1];

        match flg {
            Flags::Z => {
                if self.cc.z {
                    self.ret(upper, lower)
                }
            }
            Flags::Nz => {
                if !self.cc.z {
                    self.ret(upper, lower)
                }
            }
            Flags::S => {
                if self.cc.s {
                    self.ret(upper, lower)
                }
            }
            Flags::Ns => {
                if !self.cc.s {
                    self.ret(upper, lower)
                }
            }
            Flags::P => {
                if self.cc.p {
                    self.ret(upper, lower)
                }
            }
            Flags::Np => {
                if !self.cc.p {
                    self.ret(upper, lower)
                }
            }
            Flags::Cy => {
                if self.cc.cy {
                    self.ret(upper, lower)
                }
            }
            Flags::Ncy => {
                if !self.cc.cy {
                    self.ret(upper, lower)
                }
            }
            Flags::None => self.ret(upper, lower),
            _ => err_flag = true,
        }

        if err_flag {
            Err("Bad flag passed to RET".to_string())
        } else {
            Ok(0)
        }
    }

    fn ret(&mut self, pc_u: u8, pc_l: u8) {
        self.pc = (pc_u as u16) << 8 | pc_l as u16;
        self.sp += 2
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

    fn op_inx(&mut self, reg: Registers) -> Result<u8, String> {
        let mut err_flag = false;

        let mut val = match reg {
            Registers::B => (self.b as u16) << 8 | self.c as u16,
            Registers::D => (self.d as u16) << 8 | self.e as u16,
            Registers::H => (self.h as u16) << 8 | self.l as u16,
            Registers::Sp => self.sp,
            _ => {
                err_flag = true;
                0
            }
        };

        val += 1;
        let upper = ((val & 0xff00) >> 8) as u8;
        let lower = (val & 0x00ff) as u8;

        match reg {
            Registers::B => {
                self.b = upper;
                self.c = lower;
            }
            Registers::D => {
                self.d = upper;
                self.e = lower;
            }
            Registers::H => {
                self.h = upper;
                self.l = lower;
            }
            Registers::Sp => self.sp = val,
            _ => err_flag = true,
        }

        if err_flag {
            Err("Bad flag passed to INX".to_string())
        } else {
            Ok(0)
        }
    }

    // TODO: Missing AC flag evaluation as current implementation means that AC flag is only
    // calculated for the A register
    fn op_inr(&mut self, reg: Registers) -> Result<u8, String> {
        let mut err_flag = false;

        let mut total = match reg {
            Registers::B => self.b as u16,
            Registers::C => self.c as u16,
            Registers::D => self.d as u16,
            Registers::E => self.e as u16,
            Registers::H => self.h as u16,
            Registers::L => self.l as u16,
            Registers::M => self.memory[((self.h as u16) << 8 | (self.l as u16)) as usize] as u16,
            Registers::A => self.a as u16,
            _ => {
                err_flag = true;
                0
            }
        };

        // Store carry flag as not checked for inr or dcr
        let carry = self.cc.cy;
        total = total.wrapping_add(1);

        // Check flags
        self.flags(total);

        // Write carry flag back
        self.cc.cy = carry;

        match reg {
            Registers::B => self.b = total as u8,
            Registers::C => self.c = total as u8,
            Registers::D => self.d = total as u8,
            Registers::E => self.e = total as u8,
            Registers::H => self.h = total as u8,
            Registers::L => self.l = total as u8,
            Registers::M => {
                self.memory[((self.h as u16) << 8 | (self.l as u16)) as usize] = total as u8
            }
            Registers::A => self.a = total as u8,
            _ => err_flag = true,
        }

        if err_flag {
            Err("Bad flag passed to INR".to_string())
        } else {
            Ok(0)
        }
    }

    // TODO: Missing AC flag evaluation as current implementation means that AC flag is only
    // calculated for the A register
    fn op_dcr(&mut self, reg: Registers) -> Result<u8, String> {
        let mut err_flag = false;

        let mut total = match reg {
            Registers::B => self.b as u16,
            Registers::C => self.c as u16,
            Registers::D => self.d as u16,
            Registers::E => self.e as u16,
            Registers::H => self.h as u16,
            Registers::L => self.l as u16,
            Registers::M => self.memory[((self.h as u16) << 8 | (self.l as u16)) as usize] as u16,
            Registers::A => self.a as u16,
            _ => {
                err_flag = true;
                0
            }
        };

        // Store carry flag as not checked for inr or dcr
        let carry = self.cc.cy;
        total = total.wrapping_sub(1);

        // Check flags
        self.flags(total);

        // Write carry flag back
        self.cc.cy = carry;

        match reg {
            Registers::B => self.b = total as u8,
            Registers::C => self.c = total as u8,
            Registers::D => self.d = total as u8,
            Registers::E => self.e = total as u8,
            Registers::H => self.h = total as u8,
            Registers::L => self.l = total as u8,
            Registers::M => {
                self.memory[((self.h as u16) << 8 | (self.l as u16)) as usize] = total as u8
            }
            Registers::A => self.a = total as u8,
            _ => err_flag = true,
        }

        if err_flag {
            Err("Bad flag passed to INR".to_string())
        } else {
            Ok(0)
        }
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
