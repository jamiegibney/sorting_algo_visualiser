use super::*;

#[derive(Debug)]
pub struct Stooge;

impl Stooge {
    pub const fn new() -> Self {
        Self
    }

    fn sort(arr: &mut SortArray, l: usize, h: usize) {
        if l >= h {
            return;
        }

        if arr.cmp(l, h, Ordering::Greater) {
            arr.swap(l, h);
        }

        if h - l + 1 > 2 {
            let t = (h - l + 1) / 3;

            Self::sort(arr, l, h - t);
            Self::sort(arr, l + t, h);
            Self::sort(arr, l, h - t);
        }
    }
}

impl SortProcessor for Stooge {
    fn process(&mut self, arr: &mut SortArray) {
        Self::sort(arr, 0, arr.len() - 1);
    }
}
