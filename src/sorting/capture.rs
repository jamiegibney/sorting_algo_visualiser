use crate::prelude::*;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Default)]
pub struct SortData {
    pub reads: usize,
    pub comparisons: usize,
    pub writes: usize,
    pub swaps: usize,
}

impl SortData {
    pub fn update(&mut self, op: SortOperation, rewind: bool) {
        match op {
            SortOperation::Write { idx, value } => {
                if rewind {
                    self.writes -= 1;
                }
                else {
                    self.writes += 1;
                }
            }
            SortOperation::Read { idx } => {
                if rewind {
                    self.reads -= 1;
                }
                else {
                    self.reads += 1;
                }
            }
            SortOperation::Swap { a, b } => {
                if rewind {
                    self.swaps -= 1;
                }
                else {
                    self.swaps += 1;
                }
            }
            SortOperation::Compare { a, b, res } => {
                if rewind {
                    self.comparisons -= 1;
                }
                else {
                    self.comparisons += 1;
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SortCapture {
    ///  The initial state of the array.
    // initial_array: Vec<usize>,
    /// The list of operations.
    operations: Arc<[SortOperation]>,
    /// A stack of written values, used to undo any previous write operations.
    write_stack: Vec<usize>,

    /// The scratch buffer, used to perform the operations.
    scratch: Vec<usize>,

    /// The algorithm used for this sort.
    algorithm: SortingAlgorithm,

    /// The current position in the operation buffer.
    cursor: usize,
    /// The previous position in the operation buffer.
    cursor_last: usize,

    pub data: SortData,
}

impl SortCapture {
    /// Creates a new `SortCapture`.
    pub fn create(
        init_arr: Vec<usize>,
        operations: Arc<[SortOperation]>,
        algorithm: SortingAlgorithm,
        num_writes: usize,
    ) -> Self {
        Self {
            // initial_array: init_arr.clone(),
            operations,
            write_stack: Vec::with_capacity(num_writes),

            scratch: init_arr,

            algorithm,

            cursor: 0,
            cursor_last: 0,

            data: SortData::default(),
        }
    }

    /// The algorithm used for this sort.
    pub const fn algorithm(&self) -> SortingAlgorithm {
        self.algorithm
    }

    /// The operation at the current playback position.
    pub fn current_operation(&self) -> SortOperation {
        self.operations[self.cursor]
    }

    /// The internal array.
    pub fn arr(&self) -> &[usize] {
        &self.scratch
    }

    /// The number of elements in the array.
    pub fn len(&self) -> usize {
        self.scratch.len()
        // self.initial_array.len()
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
        self.cursor == self.operations.len()
    }

    /// Returns the current progress of the sorting process as a value between
    /// `0.0` and `1.0`.
    pub fn playback_progress(&self) -> f32 {
        let n = (self.operations.len() - 1) as f32;
        self.cursor as f32 / n
    }

    /// Sets the "playback progress" of the capture, and returns a slice of the
    /// operations performed in the process.
    ///
    /// The ordering of the operations always follows a forward arrangement in
    /// the buffer — i.e., if the progress is rewound, then the operations in
    /// the slice are still ordered going forward.
    #[must_use]
    pub fn set_progress(&mut self, progress: f32) -> Arc<[SortOperation]> {
        if self.operations.is_empty() {
            return [].into();
        }

        self.cursor_last = self.cursor;

        let n = self.operations.len() as f32;

        self.cursor = (progress.clamp(0.0, 1.0) * n) as usize;
        self.set_arr();

        // FIXME: please fix this
        self.operations[match self.cursor.cmp(&self.cursor_last) {
            Ordering::Less => self.cursor..self.cursor_last,
            Ordering::Equal => {
                if self.cursor == 0 {
                    0..1
                }
                else {
                    (self.cursor - 1)..self.cursor
                }
            }
            Ordering::Greater => self.cursor_last..self.cursor,
        }]
        .into()
    }

    pub fn reset_progress(&mut self) {
        // self.scratch.copy_from_slice(&self.initial_array);
        self.set_progress(0.0);
        self.write_stack.clear();
        self.cursor = 0;
        self.cursor_last = 0;
        self.data = SortData::default();
    }

    fn set_arr(&mut self) {
        if self.cursor_last == self.cursor {
            return;
        }

        let rewind = self.cursor < self.cursor_last;

        let mut update_arr = |i: usize| {
            if let Some(op) = self.operations.get(i).copied() {
                self.data.update(op, rewind);

                match op {
                    SortOperation::Write { idx, value } => {
                        if rewind {
                            // if we're rewinding (i.e. undoing), then we need
                            // to pop the last value
                            // from the write stack.
                            self.scratch[idx] = self.write_stack.pop().unwrap();
                        }
                        else {
                            // otherwise, we push the current value in the
                            // scratch buffer before
                            // overwriting it.
                            self.write_stack.push(self.scratch[idx]);
                            self.scratch[idx] = value;
                        }
                    }
                    SortOperation::Swap { a, b } => {
                        // swap operations are always reversible.
                        self.scratch.swap(a, b);
                    }
                    _ => {}
                }
            }
        };

        if rewind {
            for i in (self.cursor..self.cursor_last).rev() {
                update_arr(i);
            }
        }
        else {
            for i in self.cursor_last..self.cursor {
                update_arr(i);
            }
        }
    }
}
