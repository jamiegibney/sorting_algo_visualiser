use super::*;

/// A "moving-window" shuffle.
#[derive(Debug, Clone)]
pub struct Shuffle;

impl Shuffle {
    const ITERS_PER_STEP: usize = 30;

    pub const fn new() -> Self {
        Self
    }

    fn rand_idx(
        len: usize,
        win_start: usize,
        win_size: usize,
    ) -> (usize, usize) {
        let rand = |in_window: bool| {
            let end = if in_window { win_start + win_size } else { len - 1 };

            if end <= win_start {
                end
            }
            else {
                random_range(win_start, end + 1)
            }
            .clamp(0, len - 1)
        };

        (rand(true), rand(false))
    }
}

impl SortProcessor for Shuffle {
    fn process(&mut self, arr: &mut SortArray) {
        let len = arr.len();

        let mut win_size = len / 40;
        let mut win_start = 0;

        let step_size = (win_size / 8).max(1);

        while (win_start + win_size) < len {
            for _ in 0..Self::ITERS_PER_STEP {
                let (a, b) = Self::rand_idx(len, win_start, win_size);
                arr.swap(a, b);
            }

            win_start += step_size;
        }
    }
}
