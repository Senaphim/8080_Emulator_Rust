pub mod arithmetic;
pub mod control;
pub mod logical;
pub mod memory;
pub mod stack;

pub enum Registers {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    M,
    Sp,
    I,
}

pub enum Flags {
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
pub struct ConditionCodes {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool,
    // pub pad: u8,
}

pub struct State8080 {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    // pub int_enable: u8,
    pub cc: ConditionCodes,
    pub memory: Vec<u8>,
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
            memory: vec![0x00; 0x10000],
        }
    }
}

impl State8080 {
    pub fn emulate_8080(&mut self) -> Result<u8, String> {
        let opcode = self.memory[self.pc as usize];

        let status = match opcode {
            0x00 => Ok(0),
            0x01 => self.op_lxi(Registers::B),

            0x03 => self.op_inx(Registers::B),
            0x04 => self.op_inr(Registers::B),
            0x05 => self.op_dcr(Registers::B),
            0x06 => self.op_mvi(Registers::B),

            0x08 => Ok(0),
            0x09 => self.op_dad(Registers::B),
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
            0x19 => self.op_dad(Registers::D),
            0x1a => self.op_ldax(Registers::D),

            0x1c => self.op_inr(Registers::E),
            0x1d => self.op_dcr(Registers::E),
            0x1e => self.op_mvi(Registers::E),

            0x20 => Ok(0),
            0x21 => self.op_lxi(Registers::H),

            0x23 => self.op_inx(Registers::H),
            0x24 => self.op_inr(Registers::H),
            0x25 => self.op_dcr(Registers::H),
            0x26 => self.op_mvi(Registers::H),

            0x28 => Ok(0),
            0x29 => self.op_dad(Registers::H),
            0x2c => self.op_inr(Registers::L),
            0x2d => self.op_dcr(Registers::L),
            0x2e => self.op_mvi(Registers::L),

            0x30 => Ok(0),
            0x31 => self.op_lxi(Registers::Sp),

            0x33 => self.op_inx(Registers::Sp),
            0x34 => self.op_inr(Registers::M),
            0x35 => self.op_dcr(Registers::M),
            0x36 => self.op_mvi(Registers::M),

            0x38 => Ok(0),
            0x39 => self.op_dad(Registers::Sp),

            0x3c => self.op_inr(Registers::A),
            0x3d => self.op_dcr(Registers::A),

            0x3e => self.op_mvi(Registers::A),

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

            0xa0 => self.op_and(Registers::B),
            0xa1 => self.op_and(Registers::C),
            0xa2 => self.op_and(Registers::D),
            0xa3 => self.op_and(Registers::E),
            0xa4 => self.op_and(Registers::H),
            0xa5 => self.op_and(Registers::L),
            0xa6 => self.op_and(Registers::M),
            0xa7 => self.op_and(Registers::A),

            0xb8 => self.op_cmp(Registers::B),
            0xb9 => self.op_cmp(Registers::C),
            0xba => self.op_cmp(Registers::D),
            0xbb => self.op_cmp(Registers::E),
            0xbc => self.op_cmp(Registers::H),
            0xbd => self.op_cmp(Registers::L),
            0xbe => self.op_cmp(Registers::M),
            0xbf => self.op_cmp(Registers::A),
            0xc0 => self.op_ret(Flags::Nz),

            0xc2 => self.op_jmp(Flags::Nz),
            0xc3 => self.op_jmp(Flags::None),
            0xc4 => self.op_call(Flags::Nz),
            0xc5 => self.op_push(Registers::B),
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
            0xd5 => self.op_push(Registers::D),

            0xd8 => self.op_ret(Flags::Cy),
            0xd9 => Ok(0),
            0xda => self.op_jmp(Flags::Cy),

            0xdc => self.op_call(Flags::Cy),
            0xdd => Ok(0),

            0xe0 => self.op_ret(Flags::Np),

            0xe2 => self.op_jmp(Flags::Np),

            0xe4 => self.op_call(Flags::Np),
            0xe5 => self.op_push(Registers::H),
            0xe6 => self.op_and(Registers::I),

            0xe8 => self.op_ret(Flags::P),

            0xea => self.op_jmp(Flags::P),
            0xeb => self.op_xchg(),

            0xed => Ok(0),
            0xec => self.op_call(Flags::P),

            0xf0 => self.op_ret(Flags::S),

            0xf2 => self.op_jmp(Flags::S),

            0xf4 => self.op_call(Flags::S),
            0xf5 => self.op_push(Registers::A),

            0xf8 => self.op_ret(Flags::Ns),

            0xfa => self.op_jmp(Flags::Ns),

            0xfc => self.op_call(Flags::Ns),
            0xfd => Ok(0),
            0xfe => self.op_cmp(Registers::I),

            _ => Err(format!("Unimplemented opcode: {opcode:04x}")),
        };

        // The design decistion to have the pc increment at the end of the routine rather than the
        // beginning is one that has resulted in some weirdness with the call and return methods. I
        // am still not sure if that decision is a good one, but it makes the initialisation of the
        // system and loading of values much cleaner
        self.pc = self.pc.wrapping_add(1);
        status
    }
}
