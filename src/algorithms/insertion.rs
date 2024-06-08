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

impl SortAlgorithm for Insertion {
    // fn step(&mut self, arr: &mut SortArray) {
    //     if self.finished {
    //         return;
    //     }
    //
    //     if !self.has_initialized {
    //         self.key = arr.read(self.i);
    //         self.has_initialized = true;
    //     }
    //
    //     let len = arr.len();
    //
    //     if self.inner {
    //         let arr_j = arr.read(self.j);
    //
    //         // this registers the comparison, but it cannot be used below
    //         _ = arr.cmp(self.j, self.i, Ord::Greater);
    //
    //         if arr_j > self.key {
    //             arr.write(self.j + 1, arr_j);
    //
    //             if self.j == 0 {
    //                 self.inner = false;
    //                 self.write_to_0 = true;
    //             }
    //             else {
    //                 self.j -= 1;
    //             }
    //         }
    //         else {
    //             self.inner = false;
    //         }
    //     }
    //     else if self.i < len {
    //         arr.write(if self.write_to_0 { 0 } else { self.j + 1 }, self.key);
    //         self.write_to_0 = false;
    //
    //         self.i += 1;
    //
    //         if self.i == len {
    //             self.finished = true;
    //             return;
    //         }
    //
    //         self.key = arr.read(self.i);
    //         self.j = self.i - 1;
    //
    //         self.inner = true;
    //     }
    // }
    //
    // fn steps_per_second(&mut self) -> usize {
    //     SortingAlgorithm::Insertion.speed()
    // }
    //
    // fn finished(&self) -> bool {
    //     self.finished
    // }
    //
    // fn reset(&mut self) {
    //     self.i = 1;
    //     self.j = 0;
    //     self.key = 0;
    //     self.inner = true;
    //     self.finished = false;
    //     self.has_initialized = false;
    // }
    fn process(&mut self, arr: &mut SortArray) {
        todo!();
    }
}
