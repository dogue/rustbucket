use crate::cpu::Cpu6502;

pub fn load_a_immediate(cpu: &mut Cpu6502) {
    let byte = cpu.fetch_byte();
    cpu.a = byte;

    if cpu.a == 0 {
        cpu.flags.set_zero();
    }

    if (cpu.a & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    }
}

pub fn load_a_zeropage(cpu: &mut Cpu6502) {
    let byte = cpu.fetch_byte();
    cpu.set_pointer_high(0x00);
    cpu.set_pointer_low(byte);
    cpu.a = cpu.read_memory();

    if cpu.a == 0 {
        cpu.flags.set_zero();
    }

    if (cpu.a & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    }
}

pub fn load_a_zeropage_x(cpu: &mut Cpu6502) {
    // LDA zeropage, X
    let byte = cpu.fetch_byte();
    let byte = byte.wrapping_add(cpu.x);
    cpu.set_pointer_high(0x00);
    cpu.set_pointer_low(byte);
    cpu.a = cpu.read_memory();

    if cpu.a == 0 {
        cpu.flags.set_zero();
    }

    if (cpu.a & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    }
}

pub fn load_a_absolute(cpu: &mut Cpu6502) {
    // LDA absolute
    let low = cpu.fetch_byte();
    let high = cpu.fetch_byte();
    cpu.set_pointer_high(high);
    cpu.set_pointer_low(low);
    cpu.a = cpu.read_memory();

    if cpu.a == 0 {
        cpu.flags.set_zero();
    }

    if (cpu.a & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    }
}

pub fn load_a_absolute_x(cpu: &mut Cpu6502) {
    // LDA absolute, X
    let low = cpu.fetch_byte();
    let high = cpu.fetch_byte();
    let low = low.wrapping_add(cpu.x);
    cpu.set_pointer_high(high);
    cpu.set_pointer_low(low);
    cpu.a = cpu.read_memory();

    if cpu.a == 0 {
        cpu.flags.set_zero();
    }

    if (cpu.a & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    }
}

pub fn load_a_absolute_y(cpu: &mut Cpu6502) {
    // LDA absolute, Y
    let low = cpu.fetch_byte();
    let high = cpu.fetch_byte();
    let low = low.wrapping_add(cpu.y);
    cpu.set_pointer_high(high);
    cpu.set_pointer_low(low);
    cpu.a = cpu.read_memory();

    if cpu.a == 0 {
        cpu.flags.set_zero();
    }

    if (cpu.a & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    }
}

pub fn load_a_indirect_x(cpu: &mut Cpu6502) {
    // LDA (indirect, X)
    let zp = cpu.fetch_byte();
    let zp = zp.wrapping_add(cpu.x);
    let low = cpu.memory[zp as usize];
    let high = cpu.memory[(zp + 1) as usize];
    cpu.set_pointer_high(high);
    cpu.set_pointer_low(low);
    cpu.a = cpu.read_memory();

    if cpu.a == 0 {
        cpu.flags.set_zero();
    }

    if (cpu.a & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    }
}

pub fn load_a_indirect_y(cpu: &mut Cpu6502) {
    // LDA (indirect), Y
    let zp = cpu.fetch_byte();
    let low = cpu.memory[zp as usize];
    let high = cpu.memory[(zp + 1) as usize];
    let low = low.wrapping_add(cpu.y);
    cpu.set_pointer_high(high);
    cpu.set_pointer_low(low);
    cpu.a = cpu.read_memory();

    if cpu.a == 0 {
        cpu.flags.set_zero();
    }

    if (cpu.a & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_a_immediate() {
        let program: Vec<u8> = vec![0xA9, 0x69, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.run();

        assert_eq!(cpu.a, 0x69);
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
    fn load_a_zeropage_x() {
        let program: Vec<u8> = vec![0xB5, 0x68, 0xFF, 0xB5, 0xFF, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.memory[0x0069] = 0xFF;
        cpu.memory[0x0000] = 0xFF;
        cpu.x = 0x01;

        cpu.run();
        assert_eq!(cpu.a, 0xFF);

        cpu.halted = false;
        cpu.run();
        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn load_a_absolute() {
        let program: Vec<u8> = vec![0xAD, 0x00, 0x80, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.run();

        assert_eq!(cpu.a, 0xAD);
    }

    #[test]
    fn load_a_absolute_x() {
        let program: Vec<u8> = vec![0xBD, 0x68, 0x42, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.memory[0x4269] = 0xFF;
        cpu.x = 0x01;
        cpu.run();

        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn load_a_absolute_y() {
        let program: Vec<u8> = vec![0xB9, 0x68, 0x42, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.memory[0x4269] = 0xFF;
        cpu.y = 0x01;
        cpu.run();

        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn load_a_indirect_x() {
        let program: Vec<u8> = vec![0xA1, 0x10, 0xFF];
        let mut cpu = Cpu6502::with_program(program);

        cpu.x = 0x04;
        cpu.memory[0x14] = 0x20;
        cpu.memory[0x20] = 0xFF;

        cpu.run();

        assert_eq!(cpu.a, 0xFF);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.negative);
    }

    #[test]
    fn load_a_indirect_y() {
        let program: Vec<u8> = vec![0xB1, 0x42, 0xFF];
        let mut cpu = Cpu6502::with_program(program);

        cpu.y = 0x01;
        cpu.memory[0x0042] = 0x68;
        cpu.memory[0x0043] = 0x69;
        cpu.memory[0x6969] = 0xFF;

        cpu.run();

        assert_eq!(cpu.a, 0xFF);
    }
}
