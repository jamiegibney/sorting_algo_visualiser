use super::*;

#[derive(Debug)]
pub struct Shell {}

impl Shell {
    pub const fn new() -> Self {
        Self {}
    }
}

impl SortProcessor for Shell {
    fn process(&mut self, arr: &mut SortArray) {
        let n = arr.len();
        let mut gap = n / 2;

        while gap > 0 {
            for i in gap..n {
                let tmp = arr.read(i);

                let mut j = i;

                while j >= gap && arr.read(j - gap) > tmp {
                    let val = arr.read(j - gap);
                    arr.write(j, val);
                    j -= gap;
                }

                arr.write(j, tmp);
            }

            gap /= 2;
        }
    }
}
