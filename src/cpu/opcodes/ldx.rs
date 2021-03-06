use crate::cpu::Cpu6502;

pub fn load_x_immediate(cpu: &mut Cpu6502) {
    let byte = cpu.fetch_byte();
    cpu.x = byte;

    if cpu.x == 0 {
        cpu.flags.set_zero();
    } else {
        cpu.flags.clear_zero();
    }

    if (cpu.x & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    } else {
        cpu.flags.clear_negative();
    }
}

pub fn load_x_zeropage(cpu: &mut Cpu6502) {
    let byte = cpu.fetch_byte();
    cpu.set_pointer_high(0x00);
    cpu.set_pointer_low(byte);
    cpu.x = cpu.read_memory();

    if cpu.x == 0 {
        cpu.flags.set_zero();
    } else {
        cpu.flags.clear_zero();
    }

    if (cpu.x & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    } else {
        cpu.flags.clear_negative();
    }
}

pub fn load_x_zeropage_y(cpu: &mut Cpu6502) {
    let byte = cpu.fetch_byte();
    let byte = byte.wrapping_add(cpu.y);
    cpu.set_pointer_high(0x00);
    cpu.set_pointer_low(byte);
    cpu.x = cpu.read_memory();

    if cpu.x == 0 {
        cpu.flags.set_zero();
    } else {
        cpu.flags.clear_zero();
    }

    if (cpu.x & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    } else {
        cpu.flags.clear_negative();
    }
}

pub fn load_x_absolute(cpu: &mut Cpu6502) {
    let low = cpu.fetch_byte();
    let high = cpu.fetch_byte();
    cpu.set_pointer_high(high);
    cpu.set_pointer_low(low);
    cpu.x = cpu.read_memory();

    if cpu.x == 0 {
        cpu.flags.set_zero();
    } else {
        cpu.flags.clear_zero();
    }

    if (cpu.x & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    } else {
        cpu.flags.clear_negative();
    }
}

pub fn load_x_absolute_y(cpu: &mut Cpu6502) {
    let low = cpu.fetch_byte();
    let high = cpu.fetch_byte();
    let low = low.wrapping_add(cpu.y);
    cpu.set_pointer_high(high);
    cpu.set_pointer_low(low);
    cpu.x = cpu.read_memory();

    if cpu.x == 0 {
        cpu.flags.set_zero();
    } else {
        cpu.flags.clear_zero();
    }

    if (cpu.x & (1 << 7)) != 0 {
        cpu.flags.set_negative();
    } else {
        cpu.flags.clear_negative();
    }
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
    fn load_x_zeropage() {
        let program: Vec<u8> = vec![0xA6, 0x69, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.memory[0x69] = 0x69;
        cpu.run();

        assert_eq!(cpu.x, 0x69);
    }

    #[test]
    fn load_x_zeropage_y() {
        let program: Vec<u8> = vec![0xB6, 0x00, 0xFF];
        let mut cpu = Cpu6502::with_program(program);

        cpu.memory[0x01] = 0x69;
        cpu.y = 0x01;
        cpu.run();
        assert_eq!(cpu.x, 0x69);

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
    fn load_x_absolute() {
        let program: Vec<u8> = vec![0xAE, 0x00, 0x80, 0xFF];
        let mut cpu = Cpu6502::with_program(program);
        cpu.run();

        assert_eq!(cpu.x, 0xAE);
    }

    #[test]
    fn load_x_absolute_y() {
        let program: Vec<u8> = vec![0xBE, 0x68, 0x42, 0xFF];
        let mut cpu = Cpu6502::with_program(program);

        cpu.memory[0x4269] = 0x69;
        cpu.y = 0x01;
        cpu.run();
        assert_eq!(cpu.x, 0x69);

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
