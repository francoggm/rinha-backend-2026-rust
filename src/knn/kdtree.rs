use crate::models::vector::{Label, VECTOR_DIM};
use crate::repositories::fraud::FraudRepository;

const K: usize = 7;
const LEAF_SIZE: usize = 32;

pub struct KdTree {
    repository: FraudRepository,
    indices: Vec<u32>,
}

impl KdTree {
    pub fn new(repository: FraudRepository) -> Self {
        let count = repository.len();
        let mut indices: Vec<u32> = (0..count as u32).collect();

        log::info!("Building KdTree for {} vectors...", count);
        let mut stack: Vec<(usize, usize, usize)> = Vec::with_capacity(64);
        stack.push((0, count, 0));

        while let Some((start, end, depth)) = stack.pop() {
            if end - start <= LEAF_SIZE {
                continue;
            }

            let dim = depth % VECTOR_DIM;
            let mid = (start + end) / 2;

            nth_element(&repository, &mut indices[start..end], mid - start, dim);

            stack.push((start, mid, depth + 1));
            stack.push((mid + 1, end, depth + 1));
        }

        log::info!("KdTree built.");
        KdTree { repository, indices }
    }

    pub fn knn(&self, query: &[f32; VECTOR_DIM]) -> f64 {
        let mut heap = BoundedMaxHeap::new();
        self.search(query, 0, self.indices.len(), 0, &mut heap);

        let mut fraud_count = 0u32;
        for i in 0..heap.len {
            let idx = heap.items[i].index;
            if self.repository.get_label(idx as usize) == Label::Fraud {
                fraud_count += 1;
            }
        }
        fraud_count as f64 / heap.len as f64
    }

    fn search(
        &self,
        query: &[f32; VECTOR_DIM],
        start: usize,
        end: usize,
        depth: usize,
        heap: &mut BoundedMaxHeap,
    ) {
        if start >= end {
            return;
        }

        if end - start <= LEAF_SIZE {
            for i in start..end {
                let dist = self.squared_distance(query, self.indices[i] as usize);
                heap.push(self.indices[i], dist);
            }
            return;
        }

        let dim = depth % VECTOR_DIM;
        let mid = (start + end) / 2;
        let mid_idx = self.indices[mid] as usize;

        let dist = self.squared_distance(query, mid_idx);
        heap.push(self.indices[mid], dist);

        let split_val = self.repository.get_component(mid_idx, dim);
        let diff = query[dim] - split_val;

        let (first_start, first_end, second_start, second_end) = if diff <= 0.0 {
            (start, mid, mid + 1, end)
        } else {
            (mid + 1, end, start, mid)
        };

        self.search(query, first_start, first_end, depth + 1, heap);

        if !heap.is_full() || diff * diff < heap.max_dist() {
            self.search(query, second_start, second_end, depth + 1, heap);
        }
    }

    fn squared_distance(&self, query: &[f32; VECTOR_DIM], index: usize) -> f32 {
        let vec = self.repository.get_vector(index);
        let mut sum = 0.0f32;
        for i in 0..VECTOR_DIM {
            let d = query[i] - vec[i];
            sum += d * d;
        }
        sum
    }
}

fn nth_element(repo: &FraudRepository, slice: &mut [u32], k: usize, dim: usize) {
    if slice.len() <= 1 {
        return;
    }

    let mut left = 0;
    let mut right = slice.len() - 1;

    while left < right {
        let pivot_idx = median_of_three(repo, slice, left, right, dim);
        let pivot_val = repo.get_component(slice[pivot_idx] as usize, dim);
        slice.swap(pivot_idx, right);

        let mut store = left;
        for i in left..right {
            if repo.get_component(slice[i] as usize, dim) < pivot_val {
                slice.swap(i, store);
                store += 1;
            }
        }
        slice.swap(store, right);

        if store == k {
            return;
        } else if k < store {
            right = store - 1;
        } else {
            left = store + 1;
        }
    }
}

fn median_of_three(repo: &FraudRepository, slice: &[u32], left: usize, right: usize, dim: usize) -> usize {
    let mid = (left + right) / 2;
    let a = repo.get_component(slice[left] as usize, dim);
    let b = repo.get_component(slice[mid] as usize, dim);
    let c = repo.get_component(slice[right] as usize, dim);

    if (a <= b && b <= c) || (c <= b && b <= a) {
        mid
    } else if (b <= a && a <= c) || (c <= a && a <= b) {
        left
    } else {
        right
    }
}

struct BoundedMaxHeap {
    items: [HeapItem; K],
    len: usize,
}

#[derive(Clone, Copy)]
struct HeapItem {
    index: u32,
    dist: f32,
}

impl BoundedMaxHeap {
    fn new() -> Self {
        Self {
            items: [HeapItem { index: 0, dist: 0.0 }; K],
            len: 0,
        }
    }

    fn is_full(&self) -> bool {
        self.len == K
    }

    fn max_dist(&self) -> f32 {
        if self.len == 0 {
            f32::MAX
        } else {
            self.items[0].dist
        }
    }

    fn push(&mut self, index: u32, dist: f32) {
        if self.len < K {
            self.items[self.len] = HeapItem { index, dist };
            self.len += 1;
            self.sift_up(self.len - 1);
        } else if dist < self.items[0].dist {
            self.items[0] = HeapItem { index, dist };
            self.sift_down(0);
        }
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.items[idx].dist > self.items[parent].dist {
                self.items.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut largest = idx;

            if left < self.len && self.items[left].dist > self.items[largest].dist {
                largest = left;
            }
            if right < self.len && self.items[right].dist > self.items[largest].dist {
                largest = right;
            }
            if largest == idx {
                break;
            }
            self.items.swap(idx, largest);
            idx = largest;
        }
    }
}
