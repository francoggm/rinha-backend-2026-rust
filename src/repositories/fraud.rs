use std::fs::File;

use memmap2::Mmap;

use crate::models::vector::{Label, HEADER_SIZE, RECORD_SIZE, VECTOR_DIM};

pub struct FraudRepository {
    mmap: Mmap,
    count: usize,
}

impl FraudRepository {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).expect("Failed to open references.bin");
        let mmap = unsafe { Mmap::map(&file).expect("Failed to mmap references.bin") };

        let count = u32::from_le_bytes(mmap[0..4].try_into().unwrap()) as usize;
        let dim = u16::from_le_bytes(mmap[4..6].try_into().unwrap()) as usize;
        assert_eq!(dim, VECTOR_DIM, "Binary file dimension mismatch");

        log::info!("Mapped {} vectors from {} (zero-copy)", count, path);

        Self { mmap, count }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn get_vector(&self, index: usize) -> [f32; VECTOR_DIM] {
        let offset = HEADER_SIZE + index * RECORD_SIZE;
        let bytes = &self.mmap[offset..offset + VECTOR_DIM * 4];
        let mut vector = [0.0f32; VECTOR_DIM];
        for i in 0..VECTOR_DIM {
            vector[i] = f32::from_le_bytes(bytes[i * 4..(i + 1) * 4].try_into().unwrap());
        }
        vector
    }

    pub fn get_component(&self, index: usize, dim: usize) -> f32 {
        let offset = HEADER_SIZE + index * RECORD_SIZE + dim * 4;
        f32::from_le_bytes(self.mmap[offset..offset + 4].try_into().unwrap())
    }

    pub fn get_label(&self, index: usize) -> Label {
        let offset = HEADER_SIZE + index * RECORD_SIZE + VECTOR_DIM * 4;
        if self.mmap[offset] == 1 {
            Label::Fraud
        } else {
            Label::Legit
        }
    }
}
