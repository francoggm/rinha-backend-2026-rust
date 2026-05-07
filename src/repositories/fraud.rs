use std::fs;

use crate::models::vector::{Label, JsonVectorData, VectorData, VECTOR_DIM};

pub struct FraudRepository {
    data: Vec<VectorData>,
}

impl FraudRepository {
    pub fn new(path: &str) -> Self {
        let mut raw = fs::read(path).expect("Failed to read references JSON file");
        let json_data: Vec<JsonVectorData> = simd_json::from_slice(&mut raw)
            .expect("Failed to parse JSON");

        let data: Vec<VectorData> = json_data.into_iter().map(|entry| {
            let mut vector = [0.0f64; VECTOR_DIM];
            for (i, &v) in entry.vector.iter().enumerate().take(VECTOR_DIM) {
                vector[i] = v;
            }
            let label = if entry.label == "fraud" { Label::Fraud } else { Label::Legit };
            VectorData { vector, label }
        }).collect();

        log::info!("Loaded {} vectors from {}", data.len(), path);

        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_vector(&self, index: usize) -> [f64; VECTOR_DIM] {
        self.data[index].vector
    }

    pub fn get_label(&self, index: usize) -> Label {
        self.data[index].label
    }
}
