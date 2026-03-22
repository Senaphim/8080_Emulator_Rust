use crate::state::{Flags, State8080};

impl State8080 {
    // PROGRAM CONTROL
    // --------------------------------------------------------------------------------------------
    pub fn op_jmp(&mut self, flg: Flags) -> Result<u8, String> {
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

    pub fn op_call(&mut self, flg: Flags) -> Result<u8, String> {
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

    pub fn call(&mut self, pc_u: u8, pc_l: u8, ptr: u16) {
        self.memory[(self.sp as usize) - 1] = pc_u;
        self.memory[(self.sp as usize) - 2] = pc_l;
        self.sp -= 2;
        self.pc = ptr;
    }

    pub fn op_ret(&mut self, flg: Flags) -> Result<u8, String> {
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

    pub fn ret(&mut self, pc_u: u8, pc_l: u8) {
        // Not sure if subtract 1 as we will be adding 1 as part of calling function
        self.pc = (pc_u as u16) << 8 | pc_l as u16; //.wrapping_sub(1);
        self.sp += 2
    }
}
