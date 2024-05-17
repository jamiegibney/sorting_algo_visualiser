use super::*;

#[derive(Debug)]
pub struct RadixMSD4 {
    //
}

impl SortAlgorithm for RadixMSD4 {
    fn new() -> Self {
        Self {
            //
        }
    }

    fn step(&mut self, slice: &mut [usize]) -> Option<AlgorithmStep> {
        None
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
