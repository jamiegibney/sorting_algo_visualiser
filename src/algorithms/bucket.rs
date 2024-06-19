use super::*;

#[derive(Debug)]
pub struct Bucket {
    buckets: Vec<Vec<usize>>,
}

impl Bucket {
    pub fn new() -> Self {
        Self { buckets: Vec::new() }
    }

    fn insert(&mut self, bucket_idx: usize) {
        let bucket = &mut self.buckets[bucket_idx];

        for i in 1..bucket.len() {
            let key = bucket[i];
            let mut j = i as isize - 1;

            while j >= 0 && bucket[j as usize] > key {
                bucket[(j + 1) as usize] = bucket[j as usize];
                j -= 1;
            }
            bucket[(j + 1) as usize] = key;
        }
    }
}

impl SortProcessor for Bucket {
    #[allow(unused, unreachable_code)]
    fn process(&mut self, arr: &mut SortArray) {
        unimplemented!("this sort is a bit silly for this array, so is left out for now");
        let n = arr.len();
        self.buckets.resize(n, vec![]);

        for i in 0..n {
            let arr_i = arr.read(i);
            let bi = n * arr_i;
            self.buckets[bi].push(arr_i);
        }

        for i in 0..n {
            self.insert(i);
        }

        let mut idx = 0;

        for i in 0..n {
            for j in 0..self.buckets[i].len() {
                arr.write(idx, self.buckets[i][j]);
                idx += 1;
            }
        }
    }
}
