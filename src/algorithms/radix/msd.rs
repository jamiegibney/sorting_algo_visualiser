use super::*;

#[derive(Debug)]
pub struct RadixMSD {
    base: usize,
}

impl RadixMSD {
    pub const fn new(base: usize) -> Self {
        Self { base }
    }

    fn transcribe(arr: &mut SortArray, bins: &[Vec<usize>], min: usize) {
        let total = bins.iter().fold(0, |acc, b| acc + b.len());

        let mut tmp = 0;
        for bin in bins.iter().rev() {
            for &val in bin.iter().rev() {
                arr.write(total + min - tmp - 1, val);
                tmp += 1;
            }
        }
    }

    fn radix(
        &mut self,
        arr: &mut SortArray,
        min: usize,
        max: usize,
        pow: usize,
    ) {
        if min >= max {
            return;
        }

        let mut bins = vec![vec![]; self.base];

        for i in min..max {
            let arr_i = arr.read(i);
            bins[get_digit(arr_i, pow, self.base)].push(arr_i);
        }

        Self::transcribe(arr, &bins, min);

        let mut sum = 0;
        for bin in &mut bins {
            let size = bin.len();

            if pow > 0 {
                self.radix(arr, sum + min, sum + min + size, pow - 1);
            }
            sum += size;

            bin.clear();
        }
    }
}

impl SortProcessor for RadixMSD {
    fn process(&mut self, arr: &mut SortArray) {
        let max_power = max_power(arr, self.base);
        self.radix(arr, 0, arr.len(), max_power);
    }
}
