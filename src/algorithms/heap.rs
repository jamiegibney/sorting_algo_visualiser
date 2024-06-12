use super::*;

#[derive(Debug)]
pub struct Heap;

impl Heap {
    fn heapify(arr: &mut SortArray, n: usize, i: usize) {
        let mut max = i;

        let l = 2 * i + 1;
        let r = 2 * i + 2;

        if l < n && arr.cmp(l, max, Ordering::Greater) {
            max = l;
        }
        if r < n && arr.cmp(r, max, Ordering::Greater) {
            max = r;
        }

        if max != i {
            arr.swap(i, max);
            Self::heapify(arr, n, max);
        }
    }
}

impl SortProcessor for Heap {
    fn process(&mut self, arr: &mut SortArray) {
        let len = arr.len();

        for i in (0..len / 2).rev() {
            Self::heapify(arr, len, i);
        }

        for i in (1..len).rev() {
            arr.swap(0, i);
            Self::heapify(arr, i, 0);
        }
    }
}
