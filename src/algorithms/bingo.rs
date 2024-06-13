use super::*;

#[derive(Debug)]
pub struct Bingo;

impl Bingo {
    pub const fn new() -> Self {
        Self
    }
}

impl SortProcessor for Bingo {
    fn process(&mut self, arr: &mut SortArray) {
        unimplemented!();
    }
}
