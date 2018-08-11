use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct State {
    b: u8,
    pc: u16,
    sp: u16,
}

fn disassemble_1(instruction: &'static str) -> u16 {
    println!("\t{instruction}", instruction = instruction);
    1
}

fn disassemble_2(instruction: &'static str, byte_1: u8) -> u16 {
    println!("{byte_1:02x}\t{instruction}\t${byte_1:02x}", instruction = instruction, byte_1 = byte_1);
    2
}

fn disassemble_3(instruction: &'static str, byte_1: u8, byte_2: u8) -> u16 {
    println!("{byte_1:02x} {byte_2:02x}\t{instruction}\t${byte_2:02x}{byte_1:02x}", instruction = instruction, byte_1 = byte_1, byte_2 = byte_2);
    3
}

fn not_implemented() -> u16 {
    println!("\t???");
    0
}

#[allow(dead_code)]
fn disassemble(pc: u16, buffer: &Vec<u8>) -> u16 {
    let pc = pc as usize;
    let byte_1 = buffer[pc];
    let byte_2 = *buffer.get(pc + 1).unwrap_or(&0);
    let byte_3 = *buffer.get(pc + 2).unwrap_or(&0);

    print!("{:04x} {:02x} ", pc, byte_1);

    let op_bytes = match byte_1 {
        0x00 => { disassemble_1("NOP") },
        0x01 => { disassemble_3("LXI B", byte_2, byte_3) },
        0x02 => { disassemble_1("STAX B") },
        0x03 => { disassemble_1("INX B") },
        0x04 => { disassemble_1("INR B") },
        0x05 => { disassemble_1("DCR B") },
        0x06 => { disassemble_2("MVI B", byte_2) },
        0x07 => { disassemble_1("RLC") },
        0x08 => { disassemble_1("-") },
        0x09 => { disassemble_1("DAD B") },
        0x0a => { disassemble_1("LDAX B") },
        0x0b => { disassemble_1("DCX B") },
        0x0c => { disassemble_1("INR C") },
        0x0d => { disassemble_1("DCR C") },
        0x0e => { disassemble_2("MVI C", byte_2) },
        0x0f => { disassemble_1("RRC") },
        0x10 => { disassemble_1("-") },
        0x11 => { disassemble_3("LXI D", byte_2, byte_3) },
        0x12 => { disassemble_1("STAX D") },
        0x13 => { disassemble_1("INX D") },
        0x14 => { disassemble_1("INR D") },
        0x15 => { disassemble_1("DCR D") },
        0x16 => { disassemble_2("MVI D", byte_2) },
        0x17 => { not_implemented() },
        0x18 => { disassemble_1("-") },
        0x19 => { disassemble_1("DAD D") },
        0x1a => { disassemble_1("LDAX D") },
        0x1b => { disassemble_1("DCX D") },
        0x1c => { disassemble_1("INR E") },
        0x1d => { disassemble_1("DCR E") },
        0x1e => { disassemble_2("MVI E", byte_2) },
        0x1f => { disassemble_1("RAR") },
        0x20 => { disassemble_1("RIM") },
        0x21 => { disassemble_3("LXI H", byte_2, byte_3) },
        0x22 => { disassemble_3("SHLD", byte_2, byte_3) },
        0x23 => { disassemble_1("INX H") },
        0x24 => { disassemble_1("INR H") },
        0x25 => { disassemble_1("DCR H") },
        0x26 => { disassemble_2("MVI H", byte_2) },
        0x27 => { disassemble_1("DAA") },
        0x28 => { disassemble_1("-") },
        0x29 => { disassemble_1("DAD H") },
        0x2a => { disassemble_3("LHLD", byte_2, byte_3) },
        0x2b => { disassemble_1("DCX H") },
        0x2c => { disassemble_1("INR L") },
        0x2d => { not_implemented() },
        0x2e => { disassemble_2("MVI L", byte_2) },
        0x2f => { disassemble_1("CMA") },
        0x30 => { disassemble_1("SIM") },
        0x31 => { disassemble_3("LXI SP", byte_2, byte_3) },
        0x32 => { disassemble_3("STA A", byte_2, byte_3) },
        0x33 => { not_implemented() },
        0x34 => { disassemble_1("INR M") },
        0x35 => { disassemble_1("DCR M") },
        0x36 => { disassemble_2("MVI M", byte_2) },
        0x37 => { disassemble_1("STC") },
        0x38 => { disassemble_1("-") },
        0x39 => { disassemble_1("DAD SP") },
        0x3a => { disassemble_3("LDA A", byte_2, byte_3) },
        0x3b => { not_implemented() },
        0x3c => { disassemble_1("INR A") },
        0x3d => { disassemble_1("DCR A") },
        0x3e => { disassemble_2("MVI A", byte_2) },
        0x3f => { disassemble_1("CMC") },
        0x40 => { disassemble_1("MOV B,B") },
        0x41 => { disassemble_1("MOV B,C") },
        0x42 => { disassemble_1("MOV B,D") },
        0x43 => { disassemble_1("MOV B,E") },
        0x44 => { disassemble_1("MOV B,H") },
        0x45 => { disassemble_1("MOV B,L") },
        0x46 => { disassemble_1("MOV B,F") },
        0x47 => { disassemble_1("MOV B,A") },
        0x48 => { disassemble_1("MOV C,B") },
        0x49 => { disassemble_1("MOV C,C") },
        0x4a => { disassemble_1("MOV C,D") },
        0x4b => { disassemble_1("MOV C,E") },
        0x4c => { disassemble_1("MOV C,H") },
        0x4d => { disassemble_1("MOV C,L") },
        0x4e => { disassemble_1("MOV C,M") },
        0x4f => { disassemble_1("MOV C,A") },
        0x50 => { disassemble_1("MOV D,B") },
        0x51 => { disassemble_1("MOV D,C") },
        0x52 => { not_implemented() },
        0x53 => { not_implemented() },
        0x54 => { disassemble_1("MOV D,H") },
        0x55 => { not_implemented() },
        0x56 => { disassemble_1("MOV D,M") },
        0x57 => { disassemble_1("MOV D,A") },
        0x58 => { not_implemented() },
        0x59 => { disassemble_1("MOV E,C") },
        0x5a => { not_implemented() },
        0x5b => { disassemble_1("MOV E,E") },
        0x5c => { not_implemented() },
        0x5d => { not_implemented() },
        0x5e => { disassemble_1("MOV E,M") },
        0x5f => { disassemble_1("MOV E,A") },
        0x60 => { disassemble_1("MOV H,B") },
        0x61 => { disassemble_1("MOV H,C") },
        0x62 => { disassemble_1("MOV H,D") },
        0x63 => { disassemble_1("MOV H,E") },
        0x64 => { disassemble_1("MOV H,H") },
        0x65 => { disassemble_1("MOV H,L") },
        0x66 => { disassemble_1("MOV H,M") },
        0x67 => { disassemble_1("MOV H,A") },
        0x68 => { disassemble_1("MOV L,B") },
        0x69 => { disassemble_1("MOV L,C") },
        0x6a => { not_implemented() },
        0x6b => { not_implemented() },
        0x6c => { disassemble_1("MOV L,H") },
        0x6d => { disassemble_1("MOV L,L") },
        0x6e => { disassemble_1("MOV L,M") },
        0x6f => { disassemble_1("MOV L,A") },
        0x70 => { disassemble_1("MOV M,B") },
        0x71 => { disassemble_1("MOV M,C") },
        0x72 => { disassemble_1("MOV M,D") },
        0x73 => { disassemble_1("MOV M,E") },
        0x74 => { disassemble_1("MOV M,H") },
        0x75 => { not_implemented() },
        0x76 => { disassemble_1("HLT") },
        0x77 => { disassemble_1("MOV M,A") },
        0x78 => { disassemble_1("MOV A,B") },
        0x79 => { disassemble_1("MOV A,C") },
        0x7a => { disassemble_1("MOV A,D") },
        0x7b => { disassemble_1("MOV A,E") },
        0x7c => { disassemble_1("MOV A,H") },
        0x7d => { disassemble_1("MOV A,L") },
        0x7e => { disassemble_1("MOV A,M") },
        0x7f => { disassemble_1("MOV A,A") },
        0x80 => { disassemble_1("ADD B") },
        0x81 => { disassemble_1("ADD C") },
        0x82 => { disassemble_1("ADD D") },
        0x83 => { disassemble_1("ADD E") },
        0x84 => { disassemble_1("ADD H") },
        0x85 => { disassemble_1("ADD L") },
        0x86 => { disassemble_1("ADD M") },
        0x87 => { not_implemented() },
        0x88 => { disassemble_1("ADC B") },
        0x89 => { not_implemented() },
        0x8a => { disassemble_1("ADC D") },
        0x8b => { disassemble_1("ADC E") },
        0x8c => { not_implemented() },
        0x8d => { not_implemented() },
        0x8e => { disassemble_1("ADC M") },
        0x8f => { not_implemented() },
        0x90 => { disassemble_1("SUB B") },
        0x91 => { not_implemented() },
        0x92 => { not_implemented() },
        0x93 => { not_implemented() },
        0x94 => { disassemble_1("SUB H") },
        0x95 => { not_implemented() },
        0x96 => { not_implemented() },
        0x97 => { disassemble_1("SUB A") },
        0x98 => { disassemble_1("SBB B") },
        0x99 => { disassemble_1("SBB C") },
        0x9a => { disassemble_1("SBB D") },
        0x9b => { disassemble_1("SBB E") },
        0x9c => { not_implemented() },
        0x9d => { disassemble_1("SBB L") },
        0x9e => { disassemble_1("SBB M") },
        0x9f => { not_implemented() },
        0xa0 => { disassemble_1("ANA B") },
        0xa1 => { not_implemented() },
        0xa2 => { not_implemented() },
        0xa3 => { disassemble_1("ANA E") },
        0xa4 => { not_implemented() },
        0xa5 => { not_implemented() },
        0xa6 => { disassemble_1("ANA M") },
        0xa7 => { disassemble_1("ANA A") },
        0xa8 => { disassemble_1("ANA B") },
        0xa9 => { not_implemented() },
        0xaa => { disassemble_1("XRA D") },
        0xab => { not_implemented() },
        0xac => { not_implemented() },
        0xad => { not_implemented() },
        0xae => { not_implemented() },
        0xaf => { disassemble_1("XRA A") },
        0xb0 => { disassemble_1("ORA B") },
        0xb1 => { not_implemented() },
        0xb2 => { not_implemented() },
        0xb3 => { disassemble_1("ORA E") },
        0xb4 => { disassemble_1("ORA H") },
        0xb5 => { not_implemented() },
        0xb6 => { disassemble_1("ORA M") },
        0xb7 => { not_implemented() },
        0xb8 => { disassemble_1("CPM B") },
        0xb9 => { not_implemented() },
        0xba => { not_implemented() },
        0xbb => { disassemble_1("CMP E") },
        0xbc => { disassemble_1("CMP H") },
        0xbd => { not_implemented() },
        0xbe => { disassemble_1("CMP M") },
        0xbf => { not_implemented() },
        0xc0 => { disassemble_1("RNZ") },
        0xc1 => { disassemble_1("POP B") },
        0xc2 => { disassemble_3("JNZ", byte_2, byte_3) },
        0xc3 => { disassemble_3("JMP", byte_2, byte_3) },
        0xc4 => { disassemble_3("CNZ", byte_2, byte_3) },
        0xc5 => { disassemble_1("PUSH B") },
        0xc6 => { disassemble_2("ADI", byte_2) },
        0xc7 => { not_implemented() },
        0xc8 => { disassemble_1("RZ") },
        0xc9 => { disassemble_1("RET") },
        0xca => { disassemble_3("JZ", byte_2, byte_3) },
        0xcb => { not_implemented() },
        0xcc => { disassemble_3("CZ", byte_2, byte_3) },
        0xcd => { disassemble_3("CALL", byte_2, byte_3) },
        0xce => { not_implemented() },
        0xcf => { not_implemented() },
        0xd0 => { disassemble_1("RNC") },
        0xd1 => { disassemble_1("POP D") },
        0xd2 => { disassemble_3("JNC", byte_2, byte_3) },
        0xd3 => { disassemble_2("OUT", byte_2) },
        0xd4 => { disassemble_3("CNC", byte_2, byte_3) },
        0xd5 => { disassemble_1("PUSH D") },
        0xd6 => { disassemble_2("SUI", byte_2) },
        0xd7 => { not_implemented() },
        0xd8 => { disassemble_1("RC") },
        0xd9 => { not_implemented() },
        0xda => { disassemble_3("JC", byte_2, byte_3) },
        0xdb => { disassemble_2("IN", byte_2) },
        0xdc => { not_implemented() },
        0xdd => { not_implemented() },
        0xde => { disassemble_2("SBI", byte_2) },
        0xdf => { not_implemented() },
        0xe0 => { disassemble_1("RPO") },
        0xe1 => { disassemble_1("POP H") },
        0xe2 => { disassemble_3("JPO", byte_2, byte_3) },
        0xe3 => { disassemble_1("XTHL") },
        0xe4 => { not_implemented() },
        0xe5 => { disassemble_1("PUSH H") },
        0xe6 => { disassemble_2("ANI", byte_2) },
        0xe7 => { not_implemented() },
        0xe8 => { not_implemented() },
        0xe9 => { disassemble_1("PCHL") },
        0xea => { not_implemented() },
        0xeb => { disassemble_1("XCHG") },
        0xec => { disassemble_3("CPE", byte_2, byte_3) },
        0xed => { not_implemented() },
        0xee => { disassemble_2("XRI", byte_2) },
        0xef => { not_implemented() },
        0xf0 => { disassemble_1("RP") },
        0xf1 => { disassemble_1("POP PSW") },
        0xf2 => { not_implemented() },
        0xf3 => { not_implemented() },
        0xf4 => { not_implemented() },
        0xf5 => { disassemble_1("PUSH PSW") },
        0xf6 => { disassemble_2("ORI", byte_2) },
        0xf7 => { not_implemented() },
        0xf8 => { disassemble_1("RM") },
        0xf9 => { not_implemented() },
        0xfa => { disassemble_3("JM", byte_2, byte_3) },
        0xfb => { disassemble_1("EI") },
        0xfc => { disassemble_3("CM", byte_2, byte_3) },
        0xfd => { not_implemented() },
        0xfe => { disassemble_2("CPI", byte_2) },
        0xff => { disassemble_1("RST 7") },
           _ => unreachable!()
    };

    op_bytes
}

