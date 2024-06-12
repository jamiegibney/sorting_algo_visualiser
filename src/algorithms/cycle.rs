use super::*;

#[derive(Debug)]
pub struct Cycle;

impl Cycle {
    pub const fn new() -> Self {
        Self
    }
}

impl SortAlgorithm for Cycle {
    fn process(&mut self, arr: &mut SortArray) {
        let n = arr.len();

        for start in 0..(n - 1) {
            let item = arr.read(start);

            let mut pos = start;

            for i in (start + 1)..n {
                if arr.cmp(i, start, Ordering::Less) {
                    pos += 1;
                }
            }

            if pos == start {
                continue;
            }

            while arr.cmp(start, pos, Ordering::Equal) {
                pos += 1;
            }

            if pos != start {
                arr.swap(start, pos);
            }

            while pos != start {
                pos = start;

                for i in (start + 1)..n {
                    if arr.cmp(i, start, Ordering::Less) {
                        pos += 1;
                    }
                }

                if pos == start {
                    break;
                }

                while arr.cmp(start, pos, Ordering::Equal) {
                    pos += 1;
                }

                if !arr.cmp(start, pos, Ordering::Equal) {
                    arr.swap(start, pos);
                }
            }
        }
    }
}
