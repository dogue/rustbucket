use std::collections::VecDeque;

use crate::tasks::Task;

#[derive(Debug)]
pub enum Register {
    A,
    X,
    Y,
    Z,
    Ip,
    Sp,
}

#[derive(Debug)]
pub struct Cpu6502 {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub ip: u16,
    pub sp: u16,
    pub memory: Vec<u8>,

    // This is a pointer into the 6502's memory space
    // It is for managing interal state of the emulator
    // and is not part of the 6502
    pointer: u16,
    task_queue: VecDeque<Task>,
    target_register: *mut u8,
    halted: bool,
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

        self.reset();
    }

    pub fn reset(&mut self) {
        self.set_pointer_low(self.memory[0xFFFC]);
        self.set_pointer_high(self.memory[0xFFFD]);
        self.ip = self.pointer;
    }

    pub fn run(&mut self) {
        loop {
            if self.halted {
                break;
            }

            if self.task_queue.is_empty() {
                self.decode_next();
                self.ip += 1;
                continue;
            }

            match self.task_queue.pop_front().unwrap() {
                Task::FetchByte => {
                    self.write_target(self.fetch_byte());
                    self.ip += 1;
                }
                Task::FetchLow => {
                    self.set_pointer_low(self.fetch_byte());
                    self.ip += 1;
                }
                Task::FetchHigh => {
                    self.set_pointer_high(self.fetch_byte());
                    self.ip += 1;
                }
                Task::SetLow(v) => self.set_pointer_low(v),
                Task::SetHigh(v) => self.set_pointer_high(v),
                Task::AddLow(v) => {
                    self.set_pointer_low(self.get_pointer_low() + v);
                }
                Task::AddHigh(v) => self.set_pointer_high(self.get_pointer_high() + v),
                Task::MemoryRead => self.write_target(self.read_memory()),
                Task::MemoryWrite => self.write_memory(self.read_target()),
            }
        }
    }

    fn read_target(&self) -> u8 {
        unsafe { *self.target_register }
    }

    fn write_target(&mut self, value: u8) {
        unsafe { *self.target_register = value };
    }

    fn set_target(&mut self, target: Register) {
        self.target_register = match target {
            Register::A => &mut (self).a as *mut u8,
            Register::X => &mut (self).x as *mut u8,
            Register::Y => &mut (self).y as *mut u8,
            _ => panic!("You should not be writing here: {:?}", target),
        }
    }

    fn decode_next(&mut self) {
        let opcode = self.fetch_byte();
        self.decode(opcode);
    }

    fn set_pointer_high(&mut self, value: u8) {
        self.pointer = (self.pointer & 0x00FF) | ((value as u16) << 8);
    }

    fn set_pointer_low(&mut self, value: u8) {
        self.pointer = (self.pointer & 0xFF00) | (value as u16);
    }

    fn get_pointer_high(&self) -> u8 {
        (self.pointer >> 8) as u8
    }

    fn get_pointer_low(&self) -> u8 {
        self.pointer as u8
    }

    fn fetch_byte(&self) -> u8 {
        self.memory[self.ip as usize]
    }

    fn read_memory(&self) -> u8 {
        self.memory[self.pointer as usize]
    }

    fn write_memory(&mut self, value: u8) {
        self.memory[self.pointer as usize] = value;
    }

    fn decode(&mut self, opcode: u8) {
        match opcode {
            0x00 => {}
            0x01 => {}
            0x05 => {}
            0x06 => {}
            0x08 => {}
            0x09 => {}
            0x0A => {}
            0x0D => {}
            0x0E => {}
            0x10 => {}
            0x11 => {}
            0x12 => {}
            0x15 => {}
            0x16 => {}
            0x18 => {}
            0x19 => {}
            0x1D => {}
            0x1E => {}
            0x20 => {}
            0x21 => {}
            0x24 => {}
            0x25 => {}
            0x26 => {}
            0x28 => {}
            0x29 => {}
            0x2A => {}
            0x2C => {}
            0x2D => {}
            0x2E => {}
            0x30 => {}
            0x31 => {}
            0x35 => {}
            0x36 => {}
            0x38 => {}
            0x39 => {}
            0x3D => {}
            0x3E => {}
            0x40 => {}
            0x41 => {}
            0x45 => {}
            0x46 => {}
            0x48 => {}
            0x49 => {}
            0x4A => {}
            0x4C => {}
            0x4D => {}
            0x4E => {}
            0x50 => {}
            0x51 => {}
            0x55 => {}
            0x56 => {}
            0x58 => {}
            0x59 => {}
            0x5D => {}
            0x5E => {}
            0x60 => {}
            0x61 => {}
            0x65 => {}
            0x66 => {}
            0x68 => {}
            0x69 => {}
            0x6A => {}
            0x6C => {}
            0x6D => {}
            0x6E => {}
            0x70 => {}
            0x71 => {}
            0x75 => {}
            0x76 => {}
            0x78 => {}
            0x79 => {}
            0x7D => {}
            0x7E => {}
            0x81 => {}
            0x84 => {}
            0x85 => {}
            0x86 => {}
            0x88 => {}
            0x8A => {}
            0x8C => {}
            0x8D => {}
            0x8E => {}
            0x90 => {}
            0x91 => {}
            0x94 => {}
            0x95 => {}
            0x96 => {}
            0x98 => {}
            0x99 => {}
            0x9A => {}
            0x9D => {}
            0xA0 => {}
            0xA1 => {}
            0xA2 => {}
            0xA4 => {}
            0xA5 => {
                self.set_target(Register::A);
                self.set_pointer_high(0x00);
                self.task_queue.push_back(Task::FetchLow);
                self.task_queue.push_back(Task::MemoryRead);
            }
            0xA6 => {}
            0xA8 => {}
            0xA9 => {
                self.set_target(Register::A);
                self.task_queue.push_back(Task::FetchByte);
            }
            0xAA => {}
            0xAC => {}
            0xAD => {
                self.set_target(Register::A);
                self.task_queue.push_back(Task::FetchLow);
                self.task_queue.push_back(Task::FetchHigh);
                self.task_queue.push_back(Task::MemoryRead);
            }
            0xAE => {}
            0xB0 => {}
            0xB1 => {}
            0xB4 => {}
            0xB5 => {}
            0xB6 => {}
            0xB8 => {}
            0xB9 => {}
            0xBA => {}
            0xBC => {}
            0xBD => {
                self.set_target(Register::A);
                self.task_queue.push_back(Task::FetchLow);
                self.task_queue.push_back(Task::FetchHigh);
                self.task_queue.push_back(Task::AddLow(self.x));
                self.task_queue.push_back(Task::MemoryRead);
            }
            0xBE => {}
            0xC0 => {}
            0xC1 => {}
            0xC4 => {}
            0xC5 => {}
            0xC6 => {}
            0xC8 => {}
            0xC9 => {}
            0xCA => {}
            0xCC => {}
            0xCD => {}
            0xCE => {}
            0xD0 => {}
            0xD1 => {}
            0xD5 => {}
            0xD6 => {}
            0xD8 => {}
            0xD9 => {}
            0xDD => {}
            0xDE => {}
            0xE0 => {}
            0xE1 => {}
            0xE4 => {}
            0xE5 => {}
            0xE6 => {}
            0xE8 => {}
            0xE9 => {}
            0xEA => {}
            0xEC => {}
            0xED => {}
            0xEE => {}
            0xF0 => {}
            0xF1 => {}
            0xF5 => {}
            0xF6 => {}
            0xF8 => {}
            0xF9 => {}
            0xFA => {}
            0xFD => {}
            0xFE => {}
            0xFF => self.halted = true,
            _ => {}
        }
    }
}

