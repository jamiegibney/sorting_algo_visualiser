use super::*;

/// A selection sort.
#[derive(Debug)]
pub struct Selection;

impl Selection {
    pub const fn new() -> Self {
        Self
    }
}

impl SortProcessor for Selection {
    fn process(&mut self, arr: &mut SortArray) {
        let n = arr.len();
        let mut min_idx;

        for i in 0..(n - 1) {
            min_idx = i;

            for j in (i + 1)..n {
                if arr.cmp(j, min_idx, Less) {
                    min_idx = j;
                }
            }

            if min_idx != i {
                arr.swap(min_idx, i);
            }
        }

    }
}
