use crate::repositories::fraud::FraudRepository;

pub struct KDTree {
    repository: FraudRepository,
}

impl KDTree {
    pub fn new(repository: FraudRepository) -> Self {
        KDTree {
            repository,
        }
    }
}