impl Default for Cpu6502 {
    fn default() -> Self {
        // We must fill the memory vec to set the len()
        // or we'll get index bounds panics in load_program()
        let memory: Vec<u8> = vec![0; 0xFFFF];

        Self {
            a: 0,
            x: 0,
            y: 0,
            z: 0,
            ip: 0,
            sp: 0,
            memory,
            pointer: 0,
            task_queue: VecDeque::new(),
            target_register: &mut 0,
            halted: false,
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

    #[test]
    fn load_a_immediate() {
        let program: Vec<u8> = vec![0xA9, 0x69, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.run();

        assert_eq!(cpu.a, 0x69);
    }

    #[test]
    fn load_a_absolute() {
        let program: Vec<u8> = vec![0xAD, 0x00, 0x80, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.run();

        assert_eq!(cpu.a, 0xAD);
    }

    #[test]
    fn load_a_zeropage() {
        let program: Vec<u8> = vec![0xA5, 0x69, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.memory[0x69] = 0x69;
        cpu.run();

        assert_eq!(cpu.a, 0x69);
    }

    #[test]
    fn load_a_absolute_x() {
        let program: Vec<u8> = vec![0xBD, 0x68, 0x00, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.memory[0x69] = 0xFF;
        cpu.x = 0x01;
        cpu.run();

        assert_eq!(cpu.a, 0xFF);
    }
}
