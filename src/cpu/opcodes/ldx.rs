use crate::cpu::Cpu6502;

pub fn load_x_immediate(cpu: &mut Cpu6502) {
    // LDX immediate
    let byte = cpu.fetch_byte();
    cpu.x = byte;
}

pub fn load_x_zeropage(cpu: &mut Cpu6502) {
    let byte = cpu.fetch_byte();
    cpu.set_pointer_high(0x00);
    cpu.set_pointer_low(byte);
    cpu.x = cpu.read_memory();
}

pub fn load_x_absolute(cpu: &mut Cpu6502) {
    let low = cpu.fetch_byte();
    let high = cpu.fetch_byte();
    cpu.set_pointer_high(high);
    cpu.set_pointer_low(low);
    cpu.x = cpu.read_memory();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_x_immediate() {
        let program: Vec<u8> = vec![0xA2, 0x69, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.run();

        assert_eq!(cpu.x, 0x69);
    }

    #[test]
    fn load_x_absolute() {
        let program: Vec<u8> = vec![0xAE, 0x00, 0x80, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.run();

        assert_eq!(cpu.x, 0xAE);
    }

    #[test]
    fn load_x_zeropage() {
        let program: Vec<u8> = vec![0xA6, 0x69, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.memory[0x69] = 0x69;
        cpu.run();

        assert_eq!(cpu.x, 0x69);
    }
}
