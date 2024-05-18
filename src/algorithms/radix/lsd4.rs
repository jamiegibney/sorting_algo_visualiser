use super::*;

#[derive(Debug)]
pub struct RadixLSD4 {
    //
}

impl RadixLSD4 {
    pub fn new() -> Self {
        Self {}
    }
}

impl SortAlgorithm for RadixLSD4 {
    fn step(&mut self, slice: &mut SortArray) {
        todo!()
    }

    fn steps_per_second(&mut self) -> usize {
        SortingAlgorithm::RadixLSD4.steps()
    }

    fn finished(&self) -> bool {
        todo!()
    }

    fn reset(&mut self) {
        todo!();
    }
}
