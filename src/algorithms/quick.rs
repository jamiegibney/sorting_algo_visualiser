use super::*;

#[derive(Debug)]
pub struct QuickSort;

impl QuickSort {
    pub fn new() -> Self {
        Self
    }

    fn partition(arr: &mut SortArray, low: isize, high: isize) -> isize {
        let pivot = arr.read(high as usize);
        let mut i = low - 1;

        for j in low..high {
            let j = j as usize;
            if arr.read(j) < pivot {
                i += 1;
                arr.swap(i as usize, j);
            }
        }

        arr.swap((i + 1) as usize, high as usize);

        i + 1
    }

    fn sort(arr: &mut SortArray, low: isize, high: isize) {
        if low < high {
            let part = Self::partition(arr, low, high);

            Self::sort(arr, low, part - 1);
            Self::sort(arr, part + 1, high);
        }
    }
}

impl SortAlgorithm for QuickSort {
    fn process(&mut self, arr: &mut SortArray) {
        Self::sort(arr, 0, (arr.len() - 1) as isize);
    }
}
