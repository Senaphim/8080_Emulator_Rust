use crate::state::{Registers, State8080};

impl State8080 {
    // MEMORY LOADS AND MOVES
    // --------------------------------------------------------------------------------------------
    pub fn op_lxi(&mut self, reg: Registers) -> Result<u8, String> {
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

    pub fn op_mvi(&mut self, reg: Registers) -> Result<u8, String> {
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

    pub fn op_ldax(&mut self, reg: Registers) -> Result<u8, String> {
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

    pub fn op_mov(&mut self, dest: Registers, from: Registers) -> Result<u8, String> {
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
}
