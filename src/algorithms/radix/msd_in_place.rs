use super::*;

#[derive(Debug)]
pub struct RadixMSDInPlace {
    base: usize,
}

impl RadixMSDInPlace {
    pub fn new(base: usize) -> Self {
        Self { base }
    }
}

impl SortAlgorithm for RadixMSDInPlace {
    fn process(&mut self, arr: &mut SortArray) {
        unimplemented!();
        //
    }
}

