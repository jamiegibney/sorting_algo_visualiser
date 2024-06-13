use super::*;
use nannou::rand::random_range;

/// A bogosort.
#[derive(Debug)]
pub struct Bogo;

impl Bogo {
    pub const fn new() -> Self {
        Self
    }

    fn is_sorted(arr: &mut SortArray) -> bool {
        for i in 0..(arr.len() - 1) {
            if arr.cmp(i, i + 1, Greater) {
                return false;
            }
        }

        true
    }
}

impl SortProcessor for Bogo {
    fn process(&mut self, arr: &mut SortArray) {
        let len = arr.len();

        while !Self::is_sorted(arr) {
            for i in 0..len {
                let rand = random_range(0, len);
                arr.swap(i, rand);
            }
        }
    }
}
