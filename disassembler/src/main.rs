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
    let mut bytes = raw_bytes.into_iter();
    let mut end = false;
    let mut ops = "".to_string();

    while !end {
        match bytes.next() {
            Some(op) => match op {
                0x00 => ops += "0x00  NOP\n",
                0x01 => {
                    let c = bytes.next().unwrap();
                    let b = bytes.next().unwrap();
                    ops += format!("0x01  LXI    B,{b:02x}{c:02x}\n").as_str()
                }
                0x02 => ops += "0x02  STAX   B\n",
                0x03 => ops += "0x03  INX    B\n",
                0x04 => ops += "0x04  INR    B\n",
                0x05 => ops += "0x05  DCR    B\n",
                0x06 => {
                    let b = bytes.next().unwrap();
                    ops += format!("0x06  MVI    B,{b:02x}\n").as_str()
                }
                0x07 => ops += "0x07  RLC\n",
                0x08 => ops += "0x08  NOP\n",
                0x09 => ops += "0x09  DAD    B\n",
                0x0a => ops += "0x0a  LDAX   B\n",
                0x0b => ops += "0x0b  DCX    B\n",
                0x0c => ops += "0x0c  INR    C\n",
                0x0d => ops += "0x0d  DCR    C\n",
                0x0e => {
                    let c = bytes.next().unwrap();
                    ops += format!("0x0e  MVI    C,{c:02x}\n").as_str()
                }
                0x0f => ops += "0x0f  RRC\n",
                0x10 => ops += "0x10  NOP\n",
                0x11 => {
                    let e = bytes.next().unwrap();
                    let d = bytes.next().unwrap();
                    ops += format!("0x11  LXI    D,{d:02x}{e:02x}\n").as_str()
                }
                0x12 => ops += "0x12  STAX   D\n",
                0x13 => ops += "0x13  INX    D\n",
                0x14 => ops += "0x14  INR    D\n",
                0x15 => ops += "0x15  DCR    D\n",
                0x16 => {
                    let d = bytes.next().unwrap();
                    ops += format!("0x16  MVI    D,{d:02x}\n").as_str()
                }
                0x17 => ops += "0x17  RAL\n",
                0x18 => ops += "0x18  NOP\n",
                0x19 => ops += "0x19  DAD    D\n",
                0x1a => ops += "0x1a  LDAX   D\n",
                0x1b => ops += "0x1b  DCX    D\n",
                0x1c => ops += "0x1c  INR    E\n",
                0x1d => ops += "0x1d  DCR    E\n",
                0x1e => {
                    let e = bytes.next().unwrap();
                    ops += format!("0x1e  MVI    E,{e:02x}\n").as_str()
                }
                0x1f => ops += "0x1f  RAR\n",
                0x20 => ops += "0x20  NOP\n",
                0x21 => {
                    let l = bytes.next().unwrap();
                    let h = bytes.next().unwrap();
                    ops += format!("0x21  LXI    H,{h:02x}{l:02x}\n").as_str()
                }
                0x22 => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0x22  SHLD   {adr1:2x}{adr2:2x}\n").as_str()
                }
                0x23 => ops += "0x23  INX    H\n",
                0x24 => ops += "0x24  INR    H\n",
                0x25 => ops += "0x25  DCT    H\n",
                0x26 => {
                    let h = bytes.next().unwrap();
                    ops += format!("0x26  MVI    H,{h:02x}\n").as_str()
                }
                0x27 => ops += "0x27  DAA\n",
                0x28 => ops += "0x28  NOP\n",
                0x29 => ops += "0x29  DAD    H\n",
                0x2a => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0x2a  LHLD   {adr1:02x}{adr2:02x}\n").as_str()
                }
                0x2b => ops += "0x2b  DCX    H\n",
                0x2c => ops += "0x2c  INR    L\n",
                0x2d => ops += "0x2d  DCR    L\n",
                0x2e => {
                    let l = bytes.next().unwrap();
                    ops += format!("0x2e  MVI    {l:02x}\n").as_str()
                }
                0x2f => ops += "0x2f  CMA\n",
                0x30 => ops += "0x30  NOP\n",
                0x31 => {
                    let splo = bytes.next().unwrap();
                    let sphi = bytes.next().unwrap();
                    ops += format!("0x31  LXI    SP{sphi:02x}{splo:02x}\n").as_str()
                }
                0x32 => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0x32  STA    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0x33 => ops += "0x33  INX    SP\n",
                0x34 => ops += "0x34  INR    M\n",
                0x35 => ops += "0x35  DCR    M\n",
                0x36 => {
                    let m = bytes.next().unwrap();
                    ops += format!("0x36  MVI    M,{m:02x}\n").as_str()
                }
                0x37 => ops += "0x37  STC\n",
                0x38 => ops += "0x38  NOP\n",
                0x39 => ops += "0x39  DAD    SP\n",
                0x3a => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0x3a  LDA    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0x3b => ops += "0x3b  DCX    SP\n",
                0x3c => ops += "0x3c  INR    A\n",
                0x3d => ops += "0x3d  DCR    A\n",
                0x3e => {
                    let a = bytes.next().unwrap();
                    ops += format!("0x3e  MVI    A,{a:2x}\n").as_str()
                }
                0x3f => ops += "0x3f  CMC\n",
                0x40 => ops += "0x40  MOV    B,B\n",
                0x41 => ops += "0x41  MOV    B,C\n",
                0x42 => ops += "0x42  MOV    B,D\n",
                0x43 => ops += "0x43  MOV    B,E\n",
                0x44 => ops += "0x44  MOV    B,H\n",
                0x45 => ops += "0x45  MOV    B,L\n",
                0x46 => ops += "0x46  MOV    B,M\n",
                0x47 => ops += "0x47  MOV    B,A\n",
                0x48 => ops += "0x48  MOV    C,B\n",
                0x49 => ops += "0x49  MOV    C,C\n",
                0x4a => ops += "0x4a  MOV    C,D\n",
                0x4b => ops += "0x4b  MOV    C,E\n",
                0x4c => ops += "0x4c  MOV    C,H\n",
                0x4d => ops += "0x4d  MOV    C,L\n",
                0x4e => ops += "0x4e  MOV    C,M\n",
                0x4f => ops += "0x4f  MOV    C,A\n",
                0x50 => ops += "0x50  MOV    D,B\n",
                0x51 => ops += "0x51  MOV    D,C\n",
                0x52 => ops += "0x52  MOV    D,D\n",
                0x53 => ops += "0x53  MOV    D,E\n",
                0x54 => ops += "0x54  MOV    D,H\n",
                0x55 => ops += "0x55  MOV    D,L\n",
                0x56 => ops += "0x56  MOV    D,M\n",
                0x57 => ops += "0x57  MOV    D,A\n",
                0x58 => ops += "0x58  MOV    E,B\n",
                0x59 => ops += "0x59  MOV    E,C\n",
                0x5a => ops += "0x5a  MOV    E,D\n",
                0x5b => ops += "0x5b  MOV    E,E\n",
                0x5c => ops += "0x5c  MOV    E,H\n",
                0x5d => ops += "0x5h  MOV    E,L\n",
                0x5e => ops += "0x5e  MOV    E,M\n",
                0x5f => ops += "0x5f  MOV    E,A\n",
                0x60 => ops += "0x60  MOV    H,B\n",
                0x61 => ops += "0x61  MOV    H,C\n",
                0x62 => ops += "0x62  MOV    H,D\n",
                0x63 => ops += "0x63  MOV    H,E\n",
                0x64 => ops += "0x64  MOV    H,H\n",
                0x65 => ops += "0x65  MOV    H,L\n",
                0x66 => ops += "0x66  MOV    H,M\n",
                0x67 => ops += "0x67  MOV    H,A\n",
                0x68 => ops += "0x68  MOV    L,B\n",
                0x69 => ops += "0x69  MOV    L,C\n",
                0x6a => ops += "0x6a  MOV    L,D\n",
                0x6b => ops += "0x6b  MOV    L,E\n",
                0x6c => ops += "0x6c  MOV    L,H\n",
                0x6d => ops += "0x6d  MOV    L,L\n",
                0x6e => ops += "0x6e  MOV    L,M\n",
                0x6f => ops += "0x6f  MOV    L,A\n",
                0x70 => ops += "0x70  MOV    M,B\n",
                0x71 => ops += "0x71  MOV    M,C\n",
                0x72 => ops += "0x72  MOV    M,D\n",
                0x73 => ops += "0x73  MOV    M,E\n",
                0x74 => ops += "0x74  MOV    M,H\n",
                0x75 => ops += "0x75  MOV    M,L\n",
                0x76 => ops += "0x76  HLT\n",
                0x77 => ops += "0x77  MOV    M,A\n",
                0x78 => ops += "0x78  MOV    A,B\n",
                0x79 => ops += "0x79  MOV    A,C\n",
                0x7a => ops += "0x7a  MOV    A,D\n",
                0x7b => ops += "0x7b  MOV    A,E\n",
                0x7c => ops += "0x7c  MOV    A,H\n",
                0x7d => ops += "0x7d  MOV    A,L\n",
                0x7e => ops += "0x7e  MOV    A,M\n",
                0x7f => ops += "0x7f  MOV    A,A\n",
                0x80 => ops += "0x80  ADD    B\n",
                0x81 => ops += "0x81  ADD    C\n",
                0x82 => ops += "0x82  ADD    D\n",
                0x83 => ops += "0x83  ADD    E\n",
                0x84 => ops += "0x84  ADD    H\n",
                0x85 => ops += "0x85  ADD    L\n",
                0x86 => ops += "0x86  ADD    M\n",
                0x87 => ops += "0x87  ADD    A\n",
                0x88 => ops += "0x88  ADC    B\n",
                0x89 => ops += "0x89  ADC    C\n",
                0x8a => ops += "0x8a  ADC    D\n",
                0x8b => ops += "0x8b  ADC    E\n",
                0x8c => ops += "0x8c  ADC    H\n",
                0x8d => ops += "0x8d  ADC    L\n",
                0x8e => ops += "0x8e  ADC    M\n",
                0x8f => ops += "0x8f  ADC    A\n",
                0x90 => ops += "0x90  SUB    B\n",
                0x91 => ops += "0x91  SUB    C\n",
                0x92 => ops += "0x92  SUB    D\n",
                0x93 => ops += "0x93  SUB    E\n",
                0x94 => ops += "0x94  SUB    H\n",
                0x95 => ops += "0x95  SUB    L\n",
                0x96 => ops += "0x96  SUB    M\n",
                0x97 => ops += "0x97  SUB    A\n",
                0x98 => ops += "0x98  SBB    B\n",
                0x99 => ops += "0x99  SBB    C\n",
                0x9a => ops += "0x9a  SBB    D\n",
                0x9b => ops += "0x9b  SBB    E\n",
                0x9c => ops += "0x9c  SBB    H\n",
                0x9d => ops += "0x9d  SBB    L\n",
                0x9e => ops += "0x9e  SBB    M\n",
                0x9f => ops += "0x9f  SBB    A\n",
                0xa0 => ops += "0xa0  ANA    B\n",
                0xa1 => ops += "0xa1  ANA    C\n",
                0xa2 => ops += "0xa2  ANA    D\n",
                0xa3 => ops += "0xa3  ANA    E\n",
                0xa4 => ops += "0xa4  ANA    H\n",
                0xa5 => ops += "0xa5  ANA    L\n",
                0xa6 => ops += "0xa6  ANA    M\n",
                0xa7 => ops += "0xa7  ANA    A\n",
                0xa8 => ops += "0xa8  XRA    B\n",
                0xa9 => ops += "0xa9  XRA    C\n",
                0xaa => ops += "0xaa  XRA    D\n",
                0xab => ops += "0xab  XRA    E\n",
                0xac => ops += "0xac  XRA    H\n",
                0xad => ops += "0xad  XRA    L\n",
                0xae => ops += "0xae  XRA    M\n",
                0xaf => ops += "0xaf  XRA    A\n",
                0xb0 => ops += "0xb0  ORA    B\n",
                0xb1 => ops += "0xb1  ORA    C\n",
                0xb2 => ops += "0xb2  ORA    D\n",
                0xb3 => ops += "0xb3  ORA    E\n",
                0xb4 => ops += "0xb4  ORA    H\n",
                0xb5 => ops += "0xb5  ORA    L\n",
                0xb6 => ops += "0xb6  ORA    M\n",
                0xb7 => ops += "0xb7  ORA    A\n",
                0xb8 => ops += "0xb8  CMP    B\n",
                0xb9 => ops += "0xb9  CMP    C\n",
                0xba => ops += "0xba  CMP    D\n",
                0xbb => ops += "0xbb  CMP    E\n",
                0xbc => ops += "0xbc  CMP    H\n",
                0xbd => ops += "0xbd  CMP    L\n",
                0xbe => ops += "0xbe  CMP    M\n",
                0xbf => ops += "0xbf  CMP    A\n",
                0xc0 => ops += "0xc0  RNZ\n",
                0xc1 => ops += "0xc1  POP    B\n",
                0xc2 => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xc2  JNZ    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xc3 => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xc3  JMP    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xc4 => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xc4  CNZ    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xc5 => ops += "0xc5  PUSH   B\n",
                0xc6 => {
                    let byt = bytes.next().unwrap();
                    ops += format!("0xc6  ADI    {byt:02x}\n").as_str()
                }
                0xc7 => ops += "0xc7  RST    0\n",
                0xc8 => ops += "0xc8  RZ\n",
                0xc9 => ops += "0xc9  RET\n",
                0xca => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xca  JZ     {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xcb => ops += "0xcb  NOP\n",
                0xcc => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xcc  CZ     {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xcd => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xcd  CALL   {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xce => {
                    let byt = bytes.next().unwrap();
                    ops += format!("0xce  ACI    {byt:02x}\n").as_str()
                }
                0xcf => ops += "0xcf  RST    1\n",
                0xd0 => ops += "0xd0  RNC\n",
                0xd1 => ops += "0xd1  POP    D\n",
                0xd2 => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xd2  JNC    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xd3 => {
                    let byt = bytes.next().unwrap();
                    ops += format!("0xd3  OUT    {byt:02x}\n").as_str()
                }
                0xd4 => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xd4  CNC    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xd5 => ops += "0xd5  PUSH   D\n",
                0xd6 => {
                    let byt = bytes.next().unwrap();
                    ops += format!("0xd6  SUI    {byt:02x}\n").as_str()
                }
                0xd7 => ops += "0xd7  RST    2\n",
                0xd8 => ops += "0xd8  RC\n",
                0xd9 => ops += "0xd9  NOP\n",
                0xda => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xda  JA     {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xdb => {
                    let byt = bytes.next().unwrap();
                    ops += format!("0xdb  IN     {byt:02x}\n").as_str()
                }
                0xdc => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xdc  CC     {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xdd => ops += "0xdd  NOP\n",
                0xde => {
                    let byt = bytes.next().unwrap();
                    ops += format!("0xde  SBI    {byt:02x}\n").as_str()
                }
                0xdf => ops += "0xdf  RST    3\n",
                0xe0 => ops += "0xe0  RPO\n",
                0xe1 => ops += "0xe1  POP    H\n",
                0xe2 => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xe2  JPO    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xe3 => ops += "0xe3  XTHL\n",
                0xe4 => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xe4  CPO    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xe5 => ops += "0xe5  PUSH   H\n",
                0xe6 => {
                    let byt = bytes.next().unwrap();
                    ops += format!("0xe6  ANI    {byt:02x}\n").as_str()
                }
                0xe7 => ops += "0xe7  RST    4\n",
                0xe8 => ops += "0xe8  RPE\n",
                0xe9 => ops += "0xe9  PCHL\n",
                0xea => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xea  JPE    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xeb => ops += "0xeb  XCHG\n",
                0xec => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xec  CPE    {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xed => ops += "0xed  NOP\n",
                0xee => {
                    let byt = bytes.next().unwrap();
                    ops += format!("0xee  XRI    {byt:02x}\n").as_str()
                }
                0xef => ops += "0xef  RST    5\n",
                0xf0 => ops += "0xf0  RP\n",
                0xf1 => ops += "0xf1  POP PSW\n",
                0xf2 => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xf2  JP     {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xf3 => ops += "0xf3  DI\n",
                0xf4 => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xf4  CP     {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xf5 => ops += "0xf5  PUSH PSW\n",
                0xf6 => {
                    let byt = bytes.next().unwrap();
                    ops += format!("0xf6  ORI    {byt:02x}\n").as_str()
                }
                0xf7 => ops += "0xf7  RST    6\n",
                0xf8 => ops += "0xf8  RM\n",
                0xf9 => ops += "0xf9  SPHL\n",
                0xfa => {
                    let byt = bytes.next().unwrap();
                    ops += format!("0xfa  JM     {byt:02x}\n").as_str()
                }
                0xfb => ops += "0xfb  EI\n",
                0xfc => {
                    let adr2 = bytes.next().unwrap();
                    let adr1 = bytes.next().unwrap();
                    ops += format!("0xfc  CM     {adr1:02x}{adr2:02x}\n").as_str()
                }
                0xfd => ops += "0xfd  NOP\n",
                0xfe => {
                    let byt = bytes.next().unwrap();
                    ops += format!("0xfe  CPI    {byt:02x}\n").as_str()
                }
                0xff => ops += "0xff  RST    7\n",
            },
            None => end = true,
        };
    }

    ops
}
