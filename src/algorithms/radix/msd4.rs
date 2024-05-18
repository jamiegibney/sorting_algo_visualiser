use super::*;

#[derive(Debug)]
pub struct RadixMSD4 {
    //
}

impl RadixMSD4 {
    pub fn new() -> Self {
        Self {}
    }
}

impl SortAlgorithm for RadixMSD4 {
    fn step(&mut self, slice: &mut SortArray) {
        todo!()
    }

    fn steps_per_second(&mut self) -> usize {
        SortingAlgorithm::RadixMSD4.steps()
    }

    fn finished(&self) -> bool {
        todo!()
    }

    fn reset(&mut self) {
        todo!();
    }
}
