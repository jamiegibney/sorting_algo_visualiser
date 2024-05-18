use super::*;

#[derive(Debug)]
pub struct Insertion {
    finished: bool,
}

impl Insertion {
    pub fn new() -> Self {
        Self { finished: false }
    }
}

impl SortAlgorithm for Insertion {
    fn step(&mut self, slice: &mut SortArray) {
        todo!()
        // if self.finished {
        //     return None;
        // }
        //
        // let len = slice.len();
        //
        // for i in 1..len {
        //     let key = slice[i];
        //     let mut j = i as isize - 1;
        //
        //     while j >= 0 && slice[j as usize] > key {
        //         slice[j as usize + 1] = slice[j as usize];
        //         j -= 1;
        //     }
        //
        //     slice[(j + 1) as usize] = key;
        // }
        //
        // self.finished = true;
        //
        // Some(AlgorithmStep { num_ops: 0, average_idx: 0 })
    }

    fn steps_per_second(&mut self) -> usize {
        SortingAlgorithm::Insertion.steps()
    }

    fn finished(&self) -> bool {
        self.finished
    }

    fn reset(&mut self) {
        self.finished = false;
    }
}
