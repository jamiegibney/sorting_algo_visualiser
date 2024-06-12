use super::*;

#[derive(Debug)]
pub struct RadixMSD {
    base: usize,
}

impl RadixMSD {
    pub fn new(base: usize) -> Self {
        Self { base }
    }
}

impl SortAlgorithm for RadixMSD {
    fn process(&mut self, arr: &mut SortArray) {
        unimplemented!();
    }
}

