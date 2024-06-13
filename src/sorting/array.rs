use crate::prelude::*;

/// Each kind of sorting operation.
#[derive(Clone, Copy, Debug)]
pub enum SortOperation {
    Write { idx: usize, value: usize },
    Read { idx: usize },
    Swap { a: usize, b: usize },
    Compare { a: usize, b: usize, res: bool },
}

#[derive(Debug)]
pub struct SortArray {
    /// The current sorting algorithm.
    curr_algorithm: SortingAlgorithm,

    /// The scratch buffer, used for the sorting process.
    arr: Vec<usize>,
    /// The initial array before the sorting process.
    initial_arr: Vec<usize>,

    /// The buffer of operations — where the sorting operations are recorded
    /// to.
    op_buffer: Vec<SortOperation>,

    /// A counter which is passed to a capture to prevent unnecessary
    /// computation later.
    num_writes: usize,
}

impl SortArray {
    pub fn new(len: usize) -> Self {
        Self {
            curr_algorithm: SortingAlgorithm::default(),
            arr: (0..len).collect(),
            initial_arr: (0..len).collect(),
            op_buffer: vec![],
            num_writes: 0,
        }
    }

    /// Writes `value` to position `idx`. Will panic if
    /// `idx > `[`SortArray::len()`].
    pub fn write(&mut self, idx: usize, value: usize) {
        self.push(SortOperation::Write { idx, value });
        self.arr[idx] = value;
        self.num_writes += 1;
    }

    /// Returns the value as position `idx`. Will panic if
    /// `idx > `[`SortArray::len()`].
    pub fn read(&mut self, idx: usize) -> usize {
        self.push(SortOperation::Read { idx });
        self.arr[idx]
    }

    /// Swaps the elements at positions `a` and `b`. Will panic if either index
    /// is greater than [`SortArray::len()`].
    pub fn swap(&mut self, a: usize, b: usize) {
        self.push(SortOperation::Swap { a, b });
        self.arr.swap(a, b);
    }

    /// Compares the elements at positions `a` and `b` to match `ord`. Will
    /// panic if either index is greater than [`SortArray::len()`].
    pub fn cmp(&mut self, a: usize, b: usize, ord: Ordering) -> bool {
        let cmp = self.arr[a].cmp(&self.arr[b]);
        let res = cmp == ord;

        self.push(SortOperation::Compare { a, b, res });

        res
    }

    /// Copies the internal array to `dest`.
    ///
    /// # Panics
    ///
    /// Panics if `dest.len() != `[`Self::len()`].
    pub fn copy_to(&mut self, dest: &mut [usize]) {
        assert_eq!(self.len(), dest.len(), "Mismatched array lengths");

        for i in 0..self.arr.len() {
            self.push(SortOperation::Read { idx: i });
        }

        dest.copy_from_slice(&self.arr);
    }

    /// Prepares the array for sorting, using its current state as the initial
    /// array.
    pub fn prepare_for_sort(&mut self, algorithm: SortingAlgorithm) {
        self.curr_algorithm = algorithm;
        self.initial_arr = self.arr.clone();
        self.op_buffer.clear();
        self.num_writes = 0;
    }

    /// Prepares the array for sorting, using the provided slice as the initial
    /// array.
    ///
    /// # Panics
    ///
    /// Panics if `init_arr.len() != `[`Self::len()`].
    pub fn prepare_for_sort_with(
        &mut self,
        init_arr: &[usize],
        algorithm: SortingAlgorithm,
    ) {
        assert_eq!(init_arr.len(), self.len(), "Mismatched array lengths");

        self.arr.copy_from_slice(init_arr);
        self.prepare_for_sort(algorithm);
    }

    /// Generates a [`SortCapture`] from the current array state, consuming the
    /// internal data.
    pub fn dump_capture(&mut self) -> SortCapture {
        use std::mem::take;

        SortCapture::create(
            take(&mut self.initial_arr),
            Arc::new(take(&mut self.op_buffer).into_boxed_slice()),
            self.curr_algorithm,
            self.num_writes,
        )
    }

    /// Resizes the sorting array.
    pub fn resize(&mut self, new_size: usize) {
        self.arr = (0..new_size).collect();
        self.initial_arr = (0..new_size).collect();
    }

    /// Force-sorts the array.
    pub fn force_sort(&mut self) {
        self.arr.iter_mut().enumerate().for_each(|(i, x)| *x = i);
        self.initial_arr.copy_from_slice(&self.arr);
    }

    /// Whether the array is currently sorted.
    pub fn is_sorted(&self) -> bool {
        self.arr.iter().enumerate().all(|(i, &v)| i == v)
    }

    /// The number of elements in the array.
    pub fn len(&self) -> usize {
        self.arr.len()
    }

    /// Returns the array as a slice.
    ///
    /// # Safety
    ///
    /// This method should *not* be used by sorting algorithms, as it bypasses
    /// the operation recording.
    pub unsafe fn inner(&self) -> &[usize] {
        &self.arr
    }

    fn push(&mut self, op: SortOperation) {
        self.op_buffer.push(op);
    }
}
