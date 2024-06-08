use super::*;

#[derive(Debug)]
pub struct RadixMSD4 {
    base: RadixBase,
}

impl RadixMSD4 {
    pub fn new() -> Self {
        Self { base: RadixBase::msd_with_base(4) }
    }
}

impl SortAlgorithm for RadixMSD4 {
    // fn step(&mut self, arr: &mut SortArray) {
    //     self.base.step(arr);
    // }
    //
    // fn steps_per_second(&mut self) -> usize {
    //     SortingAlgorithm::RadixMSD4.speed()
    // }
    //
    // fn finished(&self) -> bool {
    //     self.base.finished()
    // }
    //
    // fn reset(&mut self) {
    //     self.base.reset();
    // }
    fn process(&mut self, arr: &mut SortArray) {
        todo!();
    }
}
