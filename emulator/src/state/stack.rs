use crate::state::{Registers, State8080};

impl State8080 {
    pub fn op_push(&mut self, reg: Registers) -> Result<u8, String> {
        let mut err_flag = false;

        match reg {
            Registers::B => self.push(self.b, self.c),
            Registers::D => self.push(self.d, self.e),
            Registers::H => self.push(self.h, self.l),
            Registers::A => self.psw(),
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

    fn psw(&mut self) {
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
}
