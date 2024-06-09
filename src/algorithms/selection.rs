use super::*;

/// A selection sort.
#[derive(Debug)]
pub struct Selection {
    write_idx: usize,
    cmp_idx: usize,
    min_idx: usize,

    finished: bool,
}

impl Selection {
    pub const fn new() -> Self {
        Self {
            write_idx: 0,
            cmp_idx: 1,
            min_idx: 0,

            finished: false,
        }
    }
}

impl SortAlgorithm for Selection {
    fn process(&mut self, arr: &mut SortArray) {
        let n = arr.len();
        let mut min_idx = 0;

        for i in 0..(n - 1) {
            min_idx = i;

            for j in (i + 1)..n {
                if arr.cmp(j, min_idx, Ordering::Less) {
                    min_idx = j;
                }
            }

            if min_idx != i {
                arr.swap(min_idx, i);
            }
        }

    }

    // fn step(&mut self, arr: &mut SortArray) {
    //     // avoids index out of bounds error
    //     if self.write_idx == arr.len() - 1 {
    //         self.finished = true;
    //         return;
    //     }
    //
    //     // if we've finished comparing
    //     if self.cmp_idx == arr.len() {
    //         // swap elements
    //         if self.min_idx != self.write_idx {
    //             arr.swap(self.min_idx, self.write_idx);
    //         }
    //
    //         // increment write pos
    //         self.write_idx += 1;
    //         self.cmp_idx = self.write_idx + 1;
    //         self.min_idx = self.write_idx;
    //         return;
    //     }
    //     else if arr.cmp(self.cmp_idx, self.min_idx, Ord::Less) {
    //         self.min_idx = self.cmp_idx;
    //     }
    //
    //     self.cmp_idx += 1;
    // }
    //
    // fn steps_per_second(&mut self) -> usize {
    //     SortingAlgorithm::Selection.speed()
    // }
    //
    // fn finished(&self) -> bool {
    //     self.finished
    // }
    //
    // fn reset(&mut self) {
    //     self.write_idx = 0;
    //     self.cmp_idx = 1;
    //     self.min_idx = 0;
    //     self.finished = false;
    // }
}
