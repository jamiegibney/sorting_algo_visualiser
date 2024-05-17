use super::*;

/// A selection sort.
#[derive(Debug)]
pub struct Selection {
    current_idx: usize,
    finished: bool,
}

impl SortAlgorithm for Selection {
    fn new() -> Self {
        Self { current_idx: 0, finished: false }
    }

    fn step(&mut self, slice: &mut [usize]) -> Option<AlgorithmStep> {
        // avoids index out of bounds error
        if self.finished {
            return None;
        }

        let average_comp_pos = (slice.len() + self.current_idx) / 2;
        let mut positions = [average_comp_pos; 3];

        let min_idx = slice
            .iter()
            .skip(self.current_idx)
            .enumerate()
            .min_by_key(|(_, &x)| x)
            .map(|(i, _)| i + self.current_idx);

        if let Some(idx) = min_idx {
            if idx != self.current_idx {
                slice.swap(idx, self.current_idx);

                positions[1] = idx;
                positions[2] = self.current_idx;
            }

            self.current_idx += 1;
        }

        if self.current_idx == slice.len() - 1 {
            self.finished = true;
        }

        Some(AlgorithmStep {
            num_ops: slice.len() - self.current_idx + 2,
            average_idx: positions.iter().sum::<usize>() / 3,
        })
    }

    fn steps_per_second(&mut self) -> usize {
        SortingAlgorithm::Selection.steps()
    }

    fn finished(&self) -> bool {
        self.finished
    }

    fn reset(&mut self) {
        self.current_idx = 0;
        self.finished = false;
    }
}
