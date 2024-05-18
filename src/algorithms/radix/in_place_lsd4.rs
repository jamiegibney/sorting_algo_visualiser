use super::*;

#[derive(Debug)]
pub struct InPlaceRadixLSD4 {
    //
}

impl InPlaceRadixLSD4 {
    pub fn new() -> Self {
        Self {}
    }
}

impl SortAlgorithm for InPlaceRadixLSD4 {
    fn step(&mut self, slice: &mut [usize]) -> Option<AlgorithmStep> {
        None
    }

    fn steps_per_second(&mut self) -> usize {
        SortingAlgorithm::InPlaceRadixLSD4.steps()
    }

    fn finished(&self) -> bool {
        todo!()
    }

    fn reset(&mut self) {
        todo!();
    }
}
