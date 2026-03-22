use crate::state::{Registers, State8080};

impl State8080 {
    // ARITHMETIC
    // --------------------------------------------------------------------------------------------
    // Missing ac (Auxillary Carry) flag, but this is only used for one operation (DAA) so will be
    // manually set there
    pub fn flags(&mut self, val: u16) {
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
    pub fn ac_flag(&mut self, val: u16) {
        let accumulator = (self.a as u16) & 0xf;
        let adder = val & 0xf;
        let total = accumulator + adder;

        self.cc.ac = total > 0xf;
    }

    pub fn op_add(&mut self, reg: Registers) -> Result<u8, String> {
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

    pub fn op_adc(&mut self, reg: Registers) -> Result<u8, String> {
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

    pub fn op_adi(&mut self) -> Result<u8, String> {
        self.pc += 1;

        let adder = self.memory[self.pc as usize] as u16;
        let total: u16 = (self.a as u16) + adder;

        // Pass to flags to check output
        self.ac_flag(adder);
        self.flags(total);
        self.a = total as u8;

        Ok(0)
    }

    pub fn op_aci(&mut self) -> Result<u8, String> {
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

    pub fn op_inx(&mut self, reg: Registers) -> Result<u8, String> {
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

        val = val.wrapping_add(1);
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
    pub fn op_inr(&mut self, reg: Registers) -> Result<u8, String> {
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
    pub fn op_dcr(&mut self, reg: Registers) -> Result<u8, String> {
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
