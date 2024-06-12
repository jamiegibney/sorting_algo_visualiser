use super::*;

#[derive(Debug)]
pub struct Timsort;

impl Timsort {
    pub const fn new() -> Self {
        Self
    }
}

impl SortProcessor for Timsort {
    fn process(&mut self, arr: &mut SortArray) {
        unimplemented!();
    }
}
