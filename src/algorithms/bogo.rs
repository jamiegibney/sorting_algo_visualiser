use super::*;
use nannou::rand::random_range;

/// A bogosort.
#[derive(Debug)]
pub struct Bogo {
    finished: bool,
}

impl Bogo {
    pub fn new() -> Self {
        Self { finished: false }
    }

    fn is_sorted(&self, slice: &[usize]) -> bool {
        for win in slice.windows(2) {
            if win[0] > win[1] {
                return false;
            }
        }

        true
    }
}

impl SortAlgorithm for Bogo {
    fn step(&mut self, slice: &mut [usize]) -> Option<AlgorithmStep> {
        if self.is_sorted(slice) {
            self.finished = true;
            return None;
        }

        let mut rand_positions = vec![0; slice.len()];

        let len = slice.len();

        for i in 0..len {
            let j = random_range(0, len);
            slice.swap(i, j);

            rand_positions[i] = j;
        }

        Some(AlgorithmStep {
            num_ops: len,
            average_idx: {
                let rand_average =
                    rand_positions.iter().sum::<usize>() / rand_positions.len();

                (rand_average + len / 2) / 2
            },
        })
    }

    fn steps_per_second(&mut self) -> usize {
        SortingAlgorithm::Bogo.steps()
    }

    fn finished(&self) -> bool {
        self.finished
    }

    fn reset(&mut self) {
        self.finished = false;
    }
}
