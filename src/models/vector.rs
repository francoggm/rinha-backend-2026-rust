pub const VECTOR_DIM: usize = 14;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Label {
    Legit,
    Fraud,
}

pub struct VectorData {
    pub vector: [f64; VECTOR_DIM],
    pub label: Label,
}
