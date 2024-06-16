use super::*;

/// A "moving-window" shuffle.
#[derive(Debug, Clone)]
pub struct Shuffle;

impl Shuffle {
    const ITERS_PER_STEP: usize = 10;

    pub const fn new() -> Self {
        Self
    }

    fn rand_above(len: usize, start: usize, size: usize) -> (usize, usize) {
        let rand = |in_win: bool| {
            let max = if in_win { start + size } else { len }.min(len + 1);

            if start == len - 1 {
                len - 1
            }
            else {
                random_range(start, max)
            }
        };

        (rand(true), rand(true))
    }

    fn rand_below(len: usize, start: usize, size: usize) -> (usize, usize) {
        let rand = |in_win: bool| {
            let min = if in_win && start >= size { start - size } else { 0 };
            random_range(min, start)
        };

        (rand(true), rand(true))
    }

    // fn rand_idx(
    //     len: usize,
    //     win_start: usize,
    //     win_size: usize,
    // ) -> (usize, usize) {
    //     let rand = |in_window: bool| {
    //         let end = if in_window { win_start + win_size } else { len - 1 };
    //
    //         if end <= win_start {
    //             end
    //         }
    //         else {
    //             random_range(win_start, end + 1)
    //         }
    //         .clamp(0, len - 1)
    //     };
    //
    //     (rand(true), rand(false))
    // }
}

impl SortProcessor for Shuffle {
    fn process(&mut self, arr: &mut SortArray) {
        let n = arr.len() - 1;
        let win_size = (n / 4).max(1);
        let step = 4;

        let mut head_bot = 0;
        let mut head_top = arr.len() - 1;

        for _ in (0..n * 2).step_by(step) {
            for _ in 0..Self::ITERS_PER_STEP {
                let (ab, bb) = Self::rand_above(n, head_bot, win_size);
                let (at, bt) = Self::rand_below(n, head_top, win_size);
                arr.swap(ab, bt);
                arr.swap(at, bb);
            }

            head_bot = (head_bot + step) % n;
            head_top = if head_top < step { n - 1 } else { head_top - step };
        }
    }
}
