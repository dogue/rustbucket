#[derive(Debug)]
pub enum AddressingMode {
    Accumulator,
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    Immediate(u8),
    Implied,
    Indirect(u16),
    XIndirect(u8),
    IndirectY(u8),
    Relative(u8),
    ZeroPage(u8),
    ZeroPageX(u8),
    ZeroPageY(u8),
}
