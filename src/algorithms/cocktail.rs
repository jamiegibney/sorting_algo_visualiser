use super::*;

#[derive(Debug)]
pub struct Cocktail {}

impl Cocktail {
    pub fn new() -> Self {
        Self {}
    }
}

impl SortAlgorithm for Cocktail {
    fn process(&mut self, arr: &mut SortArray) {
        //
    }
}
