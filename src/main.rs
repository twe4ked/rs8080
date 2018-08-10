use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct State {
    b: u8,
    pc: u16,
    sp: u16,
}

#[allow(dead_code)]
fn disassemble(pc: u16, buffer: &Vec<u8>) -> u16 {
    let mut op_bytes: u16 = 1;
    let pc = pc as usize;
    let op_code = buffer[pc];

    print!("{:04x} {:02x} ", pc, op_code);

    match op_code {
        0x00 => { println!("\tNOP") },
        0x01 => { println!("{:02x} {:02x}\tLXI\tB,${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x02 => { println!("\tSTAX B") },
        0x03 => { println!("\tINX B") },
        0x04 => { println!("\tINR\tB") },
        0x05 => { println!("\tDCR\tB") },
        0x06 => { println!("{:02x}\tMVI\tB,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x07 => { println!("\tRLC") },
        0x08 => { println!("\t-") },
        0x09 => { println!("\tDAD\tB") },
        0x0a => { println!("\tLDAX B") },
        0x0b => { println!("\tDCX B") },
        0x0c => { println!("\tINR C") },
        0x0d => { println!("\tDCR\tC") },
        0x0e => { println!("{:02x}\tMVI\tC,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x0f => { println!("\tRRC") },
        0x10 => { println!("\t-") },
        0x11 => { println!("{:02x} {:02x}\tLXI\tD,${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x12 => { println!("\tSTAX D") },
        0x13 => { println!("\tINX D") },
        0x14 => { println!("\tINR\tD") },
        0x15 => { println!("\tDCR\tD") },
        0x16 => { println!("{:02x}\tMVI\tD,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x18 => { println!("\t-") },
        0x19 => { println!("\tDAD\tD") },
        0x1a => { println!("\tLDAX D") },
        0x1b => { println!("\tDCX D") },
        0x1c => { println!("\tINR E") },
        0x1d => { println!("\tDCR E") },
        0x1f => { println!("\tRAR") },
        0x20 => { println!("\tRIM") },
        0x21 => { println!("{:02x} {:02x}\tLXI\tH,${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x22 => { println!("{:02x} {:02x}\tSHLD\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x23 => { println!("\tINX H") },
        0x24 => { println!("\tINR H") },
        0x25 => { println!("\tDCR H") },
        0x26 => { println!("{:02x}\tMVI\tH,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x27 => { println!("\tDAA\t") },
        0x28 => { println!("\t-") },
        0x29 => { println!("\tDAD\tH") },
        0x2a => { println!("{:02x} {:02x}\tLHLD\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x2b => { println!("\tDCX\tH") },
        0x2c => { println!("\tINR L") },
        0x2e => { println!("{:02x}\tMVI\tL,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x2f => { println!("\tCMA") },
        0x30 => { println!("\tSIM") },
        0x31 => { println!("{:02x} {:02x}\tLXI\tSP${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x32 => { println!("{:02x} {:02x}\tSTA\tA,${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x34 => { println!("\tINR M") },
        0x35 => { println!("\tDCR\tM") },
        0x36 => { println!("{:02x}\tMVI\tM,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x37 => { println!("\tSTC") },
        0x38 => { println!("\t-") },
        0x39 => { println!("\tDAD SP") },
        0x3a => { println!("{:02x} {:02x}\tLDA\tA,${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x3c => { println!("\tINR\tA") },
        0x3d => { println!("\tDCR\tA") },
        0x3e => { println!("{:02x}\tMVI\tA,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x3f => { println!("\tCMC") },
        0x41 => { println!("\tMOV B,C") },
        0x46 => { println!("\tMOV B,F") },
        0x47 => { println!("\tMOV B,A") },
        0x48 => { println!("\tMOV C,B") },
        0x4e => { println!("\tMOV C,M") },
        0x4f => { println!("\tMOV C,A") },
        0x54 => { println!("\tMOV D,H") },
        0x56 => { println!("\tMOV D,M") },
        0x57 => { println!("\tMOV D,A") },
        0x5e => { println!("\tMOV E,M") },
        0x5f => { println!("\tMOV E,A") },
        0x60 => { println!("\tMOV H,B") },
        0x61 => { println!("\tMOV H,C") },
        0x64 => { println!("\tMOV H,H") },
        0x65 => { println!("\tMOV H,L") },
        0x66 => { println!("\tMOV H,M") },
        0x67 => { println!("\tMOV H,A") },
        0x68 => { println!("\tMOV L,B") },
        0x69 => { println!("\tMOV L,C") },
        0x6e => { println!("\tMOV L,M") },
        0x6f => { println!("\tMOV L,A") },
        0x70 => { println!("\tMOV M,B") },
        0x71 => { println!("\tMOV M,C") },
        0x72 => { println!("\tMOV M,D") },
        0x73 => { println!("\tMOV M,E") },
        0x76 => { println!("\tHLT") },
        0x77 => { println!("\tMOV M,A") },
        0x78 => { println!("\tMOV A,B") },
        0x79 => { println!("\tMOV A,C") },
        0x7a => { println!("\tMOV A,D") },
        0x7b => { println!("\tMOV A,E") },
        0x7c => { println!("\tMOV A,H") },
        0x7d => { println!("\tMOV A,L") },
        0x7e => { println!("\tMOV A,M") },
        0x80 => { println!("\tADD B") },
        0x81 => { println!("\tADD C") },
        0x82 => { println!("\tADD D") },
        0x83 => { println!("\tADD E") },
        0x85 => { println!("\tADD L") },
        0x86 => { println!("\tADD M") },
        0x8a => { println!("\tADC D") },
        0x8b => { println!("\tADC E") },
        0x8e => { println!("\tADC M") },
        0x90 => { println!("\tSUB B") },
        0x97 => { println!("\tSUB A") },
        0x9b => { println!("\tSBB E") },
        0x9e => { println!("\tSBB M") },
        0xa0 => { println!("\tANA\tB") },
        0xa6 => { println!("\tANA\tM") },
        0xa7 => { println!("\tANA\tA") },
        0xa8 => { println!("\tANA\tB") },
        0xaf => { println!("\tXRA\tA") },
        0xb0 => { println!("\tORA B") },
        0xb4 => { println!("\tORA H") },
        0xb6 => { println!("\tORA M") },
        0xb8 => { println!("\tCPM B") },
        0xbb => { println!("\tCMP E") },
        0xbc => { println!("\tCMP H") },
        0xbe => { println!("\tCMP M") },
        0xc0 => { println!("\tRNZ") },
        0xc1 => { println!("\tPOP\tB") },
        0xc2 => { println!("{:02x} {:02x}\tJNZ\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xc3 => { println!("{:02x} {:02x}\tJMP\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xc4 => { println!("{:02x} {:02x}\tCNZ\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xc5 => { println!("\tPUSH\tB") },
        0xc6 => { println!("{:02x}\tADI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xc8 => { println!("\tRZ") },
        0xc9 => { println!("\tRET") },
        0xca => { println!("{:02x} {:02x}\tJZ\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xcc => { println!("{:02x} {:02x}\tCZ\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xcd => { println!("{:02x} {:02x}\tCALL\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xd0 => { println!("\tRNC") },
        0xd1 => { println!("\tPOP\tD") },
        0xd2 => { println!("{:02x} {:02x}\tJNC\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xd3 => { println!("{:02x}\tOUT\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xd4 => { println!("{:02x} {:02x}\tCNC\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xd5 => { println!("\tPUSH\tD") },
        0xd6 => { println!("{:02x}\tSUI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xd8 => { println!("\tRC") },
        0xda => { println!("{:02x} {:02x}\tJC\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xdb => { println!("{:02x}\tIN\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xde => { println!("{:02x}\tSBI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xe1 => { println!("\tPOP\tH") },
        0xe2 => { println!("{:02x} {:02x}\tJPO\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xe3 => { println!("\tXTHL") },
        0xe5 => { println!("\tPUSH\tH") },
        0xe6 => { println!("{:02x}\tANI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xe9 => { println!("\tPCHL") },
        0xeb => { println!("\tXCHG") },
        0xee => { println!("{:02x}\tXRI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xf1 => { println!("\tPOP\tPSW") },
        0xf5 => { println!("\tPUSH\tPSW") },
        0xf6 => { println!("{:02x}\tORI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xf8 => { println!("\tRM") },
        0xfa => { println!("{:02x} {:02x}\tJM\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xfb => { println!("\tEI") },
        0xfc => { println!("{:02x} {:02x}\tCM\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xfe => { println!("{:02x}\tCPI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xff => { println!("\tRST 7") },
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