fn read_16(buffer: &Vec<u8>, pc: usize) -> u16 {
    ((buffer[pc + 2] as u16) << 8) | (buffer[pc + 1] as u16)
}

fn step(mut state: State, buffer: &Vec<u8>) -> State {
    let pc = state.pc as usize;
    let op_code = buffer[pc];

    state.pc += 1;

    match op_code {
        0x00 => { },
        0x06 => {
            state.b = buffer[pc + 1];
            state.pc += 1;
        },
        0x31 => {
            state.sp = read_16(&buffer, pc as usize);
            state.pc += 2;
        },
        0xc3 => {
            state.pc = read_16(&buffer, pc as usize);
        },
        _   => { state.pc = 0 },
    }

    state
}

fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    let mut file = File::open("invaders.rom")?;
    let mut buffer = vec![];
    let mut state = State {
        b: 0,
        pc: 0,
        sp: 0,
    };

    file.read_to_end(&mut buffer)?;

    if args.get(1) == Some(&"disassemble".to_string()) {
        while state.pc < buffer.len() as u16 {
            let op_bytes = disassemble(state.pc, &buffer);
            if op_bytes == 0 { break }
            state.pc += op_bytes;
        }
    } else {
        loop {
            println!("{:?}", state);
            disassemble(state.pc, &buffer);

            state = step(state, &buffer);

            if state.pc == 0 { break }
        }
    }

    Ok(())
}
