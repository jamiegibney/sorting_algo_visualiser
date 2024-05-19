use super::*;

#[derive(Debug)]
pub struct Bubble {
    i: usize,
    j: usize,
    swapped: bool,
    finished: bool,
}

impl Bubble {
    pub const fn new() -> Self {
        Self { i: 0, j: 0, swapped: false, finished: false }
    }
}

impl SortAlgorithm for Bubble {
    fn step(&mut self, arr: &mut SortArray) {
        if self.finished {
            return;
        }

        let len = arr.len();
        let num_ops = len - self.i - 1;

        if self.j < len - self.i - 1 {
            if arr.cmp(self.j, self.j + 1, Ord::Greater) {
                arr.swap(self.j, self.j + 1);
                self.swapped = true;
            }

            self.j += 1;
        }
        else {
            if !self.swapped {
                self.finished = true;
                return;
            }

            self.i += 1;
            self.j = 0;
            self.swapped = false;
        }
    }

    fn steps_per_second(&mut self) -> usize {
        SortingAlgorithm::Bubble.steps()
    }

    fn finished(&self) -> bool {
        self.finished
    }

    fn reset(&mut self) {
        self.i = 0;
        self.j = 0;
        self.swapped = false;
        self.finished = false;
    }
}
