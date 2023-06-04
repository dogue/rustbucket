use rustbucket::Register;

#[derive(Debug)]
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
        cpu.load_program(program);
        cpu
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        // splice() will remove n elements, even if n is larger than the size
        // of the iterator being spliced in, so we must calculate the start
        // and end of the range based on program size
        let (start, end): (usize, usize) = (0x8000, 0x8000 + program.len());
        self.memory.splice(start..end, program);

        // reset vector
        self.memory[0xFFFC] = 0x00;
        self.memory[0xFFFD] = 0x80;
    }
}

impl Default for Cpu6502 {
    fn default() -> Self {
        // We must fill the memory vec to set the len()
        // or we'll get index bounds panics in load_program()
        let memory: Vec<u8> = vec![0; 0xFFFF];

        Self {
            a: Register::default(),
            x: Register::default(),
            y: Register::default(),
            z: Register::default(),
            ip: Register::default(),
            sp: Register::default(),
            memory,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const PROG: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];

    #[test]
    fn load_program_into_memory() {
        let mut cpu = Cpu6502::new();
        cpu.load_program(PROG.to_vec());

        assert_eq!(cpu.memory[0x8000], 0xDE);
        assert_eq!(cpu.memory[0x8001], 0xAD);
        assert_eq!(cpu.memory[0x8002], 0xBE);
        assert_eq!(cpu.memory[0x8003], 0xEF);
    }

    #[test]
    fn set_reset_vector_on_load() {
        let mut cpu = Cpu6502::new();
        cpu.load_program(PROG.to_vec());

        assert_eq!(cpu.memory[0xFFFC], 0x00);
        assert_eq!(cpu.memory[0xFFFD], 0x80);
    }
}
