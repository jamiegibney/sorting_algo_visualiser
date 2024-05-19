use super::*;

#[derive(Debug)]
pub struct RadixMSD10 {
    base: RadixBase,
}

impl RadixMSD10 {
    pub fn new() -> Self {
        Self { base: RadixBase::msd_with_base(10) }
    }
}

impl SortAlgorithm for RadixMSD10 {
    fn step(&mut self, arr: &mut SortArray) {
        self.base.step(arr);
    }

    fn steps_per_second(&mut self) -> usize {
        SortingAlgorithm::RadixMSD10.steps()
    }

    fn finished(&self) -> bool {
        self.base.finished()
    }

    fn reset(&mut self) {
        self.base.reset();
    }
}
