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
        0x06 => { println!("{:02x}\tMVI\tB,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x07 => { println!("\tRLC") },
        0x09 => { println!("\tDAD\tB") },
        0x0f => { println!("\tRRC") },
        0x16 => { println!("{:02x}\tMVI\tD,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x21 => { println!("{:02x} {:02x}\tLXI\tH,${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x22 => { println!("{:02x} {:02x}\tSHLD\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x23 => { println!("\tINX H") },
        0x27 => { println!("\tDAA\t") },
        0x2b => { println!("\tDCX\tH") },
        0x31 => { println!("{:02x} {:02x}\tLXI\tSP${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x32 => { println!("{:02x} {:02x}\tSTA\tA,${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x35 => { println!("\tDCR\tM") },
        0x3a => { println!("{:02x} {:02x}\tLDA\tA,${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x3c => { println!("\tINR\tA") },
        0x3d => { println!("\tDCR\tA") },
        0x3e => { println!("{:02x}\tMVI\tA,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x46 => { println!("\tMOV B,F") },
        0x5f => { println!("\tMOV E,A") },
        0x66 => { println!("\tMOV H,M") },
        0x67 => { println!("\tMOV H,A") },
        0x6f => { println!("\tMOV L,A") },
        0x7e => { println!("\tMOV A,M") },
        0xa7 => { println!("\tANA\tA") },
        0xaf => { println!("\tXRA\tA") },
        0xc1 => { println!("\tPOP\tB") },
        0xc2 => { println!("{:02x} {:02x}\tJNZ\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xc3 => { println!("{:02x} {:02x}\tJMP\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xc5 => { println!("\tPUSH\tB") },
        0xc6 => { println!("{:02x}\tADI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xc9 => { println!("\tRET") },
        0xca => { println!("{:02x} {:02x}\tJZ\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xcd => { println!("{:02x} {:02x}\tCALL\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xd1 => { println!("\tPOP\tD") },
        0xd2 => { println!("{:02x} {:02x}\tJNC\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xd5 => { println!("\tPUSH\tD") },
        0xda => { println!("{:02x} {:02x}\tJC\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xdb => { println!("{:02x}\tIN\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xe1 => { println!("\tPOP\tH") },
        0xe5 => { println!("\tPUSH\tH") },
        0xe6 => { println!("{:02x}\tANI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xf1 => { println!("\tPOP\tPSW") },
        0xf5 => { println!("\tPUSH\tPSW") },
        0xfb => { println!("\tEI") },
        0xfe => { println!("{:02x}\tCPI\t#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
           _ => { println!("\t???"); op_bytes = 0; }
    };

    op_bytes
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
            let byte_1 = buffer[pc + 1] as u16;
            let byte_2 = buffer[pc + 2] as u16;

            state.sp = (byte_2 << 8) | byte_1;
            state.pc += 2;
        },
        0xc3 => {
            let byte_1 = buffer[pc + 1] as u16;
            let byte_2 = buffer[pc + 2] as u16;

            state.pc = (byte_2 << 8) | byte_1;
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
