use super::*;

#[derive(Debug)]
pub struct Timsort {
    merge: Merge,
}

impl Timsort {
    const RUN: usize = 32;

    pub fn new() -> Self {
        Self { merge: Merge::new() }
    }
}

impl SortProcessor for Timsort {
    fn process(&mut self, arr: &mut SortArray) {
        let n = arr.len();

        for i in (0..n).step_by(Self::RUN) {
            let right = usize::min(i + Self::RUN - 1, n - 1);
            super::Insertion::insert(arr, i, right);
        }

        let mut size = Self::RUN;
        while size < n {
            for left in (0..n).step_by(2 * size) {
                let mid = left + size - 1;
                let right = usize::min(left + 2 * size - 1, n - 1);

                if mid < right {
                    self.merge.sort(arr, left, mid, right);
                }
            }

            size *= 2;
        }
    }
}
