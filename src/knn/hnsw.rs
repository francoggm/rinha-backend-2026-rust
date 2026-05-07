use crate::repositories::fraud::FraudRepository;

pub struct HNSW {
    repository: FraudRepository,
}

impl HNSW {
    pub fn new(repository: FraudRepository) -> Self {
        HNSW {
            repository,
        }
    }
}