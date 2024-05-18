use super::*;

#[derive(Debug)]
pub struct RadixMSD10 {
    //
}

impl RadixMSD10 {
    pub fn new() -> Self {
        Self {}
    }
}

impl SortAlgorithm for RadixMSD10 {
    fn step(&mut self, slice: &mut SortArray) {
        todo!()
    }

    fn steps_per_second(&mut self) -> usize {
        SortingAlgorithm::RadixMSD10.steps()
    }

    fn finished(&self) -> bool {
        todo!()
    }

    fn reset(&mut self) {
        todo!();
    }
}

