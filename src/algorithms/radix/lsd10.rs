use super::*;

#[derive(Debug)]
pub struct RadixLSD10 {
    //
}

impl SortAlgorithm for RadixLSD10 {
    fn new() -> Self {
        Self {
            //
        }
    }

    fn step(&mut self, slice: &mut [usize]) -> Option<AlgorithmStep> {
        None
    }

    fn steps_per_second(&mut self) -> usize {
        SortingAlgorithm::RadixLSD10.steps()
    }

    fn finished(&self) -> bool {
        todo!()
    }

    fn reset(&mut self) {
        todo!();
    }
}

