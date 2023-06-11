#[derive(Debug, Default)]
pub struct Flags {
    carry: bool,
    zero: bool,
    interrupt_disable: bool,
    decimal: bool,
    bit0: bool,
    bit1: bool,
    overflow: bool,
    negative: bool,
}

impl Flags {
    pub fn set_carry(&mut self) {
        self.carry = true;
    }

    pub fn clear_carry(&mut self) {
        self.carry = false;
    }

    pub fn set_zero(&mut self) {
        self.zero = true;
    }

    pub fn clear_zero(&mut self) {
        self.zero = false;
    }

    pub fn set_interrupt_disable(&mut self) {
        self.interrupt_disable = true;
    }

    pub fn clear_interrupt_disable(&mut self) {
        self.interrupt_disable = false;
    }

    pub fn set_decimal(&mut self) {
        self.decimal = true;
    }

    pub fn clear_decimal(&mut self) {
        self.decimal = false;
    }

    pub fn set_bit0(&mut self) {
        self.bit0 = true;
    }

    pub fn clear_bit0(&mut self) {
        self.bit0 = false;
    }

    pub fn set_bit1(&mut self) {
        self.bit1 = true;
    }

    pub fn clear_bit1(&mut self) {
        self.bit1 = false;
    }

    pub fn set_overflow(&mut self) {
        self.overflow = true;
    }

    pub fn clear_overflow(&mut self) {
        self.overflow = false;
    }

    pub fn set_negative(&mut self) {
        self.negative = true;
    }

    pub fn clear_negative(&mut self) {
        self.negative = false;
    }
}
