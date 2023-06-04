mod cpu6502;

use cpu6502::prelude::*;

fn main() {
    let program: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF];

    let cpu = Cpu6502::with_program(program);

    for i in 0x8000..0x8004 {
        print!("{:X?}", cpu.memory[i]);
    }
    println!();
}
