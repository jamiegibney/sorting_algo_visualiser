use super::*;

#[derive(Debug)]
pub struct Insertion;

impl Insertion {
    pub const fn new() -> Self {
        Self
    }

    pub fn insert(arr: &mut SortArray, left: usize, right: usize) {
        for i in (left + 1)..=(right) {
            let tmp = arr.read(i);
            let mut j = i as isize - 1;

            while j >= 0 && j as usize >= left && arr.read(j as usize) > tmp {
                let arr_j = arr.read(j as usize);
                arr.write((j + 1) as usize, arr_j);
                j -= 1;
            }
            arr.write((j + 1) as usize, tmp);
        }
    }
}

impl SortProcessor for Insertion {
    fn process(&mut self, arr: &mut SortArray) {
        Self::insert(arr, 0, arr.len() - 1);
    }
}
