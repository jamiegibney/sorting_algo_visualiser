use super::*;

#[derive(Debug)]
pub struct Counting {
    counting_arr: Vec<usize>,
    output_arr: Vec<usize>,
}

impl Counting {
    pub fn new() -> Self {
        Self { counting_arr: vec![], output_arr: vec![] }
    }

    pub fn max(arr: &mut SortArray) -> usize {
        let mut max = 0;

        for i in 0..arr.len() {
            if arr.cmp(i, max, Ordering::Greater) {
                max = i;
            }
        }

        arr.read(max)
    }
}

impl SortProcessor for Counting {
    fn process(&mut self, arr: &mut SortArray) {
        let n = arr.len();
        let max = Self::max(arr);

        self.counting_arr.resize(max + 1, 0);
        self.output_arr.resize(n, 0);

        for i in 0..n {
            self.counting_arr[arr.read(i)] += 1;
        }

        for i in 1..=max {
            self.counting_arr[i] += self.counting_arr[i - 1];
        }

        for i in (0..n).rev() {
            let arr_i = arr.read(i);
            self.output_arr[self.counting_arr[arr_i] - 1] = arr_i;

            self.counting_arr[arr_i] -= 1;
        }

        for i in 0..n {
            arr.write(i, self.output_arr[i]);
        }

        self.output_arr.clear();
        self.counting_arr.clear();
    }
}
