use super::*;

#[derive(Debug)]
pub struct RadixLSDInPlace {
    base: usize,
    analysis: Vec<usize>,
    bins: Vec<usize>,
}

impl RadixLSDInPlace {
    pub fn new(base: usize) -> Self {
        Self { base, analysis: vec![], bins: vec![0; base - 1] }
    }
}

impl SortAlgorithm for RadixLSDInPlace {
    fn process(&mut self, arr: &mut SortArray) {
        unimplemented!();
        let mut pos = 0;

        let max_power = analyze(arr, &mut self.analysis, self.base);

        for p in 0..=max_power {
            pos = 0;
            self.bins.fill(arr.len() - 1);

            for i in 0..arr.len() {
                let digit = get_digit(arr.read(pos), p, self.base);

                if digit == 0 {
                    pos += 1;
                    self.analysis[pos] = 0;
                }
                else {
                    for j in 0..self.bins.len() {
                        // self.analysis
                    }
                }
            }
        }
    }
}
