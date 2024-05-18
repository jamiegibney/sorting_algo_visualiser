use super::*;

#[derive(Debug)]
pub struct InPlaceRadixLSD10 {
    //
}

impl InPlaceRadixLSD10 {
    pub fn new() -> Self {
        Self {}
    }
}

impl SortAlgorithm for InPlaceRadixLSD10 {
    fn step(&mut self, slice: &mut SortArray) {
        todo!()
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
