use std::mem::size_of;

#[derive(Debug, Default)]
pub struct Cpu6502 {
    pub a: Register<u8>,
    pub x: Register<u8>,
    pub y: Register<u8>,
    pub z: Register<u8>,
    pub ip: Register<u16>,
    pub sp: Register<u16>,
    pub memory: Vec<u8>,
}

impl Cpu6502 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_program(program: Vec<u8>) -> Self {
        let mut cpu = Self::default();
        cpu.memory = vec![0; 0xFFFF];
        cpu.load_program(program);
        cpu
    }

    fn load_program(&mut self, program: Vec<u8>) {
        // Program length can vary, so we can't just
        // .splice() as that shrinks the memory Vec
        let mut i: usize = 0x8000;
        for v in program {
            self.memory[i] = v;
            i += 1;
        }

        // reset vector
        self.memory[0xFFFC] = 0x00;
        self.memory[0xFFFD] = 0x80;
    }
}

#[derive(Debug, Default)]
pub struct Register<W: UnsignedInt> {
    contents: W,
}

pub trait UnsignedInt {}
impl UnsignedInt for u8 {}
impl UnsignedInt for u16 {}
