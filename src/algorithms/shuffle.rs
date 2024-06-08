use super::*;

/// A "moving-window" shuffle.
#[derive(Debug, Clone)]
pub struct Scramble {
    has_initialized: bool,

    window_start: usize,
    window_size: usize,

    step_size: usize,
    iter: usize,

    finished: bool,
}

impl Scramble {
    const ITERS_PER_STEP: usize = 12;

    pub const fn new() -> Self {
        Self {
            has_initialized: false,

            window_start: 0,
            window_size: 0,

            iter: 0,
            step_size: 0,

            finished: false,
        }
    }

    fn reset(&mut self) {
        self.has_initialized = false;

        self.window_start = 0;
        self.window_size = 0;

        self.step_size = 0;
        self.iter = 0;

        self.finished = false;
    }

    fn rand_idx(&self, len: usize) -> (usize, usize) {
        let rand = |in_window: bool| {
            let end = if in_window {
                self.window_start + self.window_size
            }
            else {
                len - 1
            };

            if end <= self.window_start {
                end
            }
            else {
                random_range(self.window_start, end + 1)
            }
            .clamp(0, len - 1)
        };

        (rand(true), rand(false))
    }
}

impl SortAlgorithm for Scramble {
    // fn step(&mut self, arr: &mut SortArray) {
    //     let len = arr.len();
    //
    //     if !self.has_initialized {
    //         self.window_size = len / 40;
    //         self.step_size = (self.window_size / 8).max(1);
    //         self.has_initialized = true;
    //     }
    //
    //     let start = self.window_start;
    //     let end = start + self.window_size;
    //
    //     let (a, b) = self.rand_idx(len);
    //     arr.swap(a, b);
    //
    //     self.iter += 1;
    //     if self.iter == Self::ITERS_PER_STEP {
    //         self.iter = 0;
    //         self.window_start += self.step_size;
    //     }
    //
    //     if end > len && self.has_initialized && self.iter == 0 {
    //         self.finished = true;
    //     }
    // }
    //
    // fn steps_per_second(&mut self) -> usize {
    //     SortingAlgorithm::Shuffle.speed()
    // }
    //
    // fn finished(&self) -> bool {
    //     self.finished
    // }
    //
    // fn reset(&mut self) {
    //     self.reset();
    // }
    fn process(&mut self, arr: &mut SortArray) {
        todo!();
    }
}
