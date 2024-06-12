use super::*;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
pub struct Insertion {
    i: usize,
    j: usize,
    key: usize,
    finished: bool,
    inner: bool,
    has_initialized: bool,
    write_to_0: bool,
}

impl Insertion {
    pub const fn new() -> Self {
        Self {
            i: 1,
            j: 0,
            key: 0,
            finished: false,
            inner: true,
            has_initialized: false,
            write_to_0: false,
        }
    }
}

impl SortProcessor for Insertion {
    fn process(&mut self, arr: &mut SortArray) {
        let mut key = 0;
        let mut j = 0;

        for i in 1..arr.len() {
            key = arr.read(i);
            j = i - 1;

            let mut write_to_0 = false;

            while arr.read(j) > key {
                let arr_j = arr.read(j);
                arr.write(j + 1, arr_j);

                if j == 0 {
                    write_to_0 = true;
                    break;
                }

                j -= 1;
            }

            arr.write(if write_to_0 { 0 } else { j + 1 }, key);
        }
    }
}
