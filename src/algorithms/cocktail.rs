use super::*;

#[derive(Debug)]
pub struct Cocktail {}

impl Cocktail {
    pub fn new() -> Self {
        Self {}
    }
}

impl SortProcessor for Cocktail {
    fn process(&mut self, arr: &mut SortArray) {
        let n = arr.len();

        let mut swapped = true;
        let mut start = 0;
        let mut end = n - 1;

        while swapped {
            swapped = false;

            for i in start..end {
                if arr.cmp(i, i + 1, Ordering::Greater) {
                    arr.swap(i, i + 1);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }

            swapped = false;
            end -= 1;

            for i in (start..end).rev() {
                if arr.cmp(i, i + 1, Ordering::Greater) {
                    arr.swap(i, i + 1);
                    swapped = true;
                }
            }

            start += 1;
        }
    }
}
