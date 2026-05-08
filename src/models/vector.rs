#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Label {
    Legit,
    Fraud,
}

pub const VECTOR_DIM: usize = 14;
