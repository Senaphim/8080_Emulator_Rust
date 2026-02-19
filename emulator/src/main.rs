struct ConditionCodes {
    z: u8,
    s: u8,
    p: u8,
    cy: u8,
    ac: u8,
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

impl State8080 {
    fn emulate_8080(&mut self) -> Result<u8, String> {
        let opcode = self.memory[self.pc as usize];
        let mut status: Result<u8, String> = Ok(0);

        match opcode {
            0x00 => status = Ok(0),
            0x01 => status = Ok(0),
            _ => status = Err("Unimplemented Opcode".to_string()),
        };

        self.pc += 1;
        status
    }
}

fn main() {
    println!("Hello, world!");
}
