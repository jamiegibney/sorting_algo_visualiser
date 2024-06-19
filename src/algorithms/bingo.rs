use super::*;

#[derive(Debug)]
pub struct Bingo;

impl Bingo {
    pub const fn new() -> Self {
        Self
    }

    fn min_max(arr: &mut SortArray) -> (usize, usize) {
        let mut min_idx = 0;
        let mut min = 0;
        let mut max_idx = 0;
        let mut max = 0;

        for i in 0..arr.len() {
            if arr.cmp(i, min_idx, Less) {
                min_idx = i;
                min = arr.read(min_idx);
            }
            if arr.cmp(i, max_idx, Greater) {
                max_idx = i;
                max = arr.read(max_idx);
            }
        }

        (min, max)
    }
}

impl SortProcessor for Bingo {
    fn process(&mut self, arr: &mut SortArray) {
        let (mut bingo, mut next_bingo) = Self::min_max(arr);
        let max = next_bingo;
        let mut next_pos = 0;

        while bingo < next_bingo {
            let start = next_pos;

            for i in start..arr.len() {
                if arr.read(i) == bingo {
                    arr.swap(i, next_pos);
                    next_pos += 1;
                }
                else if arr.read(i) < next_bingo {
                    next_bingo = arr.read(i);
                }
            }

            bingo = next_bingo;
            next_bingo = max;
        }
    }
}
