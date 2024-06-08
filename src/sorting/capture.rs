use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct SortCapture {
    initial_array: Vec<usize>,
    operations: Vec<SortOperation>,

    scratch: Vec<usize>,

    algorithm: SortingAlgorithm,

    counter: usize,
    counter_last: usize,
}

impl SortCapture {
    pub fn create(
        init_arr: Vec<usize>,
        operations: Vec<SortOperation>,
        algorithm: SortingAlgorithm,
    ) -> Self {
        Self {
            initial_array: init_arr.clone(),
            scratch: init_arr,
            operations,
            algorithm,
            counter: 0,
            counter_last: 0,
        }
    }

    pub fn arr(&self) -> &[usize] {
        &self.scratch
    }

    pub const fn algorithm(&self) -> SortingAlgorithm {
        self.algorithm
    }

    pub fn len(&self) -> usize {
        self.initial_array.len()
    }

    pub fn playback_progress(&self) -> f32 {
        let n = (self.initial_array.len() - 1) as f32;
        self.counter as f32 / n
    }

    pub fn set_progress(&mut self, mut progress: f32) -> Vec<SortOperation> {
        progress = progress.clamp(0.0, 1.0);
        let n = (self.initial_array.len() - 1) as f32;

        let res = self.ops_for_step();

        self.counter = (progress * n).round() as usize;
        self.set_arr();

        res
    }

    pub fn current_operation(&self) -> SortOperation {
        self.operations[self.counter]
    }

    pub fn serialize(&self) {
        unimplemented!();
    }

    fn ops_for_step(&self) -> Vec<SortOperation> {
        Vec::from(match self.counter.cmp(&self.counter_last) {
            Ordering::Less => {
                let start = self.counter;
                let end = self.counter_last;
                &self.operations[start..end]
            }
            Ordering::Equal => &self.operations[self.counter..=self.counter],
            Ordering::Greater => {
                let start = self.counter_last;
                let end = self.counter;
                &self.operations[start..end]
            }
        })
    }

    fn set_arr(&mut self) {
        let mut tmp_counter = self.counter_last;

        let (diff, rewind) = {
            match self.counter.cmp(&self.counter_last) {
                Ordering::Less => (self.counter_last - self.counter, true),
                Ordering::Equal => return,
                Ordering::Greater => (self.counter - self.counter_last, false),
            }
        };

        for _ in 0..diff {
            match &mut self.operations[tmp_counter] {
                SortOperation::Noop
                | SortOperation::Read { .. }
                | SortOperation::Compare { .. } => {}
                SortOperation::Write { idx, value } => {
                    std::mem::swap(&mut self.scratch[*idx], value);
                }
                SortOperation::Swap { a, b } => {
                    self.scratch.swap(*a, *b);
                }
            }

            if rewind {
                tmp_counter -= 1;
            }
            else {
                tmp_counter += 1;
            }
        }

        self.counter_last = self.counter;
    }
}

