use super::*;

#[derive(Debug)]
pub struct Pancake;

impl Pancake {
    pub const fn new() -> Self {
        Self
    }

    fn flip(arr: &mut SortArray, mut i: usize) {
        let mut start = 0;

        while start < i {
            arr.swap(start, i);
            start += 1;
            i -= 1;
        }
    }

    fn max_of(arr: &mut SortArray, len: usize) -> usize {
        let mut max_idx = 0;

        for i in 0..len {
            if arr.cmp(i, max_idx, Ordering::Greater) {
                max_idx = i;
            }
        }

        max_idx
    }
}

impl SortAlgorithm for Pancake {
    fn process(&mut self, arr: &mut SortArray) {
        let n = arr.len();

        for curr_size in (2..=n).rev() {
            let max_idx = Self::max_of(arr, curr_size);

            if max_idx != curr_size - 1 {
                Self::flip(arr, max_idx);
                Self::flip(arr, curr_size - 1);
            }
        }
    }
}
