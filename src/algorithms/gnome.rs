use super::*;

#[derive(Debug)]
pub struct Gnome;

impl Gnome {
    pub const fn new() -> Self {
        Self
    }
}

impl SortProcessor for Gnome {
    fn process(&mut self, arr: &mut SortArray) {
        let n = arr.len();
        let mut i = 0;

        while i < n {
            if i == 0 {
                i = 1;
            }

            if arr.cmp(i, i - 1, Less) {
                arr.swap(i, i - 1);
                i -= 1;
            }
            else {
                i += 1;
            }
        }
    }
}
