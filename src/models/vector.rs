use serde::Deserialize;

pub const VECTOR_DIM: usize = 14;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Label {
    Legit,
    Fraud,
}

#[derive(Deserialize)]
pub struct JsonVectorData {
    pub vector: Vec<f64>,
    pub label: String,
}

pub struct VectorData {
    pub vector: [f64; VECTOR_DIM],
    pub label: Label,
}
