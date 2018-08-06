use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct State {
    b: u8,
    pc: u16,
    sp: u16,
}

#[allow(dead_code)]
fn disassemble(pc: usize, buffer: &Vec<u8>) -> usize {
    let mut op_bytes = 1;
    let op_code = buffer[pc];

    print!("{:04x} {:02x} ", pc, op_code);

    match op_code {
        0x00 =>     println!("\tNOP"),
        0x06 => {   println!("{:02x}\tMVI\tB,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0x31 => {   println!("{:02x} {:02x}\tLXI\tSP${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0x3e => {   println!("{:02x}\tMVI\tA,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
        0xc3 => {   println!("{:02x} {:02x}\tJMP\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
        0xc5 =>     println!("\tPUSH\tB"),
        0xd5 =>     println!("\tPUSH\tD"),
        0xe5 =>     println!("\tPUSH\tH"),
        0xf5 =>     println!("\tPUSH\tPSW"),
        _ =>    {   println!("\t???"); op_bytes = 0; }
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

            state.sp = (byte_2<<8) | byte_1;
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
    let mut file = File::open("invaders.rom")?;
    let mut buffer = vec![];
    let mut state = State {
        b: 0,
        pc: 0,
        sp: 0,
    };

    file.read_to_end(&mut buffer)?;

    loop {
        println!("{:?}", state);
        disassemble(state.pc as usize, &buffer);

        state = step(state, &buffer);

        if state.pc == 0 { break }
    }

    // while pc < buffer.len() {
    //     let op_bytes = disassemble(pc, &buffer);
    //     if op_bytes == 0 { break }
    //     pc += op_bytes;
    // }

    Ok(())
}
