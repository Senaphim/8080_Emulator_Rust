use crate::state::{Registers, State8080};

impl State8080 {
    // LOGICAL OPERATORS
    // --------------------------------------------------------------------------------------------
    pub fn op_and(&mut self, reg: Registers) -> Result<u8, String> {
        let mut err_flag = false;

        let total = match reg {
            Registers::B => self.a & self.b,
            Registers::C => self.a & self.c,
            Registers::D => self.a & self.d,
            Registers::E => self.a & self.e,
            Registers::H => self.a & self.h,
            Registers::L => self.a & self.l,
            Registers::M => self.a & self.memory[((self.h as u16) << 8 | (self.l as u16)) as usize],
            Registers::I => {
                self.pc += 1;
                self.a * self.memory[self.pc as usize]
            }
            Registers::A => self.a & self.a,
            _ => {
                err_flag = true;
                0
            }
        };

        self.flags(total as u16);
        self.cc.cy = false;

        if err_flag {
            Err("Bad flag passed to AND".to_string())
        } else {
            self.a = total;
            Ok(0)
        }
    }

    // TODO: ac flag needs sorting to work on subtractions...
    pub fn op_cmp(&mut self, reg: Registers) -> Result<u8, String> {
        let mut err_flag = false;

        let total = match reg {
            Registers::B => (self.a as u16).wrapping_sub(self.b as u16),
            Registers::C => (self.a as u16).wrapping_sub(self.c as u16),
            Registers::D => (self.a as u16).wrapping_sub(self.d as u16),
            Registers::E => (self.a as u16).wrapping_sub(self.e as u16),
            Registers::H => (self.a as u16).wrapping_sub(self.h as u16),
            Registers::L => (self.a as u16).wrapping_sub(self.l as u16),
            Registers::M => {
                let val = self.memory[((self.h as u16) << 8 | (self.l as u16)) as usize] as u16;
                (self.a as u16).wrapping_sub(val)
            }
            Registers::A => (self.a as u16).wrapping_sub(self.a as u16),
            Registers::I => (self.a as u16).wrapping_sub(self.b as u16),
            _ => {
                err_flag = true;
                0
            }
        };

        self.flags(total);

        if err_flag {
            Err("Bad flag passed to CMP".to_string())
        } else {
            Ok(0)
        }
    }
}
