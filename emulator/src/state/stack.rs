use crate::state::{Registers, State8080};

impl State8080 {
    // STACK OPERATIONS
    // --------------------------------------------------------------------------------------------
    pub fn op_push(&mut self, reg: Registers) -> Result<u8, String> {
        let mut err_flag = false;

        match reg {
            Registers::B => self.push(self.b, self.c),
            Registers::D => self.push(self.d, self.e),
            Registers::H => self.push(self.h, self.l),
            Registers::A => self.push_psw(),
            _ => err_flag = true,
        }

        if err_flag {
            Err("Bad register passed to PUSH".to_string())
        } else {
            Ok(0)
        }
    }

    fn push(&mut self, hi: u8, lo: u8) {
        self.memory[(self.sp - 1) as usize] = hi;
        self.memory[(self.sp - 2) as usize] = lo;
        self.sp = self.sp.wrapping_sub(2);
    }

    fn push_psw(&mut self) {
        let mut acc: u8 = 0x02;

        if self.cc.s {
            acc += 0x80
        }
        if self.cc.z {
            acc += 0x40
        }
        if self.cc.ac {
            acc += 0x10
        }
        if self.cc.p {
            acc += 0x04
        }
        if self.cc.cy {
            acc += 0x01
        }

        self.memory[(self.sp - 1) as usize] = self.a;
        self.memory[(self.sp - 2) as usize] = acc;
        self.sp = self.sp.wrapping_sub(2);
    }

    // Not technically a stack operation, but there isn't really a better place to put it
    pub fn op_xchg(&mut self) -> Result<u8, String> {
        std::mem::swap(&mut self.h, &mut self.d);
        std::mem::swap(&mut self.l, &mut self.e);

        Ok(0)
    }

    pub fn op_pop(&mut self, reg: Registers) -> Result<u8, String> {
        let mut err_flag = false;

        let lo = self.memory[self.sp as usize];
        let hi = self.memory[(self.sp + 1) as usize];

        match reg {
            Registers::B => {
                self.b = hi;
                self.c = lo;
            }
            Registers::D => {
                self.d = hi;
                self.e = lo;
            }
            Registers::H => {
                self.h = hi;
                self.l = lo;
            }
            Registers::A => {
                self.a = hi;
                self.pop_psw(lo);
            }
            _ => err_flag = true,
        }

        self.sp = self.sp.wrapping_add(2);

        if err_flag {
            Err("Bad register passed to POP".to_string())
        } else {
            Ok(0)
        }
    }

    fn pop_psw(&mut self, flgs: u8) {
        self.cc.s = flgs & 0x80 > 0;
        self.cc.z = flgs & 0x40 > 0;
        self.cc.ac = flgs & 0x10 > 0;
        self.cc.p = flgs & 0x04 > 0;
        self.cc.cy = flgs & 0x01 > 0;
    }
}
