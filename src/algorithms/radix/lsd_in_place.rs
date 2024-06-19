use super::*;

#[derive(Debug)]
pub struct RadixLSDInPlace {
    base: usize,
    bins: Vec<usize>,
}

impl RadixLSDInPlace {
    pub fn new(base: usize) -> Self {
        Self { base, bins: vec![0; base - 1] }
    }

    fn swap_to(arr: &mut SortArray, pos: usize, to: usize) {
        if to > pos {
            for i in pos..to {
                arr.swap(i, i + 1);
            }
        }
        else {
            for i in (to..pos).rev() {
                arr.swap(i, i - 1);
            }
        }
    }
}

impl SortProcessor for RadixLSDInPlace {
    fn process(&mut self, arr: &mut SortArray) {
        let mut pos;

        let max_power = max_power(arr, self.base);

        for p in 0..=max_power {
            pos = 0;
            self.bins.fill(arr.len() - 1);

            for _ in 0..arr.len() {
                let digit = get_digit(arr.read(pos), p, self.base);

                if digit == 0 {
                    pos += 1;
                }
                else {
                    Self::swap_to(arr, pos, self.bins[digit-1]);

                    for j in (1..digit).rev() {
                        self.bins[j - 1] -= 1;
                    }
                }
            }
        }
    }
}
