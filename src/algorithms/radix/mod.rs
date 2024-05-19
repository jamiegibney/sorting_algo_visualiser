use super::*;

mod in_place_lsd10;
mod in_place_lsd4;
mod lsd10;
mod lsd4;
mod msd10;
mod msd4;

pub use in_place_lsd10::InPlaceRadixLSD10;
pub use in_place_lsd4::InPlaceRadixLSD4;
pub use lsd10::RadixLSD10;
pub use lsd4::RadixLSD4;
pub use msd10::RadixMSD10;
pub use msd4::RadixMSD4;

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct RadixBase {
    base: usize,
    msd: bool,

    finished: bool,

    exp: usize,

    /// has the algorithm initialized?
    has_initialized: bool,
    /// Has the algorithm found the max value?
    max_value: Option<usize>,
    /// Has the algorithm stored each base occurrence in the count bins?
    stored_occurrences: bool,
    count_idx: usize,
    /// Has the algorithm shifted the count bins?
    shifted_bins: bool,
    built_output: bool,
    out_idx: usize,

    copied: bool,
    copy_idx: usize,

    max: usize,
    idx_max: usize,

    count: Vec<usize>,
    aux_arr: Vec<usize>,
}

impl RadixBase {
    pub fn lsd_with_base(base: usize) -> Self {
        Self {
            base,
            msd: false,

            finished: false,

            exp: 1,

            has_initialized: false,
            max_value: None,
            stored_occurrences: false,
            count_idx: 0,
            shifted_bins: false,
            built_output: false,
            out_idx: 0,

            copied: false,
            copy_idx: 0,

            max: 0,
            idx_max: 0,

            count: vec![0; base],
            aux_arr: vec![],
        }
    }

    pub fn msd_with_base(base: usize) -> Self {
        Self {
            base,
            msd: true,

            finished: false,

            exp: 1,

            has_initialized: false,
            max_value: None,
            stored_occurrences: false,
            count_idx: 0,
            shifted_bins: false,
            built_output: false,
            out_idx: 0,

            copied: false,
            copy_idx: 0,

            max: 0,
            idx_max: 0,

            count: vec![0; base],
            aux_arr: vec![],
        }
    }

    pub fn step(&mut self, arr: &mut SortArray) {
        if !self.has_initialized {
            self.aux_arr.resize(arr.len(), 0);
            self.aux_arr.copy_from_slice(arr.as_slice());
            self.has_initialized = true;
            self.out_idx = arr.len() - 1;
        }

        if let Some(max) = self.max_value {
            if max / self.exp == 0 {
                self.finished = true;
                return;
            }
            // each outer loop iter
            else if self.shifted_bins
                && self.stored_occurrences
                && self.built_output
                && self.copied
            {
                self.exp *= self.base;

                self.stored_occurrences = false;
                self.count.fill(0);
                self.count_idx = 0;

                self.shifted_bins = false;

                self.built_output = false;
                self.aux_arr.fill(0);
                self.out_idx = arr.len() - 1;

                self.copied = false;
                self.copy_idx = 0;

                self.aux_arr.copy_from_slice(arr.as_slice());
            }
        }
        else {
            // find max
            let a = arr.read(self.idx_max);
            if a > self.max {
                self.max = a;
            }

            self.idx_max += 1;

            if self.idx_max == arr.len() {
                self.max_value = Some(self.max);
            }
        }

        // store in count bins
        if !self.stored_occurrences {
            let idx = (arr.read(self.count_idx) / self.exp) % self.base;
            self.count[idx] += 1;
            self.count_idx += 1;

            if self.count_idx == arr.len() {
                self.stored_occurrences = true;
            }

            return;
        }

        // shift bins over
        if !self.shifted_bins {
            for i in 1..self.base {
                self.count[i] += self.count[i - 1];
            }

            self.shifted_bins = true;
        }

        // construct output for this exp
        if !self.built_output {
            let arr_i = arr.read(self.out_idx);
            let idx = (arr_i / self.exp) % self.base;

            self.aux_arr[self.count[idx] - 1] = arr_i;
            self.count[idx] -= 1;

            if self.out_idx == 0 {
                self.built_output = true;
                return;
            }

            self.out_idx -= 1;
            return;
        }

        // copy to output
        if !self.copied {
            arr.write(self.copy_idx, self.aux_arr[self.copy_idx]);

            self.copy_idx += 1;

            if self.copy_idx == arr.len() {
                self.copied = true;
            }
        }
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn reset(&mut self) {
        self.finished = false;

        self.exp = 1;

        self.has_initialized = false;
        self.max_value = None;
        self.stored_occurrences = false;
        self.count_idx = 0;
        self.shifted_bins = false;
        self.built_output = false;
        self.out_idx = 0;

        self.copied = false;
        self.copy_idx = 0;

        self.max = 0;
        self.idx_max = 0;

        self.count.fill(0);
        self.aux_arr.clear();
    }
}

impl SortAlgorithm for RadixBase {
    fn step(&mut self, arr: &mut SortArray) {
        self.step(arr);
    }

    fn steps_per_second(&mut self) -> usize {
        300
    }

    fn finished(&self) -> bool {
        self.finished
    }

    fn reset(&mut self) {
        self.reset();
    }
}
