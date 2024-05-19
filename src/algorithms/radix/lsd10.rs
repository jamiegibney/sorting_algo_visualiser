use super::*;

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)] // hush now clippy
pub struct RadixLSD10 {
    base: RadixBase,
}

impl RadixLSD10 {
    pub fn new() -> Self {
        Self { base: RadixBase::lsd_with_base(10) }
    }
}

impl SortAlgorithm for RadixLSD10 {
    fn step(&mut self, arr: &mut SortArray) {
        self.base.step(arr);
    }

    fn steps_per_second(&mut self) -> usize {
        SortingAlgorithm::RadixLSD10.steps()
    }

    fn finished(&self) -> bool {
        self.base.finished()
    }

    fn reset(&mut self) {
        self.base.reset();
    }
}
