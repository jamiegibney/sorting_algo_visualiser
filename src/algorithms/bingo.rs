use super::*;

#[derive(Debug)]
pub struct Bingo;

impl Bingo {
    pub const fn new() -> Self {
        Self
    }

    fn min_max_indices(arr: &mut SortArray) -> (usize, usize) {
        let mut min_idx = 0;
        let mut max_idx = 0;
        for i in 0..arr.len() {
            if arr.cmp(i, min_idx, Less) {
                min_idx = i;
            }
            if arr.cmp(i, max_idx, Greater) {
                max_idx = i;
            }
        }

        (min_idx, max_idx)
    }
}

impl SortProcessor for Bingo {
    fn process(&mut self, arr: &mut SortArray) {
        unimplemented!();
        let (mut bingo, mut next_bingo) = Self::min_max_indices(arr);
        let mut next_pos = 0;
        let max_idx = next_bingo;

        while arr.cmp(bingo, next_bingo, Less) {
            let mut start = next_pos;

            for i in start..arr.len() {
                if arr.cmp(i, bingo, Equal) {
                    arr.swap(i, next_pos);
                    next_pos += 1;
                }
                else if arr.cmp(i, next_bingo, Less) {
                    next_bingo = i;
                }
            }

            bingo = next_bingo;
            next_bingo = max_idx;
        }
    }
}
