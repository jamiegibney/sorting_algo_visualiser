use super::*;

#[derive(Debug)]
pub struct Stooge {}

impl Stooge {
    pub fn new() -> Self {
        Self {}
    }
}

impl SortAlgorithm for Stooge {
    fn process(&mut self, arr: &mut SortArray) {
        //
    }
}
