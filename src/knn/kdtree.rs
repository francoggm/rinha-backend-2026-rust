use crate::models::vector::{Label, VECTOR_DIM};
use crate::repositories::fraud::FraudRepository;

const K: usize = 5;

struct Node {
    index: u32,
    split_dim: u8,
    split_val: f32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct KdTree {
    repository: FraudRepository,
    root: Option<Box<Node>>,
}

impl KdTree {
    pub fn new(repository: FraudRepository) -> Self {
        let n = repository.len();
        let mut indices: Vec<u32> = (0..n as u32).collect();

        log::info!("Building KdTree over {} vectors...", n);
        let root = Self::build(&repository, &mut indices, 0);
        log::info!("KdTree built");

        KdTree { repository, root }
    }

    fn build(repo: &FraudRepository, indices: &mut [u32], depth: usize) -> Option<Box<Node>> {
        if indices.is_empty() {
            return None;
        }

        let split_dim = depth % VECTOR_DIM;

        indices.sort_unstable_by(|&a, &b| {
            let va = repo.get_component(a as usize, split_dim);
            let vb = repo.get_component(b as usize, split_dim);
            va.partial_cmp(&vb).unwrap_or(std::cmp::Ordering::Equal)
        });

        let median = indices.len() / 2;
        let pivot = indices[median];
        let split_val = repo.get_component(pivot as usize, split_dim);

        let (left_slice, right_slice) = indices.split_at_mut(median);
        let right_slice = &mut right_slice[1..];

        let left = Self::build(repo, left_slice, depth + 1);
        let right = Self::build(repo, right_slice, depth + 1);

        Some(Box::new(Node {
            index: pivot,
            split_dim: split_dim as u8,
            split_val,
            left,
            right,
        }))
    }

    pub fn knn(&self, query: &[f32; VECTOR_DIM]) -> f64 {
        let mut heap = BoundedMaxHeap::new(K);
        if let Some(ref root) = self.root {
            self.search(root, query, &mut heap);
        }

        let mut fraud_count = 0u32;
        let total = heap.len() as f64;
        for &(_, idx) in heap.items() {
            if self.repository.get_label(idx as usize) == Label::Fraud {
                fraud_count += 1;
            }
        }

        fraud_count as f64 / total
    }

    fn search(&self, node: &Node, query: &[f32; VECTOR_DIM], heap: &mut BoundedMaxHeap) {
        let dist = self.squared_distance(node.index as usize, query);
        heap.push(dist, node.index);

        let dim = node.split_dim as usize;
        let diff = query[dim] - node.split_val;

        let (first, second) = if diff <= 0.0 {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };

        if let Some(ref child) = first {
            self.search(child, query, heap);
        }

        let diff_sq = diff * diff;
        if !heap.is_full() || diff_sq < heap.max_dist() {
            if let Some(ref child) = second {
                self.search(child, query, heap);
            }
        }
    }

    fn squared_distance(&self, index: usize, query: &[f32; VECTOR_DIM]) -> f32 {
        let vec = self.repository.get_vector(index);
        let mut sum = 0.0f32;
        for i in 0..VECTOR_DIM {
            let d = vec[i] - query[i];
            sum += d * d;
        }
        sum
    }
}

struct BoundedMaxHeap {
    capacity: usize,
    data: Vec<(f32, u32)>,
}

impl BoundedMaxHeap {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: Vec::with_capacity(capacity + 1),
        }
    }

    fn push(&mut self, dist: f32, index: u32) {
        if self.data.len() < self.capacity {
            self.data.push((dist, index));
            self.data.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        } else if dist < self.data[0].0 {
            self.data[0] = (dist, index);
            self.data.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        }
    }

    fn is_full(&self) -> bool {
        self.data.len() >= self.capacity
    }

    fn max_dist(&self) -> f32 {
        self.data[0].0
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn items(&self) -> &[(f32, u32)] {
        &self.data
    }
}
