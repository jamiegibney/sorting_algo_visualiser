use super::*;

#[derive(Debug)]
pub struct RadixMSD10 {
    //
}

impl SortAlgorithm for RadixMSD10 {
    fn new() -> Self {
        Self {
            //
        }
    }

    fn step(&mut self, slice: &mut [usize]) {
        todo!();
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

