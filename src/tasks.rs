#[derive(Debug)]
pub enum Task {
    FetchByte,
    FetchLow,
    FetchHigh,
    SetLow(u8),
    SetHigh(u8),
    AddLow(u8),
    AddHigh(u8),
    MemoryRead,
    MemoryWrite,
}
