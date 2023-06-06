#[derive(Debug, Default)]
pub struct Register<W: UnsignedInt> {
    pub value: W,
}

pub trait UnsignedInt: Copy {}
impl UnsignedInt for u8 {}
impl UnsignedInt for u16 {}
impl UnsignedInt for u32 {}
impl UnsignedInt for u64 {}
impl UnsignedInt for u128 {}
