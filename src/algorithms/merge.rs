use super::*;

#[derive(Debug)]
pub struct Merge {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl Merge {
    pub fn new() -> Self {
        Self { left: vec![], right: vec![] }
    }

    fn merge(&mut self, arr: &mut SortArray, begin: usize, end: usize) {
        if begin >= end {
            return;
        }

        let mid = begin + (end - begin) / 2;

        self.merge(arr, begin, mid);
        self.merge(arr, mid + 1, end);
        self.sort(arr, begin, mid, end);
    }

    fn sort(
        &mut self,
        arr: &mut SortArray,
        left: usize,
        mid: usize,
        right: usize,
    ) {
        let left_len = mid - left + 1;
        let right_len = right - mid;

        self.left = vec![0; left_len];
        for i in 0..left_len {
            self.left[i] = arr.read(left + i);
        }
        self.right = vec![0; right_len];
        for i in 0..right_len {
            self.right[i] = arr.read(mid + i + 1);
        }

        let (mut l, mut r) = (0, 0);
        let mut merge = left;

        while l < left_len && r < right_len {
            if self.left[l] <= self.right[r] {
                arr.write(merge, self.left[l]);
                l += 1;
            }
            else {
                arr.write(merge, self.right[r]);
                r += 1;
            }

            merge += 1;
        }

        while l < left_len {
            arr.write(merge, self.left[l]);
            l += 1;
            merge += 1;
        }

        while r < right_len {
            arr.write(merge, self.right[r]);
            r += 1;
            merge += 1;
        }

        self.left.clear();
        self.right.clear();
    }
}

impl SortAlgorithm for Merge {
    fn process(&mut self, arr: &mut SortArray) {
        self.merge(arr, 0, arr.len() - 1);
    }
}
