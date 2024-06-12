use super::*;

#[derive(Debug)]
pub struct RadixLSD {
    base: usize,
    bins: Vec<usize>,
    analysis: Vec<usize>,
}

impl RadixLSD {
    pub fn new(base: usize) -> Self {
        Self { base, bins: vec![0; base], analysis: vec![] }
    }
}

impl SortAlgorithm for RadixLSD {
    fn process(&mut self, arr: &mut SortArray) {
        unimplemented!();
        let max_power = analyze(arr, &mut self.analysis, self.base);

        for p in 0..=max_power {
            for i in 0..arr.len() {
                let idx = get_digit(arr.read(i), p, self.base);
                self.bins[idx] += 1;
            }
        }
    }
}
