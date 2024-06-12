use super::*;

#[derive(Debug)]
pub struct Pigeonhole {
    holes: Vec<Vec<usize>>,
}

impl Pigeonhole {
    pub const fn new() -> Self {
        Self { holes: vec![] }
    }
}

impl SortProcessor for Pigeonhole {
    fn process(&mut self, arr: &mut SortArray) {
        let mut min_idx = 0;
        let mut max_idx = 0;
        let n = arr.len();

        for i in 0..n {
            if arr.cmp(i, min_idx, Ordering::Less) {
                min_idx = i;
            }
            if arr.cmp(i, max_idx, Ordering::Greater) {
                max_idx = i;
            }
        }

        let range = arr.read(max_idx) - arr.read(min_idx) + 1;

        self.holes.resize(range, vec![]);

        for i in 0..n {
            let arr_i = arr.read(i);
            let min = arr.read(min_idx);
            self.holes[arr_i - min].push(arr_i);
        }

        let mut idx = 0;

        for i in 0..range {
            if idx >= n {
                break;
            }

            for &val in &self.holes[i] {
                arr.write(idx, val);
                idx += 1;
            }
        }

        self.holes.clear();
    }
}
