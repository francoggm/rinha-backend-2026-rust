#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Label {
    Legit,
    Fraud,
}

pub const VECTOR_DIM: usize = 14;
pub const RECORD_SIZE: usize = VECTOR_DIM * 4 + 1; // 14 f32s + 1 byte label
pub const HEADER_SIZE: usize = 6; // 4 bytes count (u32 LE) + 2 bytes dim (u16 LE)
