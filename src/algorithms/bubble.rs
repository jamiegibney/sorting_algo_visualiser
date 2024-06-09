use super::*;

#[derive(Debug)]
pub struct Bubble;

impl Bubble {
    pub const fn new() -> Self {
        Self
    }
}

impl SortAlgorithm for Bubble {
    fn process(&mut self, arr: &mut SortArray) {
        let n = arr.len();
        let mut any_swapped = false;

        for i in 0..(n - 1) {
            any_swapped = false;

            for j in 0..(n - i - 1) {
                if arr.cmp(j, j + 1, Ordering::Greater) {
                    arr.swap(j, j + 1);
                    any_swapped = true;
                }
            }

            if !any_swapped {
                break;
            }
        }
    }
}
