use super::*;

#[derive(Debug)]
pub struct Comb;

impl Comb {
    pub const fn new() -> Self {
        Self
    }

    fn next_gap(gap: usize) -> usize {
        (gap * 10 / 13).max(1)
    }
}

impl SortAlgorithm for Comb {
    fn process(&mut self, arr: &mut SortArray) {
        let n = arr.len();
        let mut gap = n;
        let mut swapped = true;

        while gap != 1 || swapped {
            gap = Self::next_gap(gap);
            swapped = false;

            for i in 0..(n - gap) {
                if arr.cmp(i, i + gap, Ordering::Greater) {
                    arr.swap(i, i + gap);
                    swapped = true;
                }
            }
        }
    }
}
