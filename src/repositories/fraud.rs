use std::fs::File;

use memmap2::Mmap;

use crate::models::vector::{Label, VECTOR_DIM};

const HEADER_SIZE: usize = 6; // 4 bytes count (u32 LE) + 2 bytes dim (u16 LE)
const RECORD_SIZE: usize = VECTOR_DIM * 8 + 1; // 14 f64s + 1 byte label

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

    pub fn get_vector(&self, index: usize) -> [f64; VECTOR_DIM] {
        let offset = HEADER_SIZE + index * RECORD_SIZE;
        let bytes = &self.mmap[offset..offset + VECTOR_DIM * 8];
        let mut vector = [0.0f64; VECTOR_DIM];
        for i in 0..VECTOR_DIM {
            vector[i] = f64::from_le_bytes(bytes[i * 8..(i + 1) * 8].try_into().unwrap());
        }
        vector
    }

    pub fn get_label(&self, index: usize) -> Label {
        let offset = HEADER_SIZE + index * RECORD_SIZE + VECTOR_DIM * 8;
        if self.mmap[offset] == 1 {
            Label::Fraud
        } else {
            Label::Legit
        }
    }
}
