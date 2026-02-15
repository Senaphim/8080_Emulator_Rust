use std::env;
use std::fs;

fn main() {
    let argv: Vec<String> = env::args().collect();

    let opt = &argv[1];
    let filepath = &argv[argv.len() - 1];

    let mut dest: &String = &"".to_string();
    let mut to_file = false;

    if opt == "-f" {
        to_file = true;
        dest = &argv[2];
    }

    let raw_bytes = fs::read(filepath).expect("Should have been able to read the file");
    let out = disassemble(raw_bytes);

    if to_file {
        fs::write(dest, out).expect("Should have been able to write to file");
    } else {
        println!("{out}");
    }
}

fn disassemble(raw_bytes: Vec<u8>) -> String {
    let mut bytes = raw_bytes.into_iter().enumerate();
    let mut end = false;
    let mut ops = "".to_string();

    while !end {
        match bytes.next() {
            Some((idx, op)) => match op {
                0x00 => ops += format!("{idx:04x}  0x00  NOP\n").as_str(),
                0x01 => {
                    let c = bytes.next().unwrap().1;
                    let b = bytes.next().unwrap().1;
                    ops += format!("0x01  LXI    B,{b:02x}{c:02x}\n").as_str()
                }
                0x02 => ops += format!("{idx:04x}  0x02  STAX   B\n").as_str(),
                0x03 => ops += format!("{idx:04x}  0x03  INX    B\n").as_str(),
                0x04 => ops += format!("{idx:04x}  0x04  INR    B\n").as_str(),
                0x05 => ops += format!("{idx:04x}  0x05  DCR    B\n").as_str(),
                0x06 => {
                    let b = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x06  MVI    B,{b:02x}\n").as_str()
                }
                0x07 => ops += format!("{idx:04x}  0x07  RLC\n").as_str(),
                0x08 => ops += format!("{idx:04x}  0x08  NOP\n").as_str(),
                0x09 => ops += format!("{idx:04x}  0x09  DAD    B\n").as_str(),
                0x0a => ops += format!("{idx:04x}  0x0a  LDAX   B\n").as_str(),
                0x0b => ops += format!("{idx:04x}  0x0b  DCX    B\n").as_str(),
                0x0c => ops += format!("{idx:04x}  0x0c  INR    C\n").as_str(),
                0x0d => ops += format!("{idx:04x}  0x0d  DCR    C\n").as_str(),
                0x0e => {
                    let c = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x0e  MVI    C,{c:02x}\n").as_str()
                }
                0x0f => ops += format!("{idx:04x}  0x0f  RRC\n").as_str(),
                0x10 => ops += format!("{idx:04x}  0x10  NOP\n").as_str(),
                0x11 => {
                    let e = bytes.next().unwrap().1;
                    let d = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x11  LXI    D,{d:02x}{e:02x}\n").as_str()
                }
                0x12 => ops += format!("{idx:04x}  0x12  STAX   D\n").as_str(),
                0x13 => ops += format!("{idx:04x}  0x13  INX    D\n").as_str(),
                0x14 => ops += format!("{idx:04x}  0x14  INR    D\n").as_str(),
                0x15 => ops += format!("{idx:04x}  0x15  DCR    D\n").as_str(),
                0x16 => {
                    let d = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x16  MVI    D,{d:02x}\n").as_str()
                }
                0x17 => ops += format!("{idx:04x}  0x17  RAL\n").as_str(),
                0x18 => ops += format!("{idx:04x}  0x18  NOP\n").as_str(),
                0x19 => ops += format!("{idx:04x}  0x19  DAD    D\n").as_str(),
                0x1a => ops += format!("{idx:04x}  0x1a  LDAX   D\n").as_str(),
                0x1b => ops += format!("{idx:04x}  0x1b  DCX    D\n").as_str(),
                0x1c => ops += format!("{idx:04x}  0x1c  INR    E\n").as_str(),
                0x1d => ops += format!("{idx:04x}  0x1d  DCR    E\n").as_str(),
                0x1e => {
                    let e = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x1e  MVI    E,{e:02x}\n").as_str()
                }
                0x1f => ops += format!("{idx:04x}  0x1f  RAR\n").as_str(),
                0x20 => ops += format!("{idx:04x}  0x20  NOP\n").as_str(),
                0x21 => {
                    let l = bytes.next().unwrap().1;
                    let h = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x21  LXI    H,{h:02x}{l:02x}\n").as_str()
                }
                0x22 => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x22  SHLD   {adr1:2x}{adr2:2x}\n").as_str()
                }
                0x23 => ops += format!("{idx:04x}  0x23  INX    H\n").as_str(),
                0x24 => ops += format!("{idx:04x}  0x24  INR    H\n").as_str(),
                0x25 => ops += format!("{idx:04x}  0x25  DCT    H\n").as_str(),
                0x26 => {
                    let h = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x26  MVI    H,{h:02x}\n").as_str()
                }
                0x27 => ops += format!("{idx:04x}  0x27  DAA\n").as_str(),
                0x28 => ops += format!("{idx:04x}  0x28  NOP\n").as_str(),
                0x29 => ops += format!("{idx:04x}  0x29  DAD    H\n").as_str(),
                0x2a => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x2a  LHLD   {adr1:02x}{adr2:02x}\n").as_str()
                }
                0x2b => ops += format!("{idx:04x}  0x2b  DCX    H\n").as_str(),
                0x2c => ops += format!("{idx:04x}  0x2c  INR    L\n").as_str(),
                0x2d => ops += format!("{idx:04x}  0x2d  DCR    L\n").as_str(),
                0x2e => {
                    let l = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x2e  MVI    {l:02x}\n").as_str()
                }
                0x2f => ops += format!("{idx:04x}  0x2f  CMA\n").as_str(),
                0x30 => ops += format!("{idx:04x}  0x30  NOP\n").as_str(),
                0x31 => {
                    let splo = bytes.next().unwrap().1;
                    let sphi = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x31  LXI    SP{sphi:02x}{splo:02x}\n").as_str()
                }
                0x32 => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x32  STA    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0x33 => ops += format!("{idx:04x}  0x33  INX    SP\n").as_str(),
                0x34 => ops += format!("{idx:04x}  0x34  INR    M\n").as_str(),
                0x35 => ops += format!("{idx:04x}  0x35  DCR    M\n").as_str(),
                0x36 => {
                    let m = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x36  MVI    M,{m:02x}\n").as_str()
                }
                0x37 => ops += format!("{idx:04x}  0x37  STC\n").as_str(),
                0x38 => ops += format!("{idx:04x}  0x38  NOP\n").as_str(),
                0x39 => ops += format!("{idx:04x}  0x39  DAD    SP\n").as_str(),
                0x3a => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x3a  LDA    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0x3b => ops += format!("{idx:04x}  0x3b  DCX    SP\n").as_str(),
                0x3c => ops += format!("{idx:04x}  0x3c  INR    A\n").as_str(),
                0x3d => ops += format!("{idx:04x}  0x3d  DCR    A\n").as_str(),
                0x3e => {
                    let a = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0x3e  MVI    A,{a:2x}\n").as_str()
                }
                0x3f => ops += format!("{idx:04x}  0x3f  CMC\n").as_str(),
                0x40 => ops += format!("{idx:04x}  0x40  MOV    B,B\n").as_str(),
                0x41 => ops += format!("{idx:04x}  0x41  MOV    B,C\n").as_str(),
                0x42 => ops += format!("{idx:04x}  0x42  MOV    B,D\n").as_str(),
                0x43 => ops += format!("{idx:04x}  0x43  MOV    B,E\n").as_str(),
                0x44 => ops += format!("{idx:04x}  0x44  MOV    B,H\n").as_str(),
                0x45 => ops += format!("{idx:04x}  0x45  MOV    B,L\n").as_str(),
                0x46 => ops += format!("{idx:04x}  0x46  MOV    B,M\n").as_str(),
                0x47 => ops += format!("{idx:04x}  0x47  MOV    B,A\n").as_str(),
                0x48 => ops += format!("{idx:04x}  0x48  MOV    C,B\n").as_str(),
                0x49 => ops += format!("{idx:04x}  0x49  MOV    C,C\n").as_str(),
                0x4a => ops += format!("{idx:04x}  0x4a  MOV    C,D\n").as_str(),
                0x4b => ops += format!("{idx:04x}  0x4b  MOV    C,E\n").as_str(),
                0x4c => ops += format!("{idx:04x}  0x4c  MOV    C,H\n").as_str(),
                0x4d => ops += format!("{idx:04x}  0x4d  MOV    C,L\n").as_str(),
                0x4e => ops += format!("{idx:04x}  0x4e  MOV    C,M\n").as_str(),
                0x4f => ops += format!("{idx:04x}  0x4f  MOV    C,A\n").as_str(),
                0x50 => ops += format!("{idx:04x}  0x50  MOV    D,B\n").as_str(),
                0x51 => ops += format!("{idx:04x}  0x51  MOV    D,C\n").as_str(),
                0x52 => ops += format!("{idx:04x}  0x52  MOV    D,D\n").as_str(),
                0x53 => ops += format!("{idx:04x}  0x53  MOV    D,E\n").as_str(),
                0x54 => ops += format!("{idx:04x}  0x54  MOV    D,H\n").as_str(),
                0x55 => ops += format!("{idx:04x}  0x55  MOV    D,L\n").as_str(),
                0x56 => ops += format!("{idx:04x}  0x56  MOV    D,M\n").as_str(),
                0x57 => ops += format!("{idx:04x}  0x57  MOV    D,A\n").as_str(),
                0x58 => ops += format!("{idx:04x}  0x58  MOV    E,B\n").as_str(),
                0x59 => ops += format!("{idx:04x}  0x59  MOV    E,C\n").as_str(),
                0x5a => ops += format!("{idx:04x}  0x5a  MOV    E,D\n").as_str(),
                0x5b => ops += format!("{idx:04x}  0x5b  MOV    E,E\n").as_str(),
                0x5c => ops += format!("{idx:04x}  0x5c  MOV    E,H\n").as_str(),
                0x5d => ops += format!("{idx:04x}  0x5h  MOV    E,L\n").as_str(),
                0x5e => ops += format!("{idx:04x}  0x5e  MOV    E,M\n").as_str(),
                0x5f => ops += format!("{idx:04x}  0x5f  MOV    E,A\n").as_str(),
                0x60 => ops += format!("{idx:04x}  0x60  MOV    H,B\n").as_str(),
                0x61 => ops += format!("{idx:04x}  0x61  MOV    H,C\n").as_str(),
                0x62 => ops += format!("{idx:04x}  0x62  MOV    H,D\n").as_str(),
                0x63 => ops += format!("{idx:04x}  0x63  MOV    H,E\n").as_str(),
                0x64 => ops += format!("{idx:04x}  0x64  MOV    H,H\n").as_str(),
                0x65 => ops += format!("{idx:04x}  0x65  MOV    H,L\n").as_str(),
                0x66 => ops += format!("{idx:04x}  0x66  MOV    H,M\n").as_str(),
                0x67 => ops += format!("{idx:04x}  0x67  MOV    H,A\n").as_str(),
                0x68 => ops += format!("{idx:04x}  0x68  MOV    L,B\n").as_str(),
                0x69 => ops += format!("{idx:04x}  0x69  MOV    L,C\n").as_str(),
                0x6a => ops += format!("{idx:04x}  0x6a  MOV    L,D\n").as_str(),
                0x6b => ops += format!("{idx:04x}  0x6b  MOV    L,E\n").as_str(),
                0x6c => ops += format!("{idx:04x}  0x6c  MOV    L,H\n").as_str(),
                0x6d => ops += format!("{idx:04x}  0x6d  MOV    L,L\n").as_str(),
                0x6e => ops += format!("{idx:04x}  0x6e  MOV    L,M\n").as_str(),
                0x6f => ops += format!("{idx:04x}  0x6f  MOV    L,A\n").as_str(),
                0x70 => ops += format!("{idx:04x}  0x70  MOV    M,B\n").as_str(),
                0x71 => ops += format!("{idx:04x}  0x71  MOV    M,C\n").as_str(),
                0x72 => ops += format!("{idx:04x}  0x72  MOV    M,D\n").as_str(),
                0x73 => ops += format!("{idx:04x}  0x73  MOV    M,E\n").as_str(),
                0x74 => ops += format!("{idx:04x}  0x74  MOV    M,H\n").as_str(),
                0x75 => ops += format!("{idx:04x}  0x75  MOV    M,L\n").as_str(),
                0x76 => ops += format!("{idx:04x}  0x76  HLT\n").as_str(),
                0x77 => ops += format!("{idx:04x}  0x77  MOV    M,A\n").as_str(),
                0x78 => ops += format!("{idx:04x}  0x78  MOV    A,B\n").as_str(),
                0x79 => ops += format!("{idx:04x}  0x79  MOV    A,C\n").as_str(),
                0x7a => ops += format!("{idx:04x}  0x7a  MOV    A,D\n").as_str(),
                0x7b => ops += format!("{idx:04x}  0x7b  MOV    A,E\n").as_str(),
                0x7c => ops += format!("{idx:04x}  0x7c  MOV    A,H\n").as_str(),
                0x7d => ops += format!("{idx:04x}  0x7d  MOV    A,L\n").as_str(),
                0x7e => ops += format!("{idx:04x}  0x7e  MOV    A,M\n").as_str(),
                0x7f => ops += format!("{idx:04x}  0x7f  MOV    A,A\n").as_str(),
                0x80 => ops += format!("{idx:04x}  0x80  ADD    B\n").as_str(),
                0x81 => ops += format!("{idx:04x}  0x81  ADD    C\n").as_str(),
                0x82 => ops += format!("{idx:04x}  0x82  ADD    D\n").as_str(),
                0x83 => ops += format!("{idx:04x}  0x83  ADD    E\n").as_str(),
                0x84 => ops += format!("{idx:04x}  0x84  ADD    H\n").as_str(),
                0x85 => ops += format!("{idx:04x}  0x85  ADD    L\n").as_str(),
                0x86 => ops += format!("{idx:04x}  0x86  ADD    M\n").as_str(),
                0x87 => ops += format!("{idx:04x}  0x87  ADD    A\n").as_str(),
                0x88 => ops += format!("{idx:04x}  0x88  ADC    B\n").as_str(),
                0x89 => ops += format!("{idx:04x}  0x89  ADC    C\n").as_str(),
                0x8a => ops += format!("{idx:04x}  0x8a  ADC    D\n").as_str(),
                0x8b => ops += format!("{idx:04x}  0x8b  ADC    E\n").as_str(),
                0x8c => ops += format!("{idx:04x}  0x8c  ADC    H\n").as_str(),
                0x8d => ops += format!("{idx:04x}  0x8d  ADC    L\n").as_str(),
                0x8e => ops += format!("{idx:04x}  0x8e  ADC    M\n").as_str(),
                0x8f => ops += format!("{idx:04x}  0x8f  ADC    A\n").as_str(),
                0x90 => ops += format!("{idx:04x}  0x90  SUB    B\n").as_str(),
                0x91 => ops += format!("{idx:04x}  0x91  SUB    C\n").as_str(),
                0x92 => ops += format!("{idx:04x}  0x92  SUB    D\n").as_str(),
                0x93 => ops += format!("{idx:04x}  0x93  SUB    E\n").as_str(),
                0x94 => ops += format!("{idx:04x}  0x94  SUB    H\n").as_str(),
                0x95 => ops += format!("{idx:04x}  0x95  SUB    L\n").as_str(),
                0x96 => ops += format!("{idx:04x}  0x96  SUB    M\n").as_str(),
                0x97 => ops += format!("{idx:04x}  0x97  SUB    A\n").as_str(),
                0x98 => ops += format!("{idx:04x}  0x98  SBB    B\n").as_str(),
                0x99 => ops += format!("{idx:04x}  0x99  SBB    C\n").as_str(),
                0x9a => ops += format!("{idx:04x}  0x9a  SBB    D\n").as_str(),
                0x9b => ops += format!("{idx:04x}  0x9b  SBB    E\n").as_str(),
                0x9c => ops += format!("{idx:04x}  0x9c  SBB    H\n").as_str(),
                0x9d => ops += format!("{idx:04x}  0x9d  SBB    L\n").as_str(),
                0x9e => ops += format!("{idx:04x}  0x9e  SBB    M\n").as_str(),
                0x9f => ops += format!("{idx:04x}  0x9f  SBB    A\n").as_str(),
                0xa0 => ops += format!("{idx:04x}  0xa0  ANA    B\n").as_str(),
                0xa1 => ops += format!("{idx:04x}  0xa1  ANA    C\n").as_str(),
                0xa2 => ops += format!("{idx:04x}  0xa2  ANA    D\n").as_str(),
                0xa3 => ops += format!("{idx:04x}  0xa3  ANA    E\n").as_str(),
                0xa4 => ops += format!("{idx:04x}  0xa4  ANA    H\n").as_str(),
                0xa5 => ops += format!("{idx:04x}  0xa5  ANA    L\n").as_str(),
                0xa6 => ops += format!("{idx:04x}  0xa6  ANA    M\n").as_str(),
                0xa7 => ops += format!("{idx:04x}  0xa7  ANA    A\n").as_str(),
                0xa8 => ops += format!("{idx:04x}  0xa8  XRA    B\n").as_str(),
                0xa9 => ops += format!("{idx:04x}  0xa9  XRA    C\n").as_str(),
                0xaa => ops += format!("{idx:04x}  0xaa  XRA    D\n").as_str(),
                0xab => ops += format!("{idx:04x}  0xab  XRA    E\n").as_str(),
                0xac => ops += format!("{idx:04x}  0xac  XRA    H\n").as_str(),
                0xad => ops += format!("{idx:04x}  0xad  XRA    L\n").as_str(),
                0xae => ops += format!("{idx:04x}  0xae  XRA    M\n").as_str(),
                0xaf => ops += format!("{idx:04x}  0xaf  XRA    A\n").as_str(),
                0xb0 => ops += format!("{idx:04x}  0xb0  ORA    B\n").as_str(),
                0xb1 => ops += format!("{idx:04x}  0xb1  ORA    C\n").as_str(),
                0xb2 => ops += format!("{idx:04x}  0xb2  ORA    D\n").as_str(),
                0xb3 => ops += format!("{idx:04x}  0xb3  ORA    E\n").as_str(),
                0xb4 => ops += format!("{idx:04x}  0xb4  ORA    H\n").as_str(),
                0xb5 => ops += format!("{idx:04x}  0xb5  ORA    L\n").as_str(),
                0xb6 => ops += format!("{idx:04x}  0xb6  ORA    M\n").as_str(),
                0xb7 => ops += format!("{idx:04x}  0xb7  ORA    A\n").as_str(),
                0xb8 => ops += format!("{idx:04x}  0xb8  CMP    B\n").as_str(),
                0xb9 => ops += format!("{idx:04x}  0xb9  CMP    C\n").as_str(),
                0xba => ops += format!("{idx:04x}  0xba  CMP    D\n").as_str(),
                0xbb => ops += format!("{idx:04x}  0xbb  CMP    E\n").as_str(),
                0xbc => ops += format!("{idx:04x}  0xbc  CMP    H\n").as_str(),
                0xbd => ops += format!("{idx:04x}  0xbd  CMP    L\n").as_str(),
                0xbe => ops += format!("{idx:04x}  0xbe  CMP    M\n").as_str(),
                0xbf => ops += format!("{idx:04x}  0xbf  CMP    A\n").as_str(),
                0xc0 => ops += format!("{idx:04x}  0xc0  RNZ\n").as_str(),
                0xc1 => ops += format!("{idx:04x}  0xc1  POP    B\n").as_str(),
                0xc2 => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xc2  JNZ    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xc3 => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xc3  JMP    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xc4 => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xc4  CNZ    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xc5 => ops += format!("{idx:04x}  0xc5  PUSH   B\n").as_str(),
                0xc6 => {
                    let byt = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xc6  ADI    {byt:02x}\n").as_str()
                }
                0xc7 => ops += format!("{idx:04x}  0xc7  RST    0\n").as_str(),
                0xc8 => ops += format!("{idx:04x}  0xc8  RZ\n").as_str(),
                0xc9 => ops += format!("{idx:04x}  0xc9  RET\n").as_str(),
                0xca => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xca  JZ     {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xcb => ops += format!("{idx:04x}  0xcb  NOP\n").as_str(),
                0xcc => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xcc  CZ     {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xcd => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xcd  CALL   {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xce => {
                    let byt = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xce  ACI    {byt:02x}\n").as_str()
                }
                0xcf => ops += format!("{idx:04x}  0xcf  RST    1\n").as_str(),
                0xd0 => ops += format!("{idx:04x}  0xd0  RNC\n").as_str(),
                0xd1 => ops += format!("{idx:04x}  0xd1  POP    D\n").as_str(),
                0xd2 => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xd2  JNC    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xd3 => {
                    let byt = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xd3  OUT    {byt:02x}\n").as_str()
                }
                0xd4 => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xd4  CNC    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xd5 => ops += format!("{idx:04x}  0xd5  PUSH   D\n").as_str(),
                0xd6 => {
                    let byt = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xd6  SUI    {byt:02x}\n").as_str()
                }
                0xd7 => ops += format!("{idx:04x}  0xd7  RST    2\n").as_str(),
                0xd8 => ops += format!("{idx:04x}  0xd8  RC\n").as_str(),
                0xd9 => ops += format!("{idx:04x}  0xd9  NOP\n").as_str(),
                0xda => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xda  JA     {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xdb => {
                    let byt = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xdb  IN     {byt:02x}\n").as_str()
                }
                0xdc => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xdc  CC     {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xdd => ops += format!("{idx:04x}  0xdd  NOP\n").as_str(),
                0xde => {
                    let byt = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xde  SBI    {byt:02x}\n").as_str()
                }
                0xdf => ops += format!("{idx:04x}  0xdf  RST    3\n").as_str(),
                0xe0 => ops += format!("{idx:04x}  0xe0  RPO\n").as_str(),
                0xe1 => ops += format!("{idx:04x}  0xe1  POP    H\n").as_str(),
                0xe2 => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xe2  JPO    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xe3 => ops += format!("{idx:04x}  0xe3  XTHL\n").as_str(),
                0xe4 => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xe4  CPO    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xe5 => ops += format!("{idx:04x}  0xe5  PUSH   H\n").as_str(),
                0xe6 => {
                    let byt = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xe6  ANI    {byt:02x}\n").as_str()
                }
                0xe7 => ops += format!("{idx:04x}  0xe7  RST    4\n").as_str(),
                0xe8 => ops += format!("{idx:04x}  0xe8  RPE\n").as_str(),
                0xe9 => ops += format!("{idx:04x}  0xe9  PCHL\n").as_str(),
                0xea => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xea  JPE    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xeb => ops += format!("{idx:04x}  0xeb  XCHG\n").as_str(),
                0xec => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xec  CPE    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xed => ops += format!("{idx:04x}  0xed  NOP\n").as_str(),
                0xee => {
                    let byt = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xee  XRI    {byt:02x}\n").as_str()
                }
                0xef => ops += format!("{idx:04x}  0xef  RST    5\n").as_str(),
                0xf0 => ops += format!("{idx:04x}  0xf0  RP\n").as_str(),
                0xf1 => ops += format!("{idx:04x}  0xf1  POP PSW\n").as_str(),
                0xf2 => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xf2  JP     {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xf3 => ops += format!("{idx:04x}  0xf3  DI\n").as_str(),
                0xf4 => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xf4  CP     {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xf5 => ops += format!("{idx:04x}  0xf5  PUSH PSW\n").as_str(),
                0xf6 => {
                    let byt = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xf6  ORI    {byt:02x}\n").as_str()
                }
                0xf7 => ops += format!("{idx:04x}  0xf7  RST    6\n").as_str(),
                0xf8 => ops += format!("{idx:04x}  0xf8  RM\n").as_str(),
                0xf9 => ops += format!("{idx:04x}  0xf9  SPHL\n").as_str(),
                0xfa => {
                    let byt = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xfa  JM     {byt:02x}\n").as_str()
                }
                0xfb => ops += format!("{idx:04x}  0xfb  EI\n").as_str(),
                0xfc => {
                    let adr2 = bytes.next().unwrap().1;
                    let adr1 = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xfc  CM     {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xfd => ops += format!("{idx:04x}  0xfd  NOP\n").as_str(),
                0xfe => {
                    let byt = bytes.next().unwrap().1;
                    ops += format!("{idx:04x}  0xfe  CPI    {byt:02x}\n").as_str()
                }
                0xff => ops += format!("{idx:04x}  0xff  RST    7\n").as_str(),
            },
            None => end = true,
        };
    }

    ops
}
