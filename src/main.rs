use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("invaders.rom")?;
    let mut buffer = vec![];

    file.read_to_end(&mut buffer)?;

    let mut pc = 0;

    while pc < buffer.len() {
        let mut op_bytes = 1;
        let op_code = buffer[pc];

        print!("{:04x} {:02x} ", pc, op_code);

        match op_code {
            0x00 =>     println!("\tNOP"),
            0xc3 => {   println!("{:02x} {:02x}\tJMP\t${1:02x}{0:02x}", buffer[pc + 1], buffer[pc + 2]); op_bytes = 3; },
            0xf5 =>     println!("\tPUSH\tPSW"),
            0xc5 =>     println!("\tPUSH\tB"),
            0xd5 =>     println!("\tPUSH\tD"),
            0xe5 =>     println!("\tPUSH\tH"),
            0x3e => {   println!("{:02x}\tMVI\tA,#${0:02x}", buffer[pc + 1]); op_bytes = 2 },
            _ =>    {   println!("\t???"); break; }
        };

        pc += op_bytes;
    }

    Ok(())
}
