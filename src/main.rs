use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct State {
    b: u8,
    pc: u16,
    sp: u16,
}

fn disassemble_1(instruction: &'static str) {
    println!("\t{instruction}", instruction = instruction)
}

#[allow(dead_code)]
fn disassemble(pc: u16, buffer: &Vec<u8>) -> u16 {
    let mut op_bytes: u16 = 1;
    let pc = pc as usize;
    let op_code = buffer[pc];

    print!("{:04x} {:02x} ", pc, op_code);

    match op_code {
        0x00 => { disassemble_1("NOP") },
        0x01 => { println!("{:02x} {:02x}\tLXI\tB,${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x02 => { disassemble_1("STAX B") },
        0x03 => { disassemble_1("INX B") },
        0x04 => { disassemble_1("INR\tB") },
        0x05 => { disassemble_1("DCR\tB") },
        0x06 => { println!("{:02x}\tMVI\tB,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x07 => { disassemble_1("RLC") },
        0x08 => { disassemble_1("-") },
        0x09 => { disassemble_1("DAD\tB") },
        0x0a => { disassemble_1("LDAX B") },
        0x0b => { disassemble_1("DCX B") },
        0x0c => { disassemble_1("INR C") },
        0x0d => { disassemble_1("DCR\tC") },
        0x0e => { println!("{:02x}\tMVI\tC,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x0f => { disassemble_1("RRC") },
        0x10 => { disassemble_1("-") },
        0x11 => { println!("{:02x} {:02x}\tLXI\tD,${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x12 => { disassemble_1("STAX D") },
        0x13 => { disassemble_1("INX D") },
        0x14 => { disassemble_1("INR\tD") },
        0x15 => { disassemble_1("DCR\tD") },
        0x16 => { println!("{:02x}\tMVI\tD,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x18 => { disassemble_1("-") },
        0x19 => { disassemble_1("DAD\tD") },
        0x1a => { disassemble_1("LDAX D") },
        0x1b => { disassemble_1("DCX D") },
        0x1c => { disassemble_1("INR E") },
        0x1d => { disassemble_1("DCR E") },
        0x1f => { disassemble_1("RAR") },
        0x20 => { disassemble_1("RIM") },
        0x21 => { println!("{:02x} {:02x}\tLXI\tH,${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x22 => { println!("{:02x} {:02x}\tSHLD\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x23 => { disassemble_1("INX H") },
        0x24 => { disassemble_1("INR H") },
        0x25 => { disassemble_1("DCR H") },
        0x26 => { println!("{:02x}\tMVI\tH,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x27 => { disassemble_1("DAA\t") },
        0x28 => { disassemble_1("-") },
        0x29 => { disassemble_1("DAD\tH") },
        0x2a => { println!("{:02x} {:02x}\tLHLD\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x2b => { disassemble_1("DCX\tH") },
        0x2c => { disassemble_1("INR L") },
        0x2e => { println!("{:02x}\tMVI\tL,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x2f => { disassemble_1("CMA") },
        0x30 => { disassemble_1("SIM") },
        0x31 => { println!("{:02x} {:02x}\tLXI\tSP${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x32 => { println!("{:02x} {:02x}\tSTA\tA,${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x34 => { disassemble_1("INR M") },
        0x35 => { disassemble_1("DCR\tM") },
        0x36 => { println!("{:02x}\tMVI\tM,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x37 => { disassemble_1("STC") },
        0x38 => { disassemble_1("-") },
        0x39 => { disassemble_1("DAD SP") },
        0x3a => { println!("{:02x} {:02x}\tLDA\tA,${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x3c => { disassemble_1("INR\tA") },
        0x3d => { disassemble_1("DCR\tA") },
        0x3e => { println!("{:02x}\tMVI\tA,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x3f => { disassemble_1("CMC") },
        0x41 => { disassemble_1("MOV B,C") },
        0x46 => { disassemble_1("MOV B,F") },
        0x47 => { disassemble_1("MOV B,A") },
        0x48 => { disassemble_1("MOV C,B") },
        0x4e => { disassemble_1("MOV C,M") },
        0x4f => { disassemble_1("MOV C,A") },
        0x54 => { disassemble_1("MOV D,H") },
        0x56 => { disassemble_1("MOV D,M") },
        0x57 => { disassemble_1("MOV D,A") },
        0x5e => { disassemble_1("MOV E,M") },
        0x5f => { disassemble_1("MOV E,A") },
        0x60 => { disassemble_1("MOV H,B") },
        0x61 => { disassemble_1("MOV H,C") },
        0x64 => { disassemble_1("MOV H,H") },
        0x65 => { disassemble_1("MOV H,L") },
        0x66 => { disassemble_1("MOV H,M") },
        0x67 => { disassemble_1("MOV H,A") },
        0x68 => { disassemble_1("MOV L,B") },
        0x69 => { disassemble_1("MOV L,C") },
        0x6e => { disassemble_1("MOV L,M") },
        0x6f => { disassemble_1("MOV L,A") },
        0x70 => { disassemble_1("MOV M,B") },
        0x71 => { disassemble_1("MOV M,C") },
        0x72 => { disassemble_1("MOV M,D") },
        0x73 => { disassemble_1("MOV M,E") },
        0x76 => { disassemble_1("HLT") },
        0x77 => { disassemble_1("MOV M,A") },
        0x78 => { disassemble_1("MOV A,B") },
        0x79 => { disassemble_1("MOV A,C") },
        0x7a => { disassemble_1("MOV A,D") },
        0x7b => { disassemble_1("MOV A,E") },
        0x7c => { disassemble_1("MOV A,H") },
        0x7d => { disassemble_1("MOV A,L") },
        0x7e => { disassemble_1("MOV A,M") },
        0x80 => { disassemble_1("ADD B") },
        0x81 => { disassemble_1("ADD C") },
        0x82 => { disassemble_1("ADD D") },
        0x83 => { disassemble_1("ADD E") },
        0x85 => { disassemble_1("ADD L") },
        0x86 => { disassemble_1("ADD M") },
        0x8a => { disassemble_1("ADC D") },
        0x8b => { disassemble_1("ADC E") },
        0x8e => { disassemble_1("ADC M") },
        0x90 => { disassemble_1("SUB B") },
        0x97 => { disassemble_1("SUB A") },
        0x9b => { disassemble_1("SBB E") },
        0x9e => { disassemble_1("SBB M") },
        0xa0 => { disassemble_1("ANA\tB") },
        0xa6 => { disassemble_1("ANA\tM") },
        0xa7 => { disassemble_1("ANA\tA") },
        0xa8 => { disassemble_1("ANA\tB") },
        0xaf => { disassemble_1("XRA\tA") },
        0xb0 => { disassemble_1("ORA B") },
        0xb4 => { disassemble_1("ORA H") },
        0xb6 => { disassemble_1("ORA M") },
        0xb8 => { disassemble_1("CPM B") },
        0xbb => { disassemble_1("CMP E") },
        0xbc => { disassemble_1("CMP H") },
        0xbe => { disassemble_1("CMP M") },
        0xc0 => { disassemble_1("RNZ") },
        0xc1 => { disassemble_1("POP\tB") },
        0xc2 => { println!("{:02x} {:02x}\tJNZ\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xc3 => { println!("{:02x} {:02x}\tJMP\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xc4 => { println!("{:02x} {:02x}\tCNZ\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xc5 => { disassemble_1("PUSH\tB") },
        0xc6 => { println!("{:02x}\tADI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xc8 => { disassemble_1("RZ") },
        0xc9 => { disassemble_1("RET") },
        0xca => { println!("{:02x} {:02x}\tJZ\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xcc => { println!("{:02x} {:02x}\tCZ\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xcd => { println!("{:02x} {:02x}\tCALL\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xd0 => { disassemble_1("RNC") },
        0xd1 => { disassemble_1("POP\tD") },
        0xd2 => { println!("{:02x} {:02x}\tJNC\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xd3 => { println!("{:02x}\tOUT\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xd4 => { println!("{:02x} {:02x}\tCNC\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xd5 => { disassemble_1("PUSH\tD") },
        0xd6 => { println!("{:02x}\tSUI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xd8 => { disassemble_1("RC") },
        0xda => { println!("{:02x} {:02x}\tJC\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xdb => { println!("{:02x}\tIN\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xde => { println!("{:02x}\tSBI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xe1 => { disassemble_1("POP\tH") },
        0xe2 => { println!("{:02x} {:02x}\tJPO\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xe3 => { disassemble_1("XTHL") },
        0xe5 => { disassemble_1("PUSH\tH") },
        0xe6 => { println!("{:02x}\tANI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xe9 => { disassemble_1("PCHL") },
        0xeb => { disassemble_1("XCHG") },
        0xee => { println!("{:02x}\tXRI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xf1 => { disassemble_1("POP\tPSW") },
        0xf5 => { disassemble_1("PUSH\tPSW") },
        0xf6 => { println!("{:02x}\tORI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xf8 => { disassemble_1("RM") },
        0xfa => { println!("{:02x} {:02x}\tJM\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xfb => { disassemble_1("EI") },
        0xfc => { println!("{:02x} {:02x}\tCM\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xfe => { println!("{:02x}\tCPI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xff => { disassemble_1("RST 7") },
           _ => { println!("\t???"); op_bytes = 0; }
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
