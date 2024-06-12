use super::*;

#[derive(Debug)]
pub struct Shell {}

impl Shell {
    pub fn new() -> Self {
        Self {}
    }
}

impl SortAlgorithm for Shell {
    fn process(&mut self, arr: &mut SortArray) {
        //
    }
}
