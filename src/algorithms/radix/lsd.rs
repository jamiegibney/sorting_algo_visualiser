use super::*;

#[derive(Debug)]
pub struct RadixLSD {
    base: usize,
    bins: Vec<Vec<usize>>,
}

impl RadixLSD {
    pub fn new(base: usize) -> Self {
        Self { base, bins: vec![vec![]; base] }
    }

    fn transcribe(arr: &mut SortArray, bins: &mut [Vec<usize>]) {
        let n = arr.len();

        let base = bins.len();
        let mut tmp = vec![0; n];
        let mut tmp_write = vec![false; n];

        let mut total = 0;
        for bin in bins {
            for &val in bin.iter() {
                tmp[total] = val;
                total += 1;
            }

            bin.clear();
        }

        for i in 0..n {
            let bin = i % base;
            let r_f32 = base as f32;
            let pos =
                (bin as f32 * (n as f32 / r_f32) + (i as f32 / r_f32)) as usize;

            if !tmp_write[pos] {
                arr.write(pos, tmp[pos]);
                tmp_write[pos] = true;
            }
        }

        for i in 0..n {
            if !tmp_write[i] {
                arr.write(i, tmp[i]);
            }
        }
    }
}

impl SortProcessor for RadixLSD {
    fn process(&mut self, arr: &mut SortArray) {
        let max_power = max_power(arr, self.base);

        for p in 0..=max_power {
            for i in 0..arr.len() {
                let arr_i = arr.read(i);
                let idx = get_digit(arr_i, p, self.base);
                self.bins[idx].push(arr_i);
            }

            Self::transcribe(arr, &mut self.bins);
        }

        self.bins.iter_mut().for_each(Vec::clear);
    }
}
