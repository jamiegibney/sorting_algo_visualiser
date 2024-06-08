use crate::prelude::*;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct SortCapture {
    /// The initial state of the array.
    initial_array: Vec<usize>,
    /// The list of operations.
    operations: Vec<SortOperation>,
    /// A stack of written values, used to undo any previous write operations.
    write_stack: Vec<usize>,

    /// The scratch buffer, used to perform the operations.
    scratch: Vec<usize>,

    /// The algorithm used for this sort.
    algorithm: SortingAlgorithm,

    /// The current position in the operation buffer.
    counter: usize,
    /// The previous position in the operation buffer.
    counter_last: usize,
}

impl SortCapture {
    /// Creates a new `SortCapture`.
    pub fn create(
        init_arr: Vec<usize>,
        operations: Vec<SortOperation>,
        algorithm: SortingAlgorithm,
    ) -> Self {
        let num_writes = operations
            .iter()
            .filter(|&&op| matches!(op, SortOperation::Write { .. }))
            .count();

        Self {
            initial_array: init_arr.clone(),
            operations,
            write_stack: Vec::with_capacity(num_writes),

            scratch: init_arr,

            algorithm,

            counter: 0,
            counter_last: 0,
        }
    }

    /// The algorithm used for this sort.
    pub const fn algorithm(&self) -> SortingAlgorithm {
        self.algorithm
    }

    /// The operation at the current playback position.
    pub fn current_operation(&self) -> SortOperation {
        self.operations[self.counter]
    }

    /// The current state of the array.
    pub fn arr(&self) -> &[usize] {
        &self.scratch
    }

    /// The number of elements in the array.
    pub fn len(&self) -> usize {
        self.initial_array.len()
    }

    /// Whether the array is currently sorted.
    pub fn is_sorted(&self) -> bool {
        self.scratch.iter().enumerate().all(|(i, &val)| i == val)
    }

    /// (unimplemented)
    pub fn serialize(&self) {
        unimplemented!();
    }

    pub fn is_done(&self) -> bool {
        self.counter == self.initial_array.len() - 1
    }

    /// Returns the current progress of the sorting process as a value between
    /// `0.0` and `1.0`.
    pub fn playback_progress(&self) -> f32 {
        let n = (self.initial_array.len() - 1) as f32;
        self.counter as f32 / n
    }

    /// Sets the "playback progress" of the capture, and returns a slice of the
    /// operations performed in the process.
    ///
    /// The ordering of the operations always follows a forward arrangement in
    /// the buffer — i.e., if the progress is rewound, then the operations in
    /// the slice are still ordered going forward.
    #[must_use]
    pub fn set_progress(&mut self, mut progress: f32) -> Rc<[SortOperation]> {
        progress = progress.clamp(0.0, 1.0);
        let n = (self.initial_array.len() - 1) as f32;

        let curr = self.counter;
        let last = self.counter_last;

        self.counter = (progress * n).round() as usize;
        self.set_arr();

        self.operations[match curr.cmp(&last) {
            Ordering::Less => curr..last,
            // useless lint as its suggestion doesn't compile
            #[allow(clippy::range_plus_one)]
            Ordering::Equal => curr..curr + 1,
            Ordering::Greater => last..curr,
        }]
        .into()
    }

    fn set_arr(&mut self) {
        let mut tmp_counter = self.counter_last;

        let (num_ops, rewind) = {
            match self.counter.cmp(&self.counter_last) {
                Ordering::Less => (self.counter_last - self.counter, true),
                Ordering::Equal => return,
                Ordering::Greater => (self.counter - self.counter_last, false),
            }
        };

        for _ in 0..num_ops {
            match self.operations.get(tmp_counter).copied() {
                Some(SortOperation::Write { idx, value }) => {
                    if rewind {
                        // if we're rewinding (i.e. undoing), then we need to
                        // pop the last value from the write stack.
                        self.scratch[idx] = self.write_stack.pop().unwrap();
                    }
                    else {
                        // otherwise, we push the current value in the scratch
                        // buffer before overwriting it.
                        self.write_stack.push(self.scratch[idx]);
                        self.scratch[idx] = value;
                    }
                }
                Some(SortOperation::Swap { a, b }) => {
                    // swap operations are always reversible.
                    self.scratch.swap(a, b);
                }

                _ => {}
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
