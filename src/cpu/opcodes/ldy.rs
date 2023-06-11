use crate::cpu::Cpu6502;

pub fn load_y_immediate(cpu: &mut Cpu6502) {
    let byte = cpu.fetch_byte();
    cpu.y = byte;

    if cpu.y == 0 {
        cpu.flags.set_zero();
    } else {
        cpu.flags.clear_zero();
    }

    if (cpu.y & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    } else {
        cpu.flags.clear_negative();
    }
}

pub fn load_y_zeropage(cpu: &mut Cpu6502) {
    let byte = cpu.fetch_byte();
    cpu.set_pointer_high(0x00);
    cpu.set_pointer_low(byte);
    cpu.y = cpu.read_memory();

    if cpu.y == 0 {
        cpu.flags.set_zero();
    } else {
        cpu.flags.clear_zero();
    }

    if (cpu.y & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    } else {
        cpu.flags.clear_negative();
    }
}

pub fn load_y_zeropage_x(cpu: &mut Cpu6502) {
    let byte = cpu.fetch_byte();
    let byte = byte.wrapping_add(cpu.x);
    cpu.set_pointer_high(0x00);
    cpu.set_pointer_low(byte);
    cpu.y = cpu.read_memory();

    if cpu.y == 0 {
        cpu.flags.set_zero();
    } else {
        cpu.flags.clear_zero();
    }

    if (cpu.y & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    } else {
        cpu.flags.clear_negative();
    }
}

pub fn load_y_absolute(cpu: &mut Cpu6502) {
    let low = cpu.fetch_byte();
    let high = cpu.fetch_byte();
    cpu.set_pointer_high(high);
    cpu.set_pointer_low(low);
    cpu.y = cpu.read_memory();

    if cpu.y == 0 {
        cpu.flags.set_zero();
    } else {
        cpu.flags.clear_zero();
    }

    if (cpu.y & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    } else {
        cpu.flags.clear_negative();
    }
}

pub fn load_y_absolute_x(cpu: &mut Cpu6502) {
    let low = cpu.fetch_byte();
    let high = cpu.fetch_byte();
    let low = low.wrapping_add(cpu.x);
    cpu.set_pointer_high(high);
    cpu.set_pointer_low(low);
    cpu.y = cpu.read_memory();

    if cpu.y == 0 {
        cpu.flags.set_zero();
    } else {
        cpu.flags.clear_zero();
    }

    if (cpu.y & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    } else {
        cpu.flags.clear_negative();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_y_immediate() {
        let program: Vec<u8> = vec![0xA0, 0x69, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.run();

        assert_eq!(cpu.y, 0x69);
    }

    #[test]
    fn load_y_zeropage() {
        let program: Vec<u8> = vec![0xA4, 0x69, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.memory[0x69] = 0x69;
        cpu.run();

        assert_eq!(cpu.y, 0x69);
    }

    #[test]
    fn load_y_zeropage_x() {
        let program: Vec<u8> = vec![0xB4, 0x00, 0xFF];
        let mut cpu = Cpu6502::with_program(program);

        cpu.memory[0x01] = 0x69;
        cpu.x = 0x01;
        cpu.run();
        assert_eq!(cpu.y, 0x69);

        cpu.memory[0x01] = 0x00;
        cpu.reset();
        cpu.run();
        assert!(cpu.flags.zero);

        cpu.memory[0x01] = 0xFF;
        cpu.reset();
        cpu.run();
        assert!(cpu.flags.negative);
    }

    #[test]
    fn load_y_absolute() {
        let program: Vec<u8> = vec![0xAC, 0x00, 0x80, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.run();

        assert_eq!(cpu.y, 0xAC);
    }

    #[test]
    fn load_y_absolute_x() {
        let program: Vec<u8> = vec![0xBC, 0x68, 0x42, 0xFF];
        let mut cpu = Cpu6502::with_program(program);

        cpu.memory[0x4269] = 0x69;
        cpu.x = 0x01;
        cpu.run();
        assert_eq!(cpu.y, 0x69);

        cpu.memory[0x4269] = 0x00;
        cpu.reset();
        cpu.run();
        assert!(cpu.flags.zero);

        cpu.memory[0x4269] = 0xFF;
        cpu.reset();
        cpu.run();
        assert!(cpu.flags.negative);
    }
}
