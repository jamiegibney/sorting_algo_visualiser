use super::*;

#[derive(Debug)]
pub struct Bucket {
    buckets: Vec<Vec<usize>>,
}

impl Bucket {
    pub fn new() -> Self {
        Self { buckets: Vec::new() }
    }

    fn insert(&mut self, bucket_idx: usize) {
        let bucket = &mut self.buckets[bucket_idx];
        for i in 1..bucket.len() {
            let key = bucket[i];
            let mut j = i as isize - 1;

            // while
        }
    }
}

impl SortAlgorithm for Bucket {
    fn process(&mut self, arr: &mut SortArray) {
        //
    }
}
