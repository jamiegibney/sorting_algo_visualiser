use super::*;

#[derive(Debug)]
pub struct InPlaceRadixLSD10 {
    //
}

impl SortAlgorithm for InPlaceRadixLSD10 {
    fn new() -> Self {
        Self {
            //
        }
    }

    fn step(&mut self, slice: &mut [usize]) -> Option<AlgorithmStep> {
        None
    }

    fn steps_per_second(&mut self) -> usize {
        SortingAlgorithm::InPlaceRadixLSD10.steps()
    }

    fn finished(&self) -> bool {
        todo!()
    }

    fn reset(&mut self) {
        todo!();
    }
}
