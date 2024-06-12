use super::*;

#[derive(Debug)]
pub struct Pancake {}

impl Pancake {
    pub fn new() -> Self {
        Self {}
    }
}

impl SortAlgorithm for Pancake {
    fn process(&mut self, arr: &mut SortArray) {
        //
    }
}
