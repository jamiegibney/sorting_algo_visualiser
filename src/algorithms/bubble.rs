use super::*;

#[derive(Debug)]
pub struct Bubble {
    current_idx: usize,
    swapped: bool,
    finished: bool,
}

impl Bubble {
    pub fn new() -> Self {
        Self { current_idx: 0, swapped: false, finished: false }
    }
}

impl SortAlgorithm for Bubble {
    fn step(&mut self, slice: &mut SortArray) {
        todo!()
        // if self.finished {
        //     return None;
        // }
        //
        // let mut swaps = Vec::new();
        //
        // let len = slice.len();
        // let num_ops = len - self.current_idx - 1;
        //
        // for j in 0..(len - self.current_idx - 1) {
        //     if slice[j] > slice[j + 1] {
        //         slice.swap(j, j + 1);
        //
        //         swaps.push(j);
        //         swaps.push(j + 1);
        //     }
        // }
        //
        // self.current_idx += 1;
        //
        // if swaps.is_empty() {
        //     self.finished = true;
        // }
        //
        // Some(AlgorithmStep {
        //     num_ops,
        //     average_idx: if swaps.is_empty() {
        //         0
        //     }
        //     else {
        //         swaps.iter().sum::<usize>() / swaps.len()
        //     },
        // })
    }

    fn steps_per_second(&mut self) -> usize {
        SortingAlgorithm::Bubble.steps()
    }

    fn finished(&self) -> bool {
        self.finished
    }

    fn reset(&mut self) {
        self.current_idx = 0;
        self.swapped = false;
        self.finished = false;
    }
}
