use crate::prelude::*;

/// Each kind of sorting operation.
#[derive(Clone, Copy, Debug)]
pub enum SortOperation {
    Noop,
    Write { idx: usize, value: usize },
    Read { idx: usize },
    Swap { a: usize, b: usize },
    Compare { a: usize, b: usize, res: bool },
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SortResults {
    pub writes: usize,
    pub reads: usize,
    pub swaps: usize,
    pub comparisons: usize,
}

impl SortResults {
    pub fn add_from(&mut self, other: &Self) {
        self.writes += other.writes;
        self.reads += other.reads;
        self.swaps += other.swaps;
        self.comparisons += other.comparisons;
    }

    pub fn reset(&mut self) {
        self.writes = 0;
        self.reads = 0;
        self.swaps = 0;
        self.comparisons = 0;
    }
}

/// The sortable array.
#[derive(Debug)]
pub struct SortArray {
    /// The array of data.
    arr: Vec<usize>,

    /// The most recent sorting operation.
    last_op: SortOperation,
    /// A buffer of previous sorting operations.
    op_buffer: Option<Vec<SortOperation>>,

    current_algorithm: SortingAlgorithm,

    note_event_sender: Sender<NoteEvent>,
    audio_callback_timer: Arc<Atomic<InstantTime>>,
}

impl SortArray {
    /// Creates a new sorting array with `len` elements.
    pub fn new(
        len: usize,
        note_event_sender: Sender<NoteEvent>,
        audio_callback_timer: Arc<Atomic<InstantTime>>,
    ) -> Self {
        Self {
            arr: (0..len).collect(),

            op_buffer: None,
            last_op: SortOperation::Noop,

            current_algorithm: SortingAlgorithm::default(),

            note_event_sender,
            audio_callback_timer,
        }
    }

    /// Writes `value` to position `idx`. Will panic if `idx > `[`SortArray::len()`].
    pub fn write(&mut self, idx: usize, value: usize) {
        self.add_op(SortOperation::Write { idx, value });
        self.arr[idx] = value;
    }

    /// Returns the value as position `idx`. Will panic if `idx > `[`SortArray::len()`].
    pub fn read(&mut self, idx: usize) -> usize {
        self.add_op(SortOperation::Read { idx });
        self.arr[idx]
    }

    /// Swaps the elements at positions `a` and `b`. Will panic if either index is
    /// greater than [`SortArray::len()`].
    pub fn swap(&mut self, a: usize, b: usize) {
        self.add_op(SortOperation::Swap { a, b });
        self.arr.swap(a, b);
    }

    /// Compares the elements at positions `a` and `b` to match `ord`. Will panic if
    /// either index is greater than [`SortArray::len()`].
    pub fn cmp(&mut self, a: usize, b: usize, ord: Ordering) -> bool {
        let cmp = self.arr[a].cmp(&self.arr[b]);
        let res = cmp == ord;

        self.add_op(SortOperation::Compare { a, b, res });

        res
    }

    /// Returns the sorting results from the `SortArray`'s operation buffer.
    pub fn sort_results(&self) -> SortResults {
        self.op_buffer
            .as_ref()
            .map_or_else(SortResults::default, |buf| {
                let (mut writes, mut reads, mut swaps, mut comparisons) =
                    (0, 0, 0, 0);
                for x in buf {
                    match x {
                        SortOperation::Noop => (),
                        SortOperation::Write { .. } => writes += 1,
                        SortOperation::Read { .. } => reads += 1,
                        SortOperation::Swap { .. } => swaps += 1,
                        SortOperation::Compare { .. } => comparisons += 1,
                    }
                }

                SortResults { writes, reads, swaps, comparisons }
            })
    }

    /// Returns the last sorting operation, if it exists.
    pub const fn last_operation(&self) -> SortOperation {
        self.last_op
    }

    /// Consumes the operation buffer and returns it.
    pub fn take_op_buffer(&mut self) -> Vec<SortOperation> {
        self.op_buffer.take().unwrap_or_default()
    }

    /// Resizes the sorting array.
    pub fn resize(&mut self, new_size: usize) {
        self.arr.resize(new_size, 0);
        self.arr.iter_mut().enumerate().for_each(|(i, x)| *x = i);
    }

    /// Sets the current sorting algorithm.
    pub fn set_current_algorithm(&mut self, algorithm: SortingAlgorithm) {
        self.current_algorithm = algorithm;
    }

    /// Forces the array to be sorted with `std::sort_unstable()`.
    pub fn force_sort(&mut self) {
        self.arr.sort_unstable();
    }

    /// The length of the sorting array.
    pub fn len(&self) -> usize {
        self.arr.len()
    }

    /// Returns the sorting array as a slice.
    pub fn as_slice(&self) -> &[usize] {
        &self.arr
    }

    fn add_op(&mut self, op: SortOperation) {
        self.last_op = op;
        if let Some(buf) = self.op_buffer.as_mut() {
            buf.push(op);
        }
        else {
            self.op_buffer = Some(vec![op]);
        }

        self.send_note_event(op);
    }

    fn send_note_event(&self, op: SortOperation) {
        let len_f = self.len() as f32;

        let (mut freq, mut amp) = (0.5, 1.0);

        match op {
            SortOperation::Noop => {}
            SortOperation::Write { idx, .. } => {
                freq = idx as f32 / len_f * 0.5;
                amp = 0.6;
            }
            SortOperation::Read { idx } => {
                freq = idx as f32 / len_f * 0.5;
                freq *= 2.0;
                amp = 0.5;
            }
            SortOperation::Swap { a, b } => {
                freq = (a as f32 / len_f + b as f32 / len_f) * 0.25;
                amp = 0.7;
            }
            SortOperation::Compare { a, b, .. } => {
                freq = (a as f32 / len_f + b as f32 / len_f) * 0.25;
                freq *= 2.0;
                amp = 0.4;
            }
        }

        _ = self.note_event_sender.try_send(NoteEvent {
            freq: Self::map_freq(freq),
            amp,
            timing: self.buffer_sample_offset(),
        });
    }

    fn map_freq(average_idx: f32) -> f32 {
        const MIN_NOTE: f32 = 48.0;
        const MAX_NOTE: f32 = 100.0;

        let x = average_idx.clamp(0.0, 1.0).powf(1.5);
        let note = (MAX_NOTE - MIN_NOTE).mul_add(x, MIN_NOTE).round();
        let quantized = Audio::quantize_to_scale(&MAJ_PENTATONIC, note, 63.0);

        Audio::note_to_freq(quantized)
    }

    fn buffer_sample_offset(&self) -> u32 {
        use std::sync::atomic::Ordering::Relaxed;

        let samples_exact = self
            .audio_callback_timer
            .load(Relaxed)
            .elapsed()
            .as_secs_f32()
            * SAMPLE_RATE as f32;

        samples_exact.round() as u32 % BUFFER_SIZE as u32
    }
}

#[derive(Debug)]
pub struct SortArray2 {
    /// The current sorting algorithm.
    curr_algorithm: SortingAlgorithm,

    /// The scratch buffer, used for the sorting process.
    arr: Vec<usize>,
    /// The initial array before the sorting process.
    initial_arr: Vec<usize>,

    /// The buffer of operations — where the sorting operations are recorded to.
    op_buffer: Vec<SortOperation>,
}

impl SortArray2 {
    pub fn new(
        len: usize,
    ) -> Self {
        Self {
            curr_algorithm: SortingAlgorithm::default(),
            arr: (0..len).collect(),
            initial_arr: (0..len).collect(),
            op_buffer: vec![],
        }
    }

    /// Writes `value` to position `idx`. Will panic if
    /// `idx > `[`SortArray::len()`].
    pub fn write(&mut self, idx: usize, value: usize) {
        self.push(SortOperation::Write { idx, value });
        self.arr[idx] = value;
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
        self.arr.swap(a, b);

        res
    }

    /// Prepares the array for sorting, using its current state as the initial
    /// array.
    pub fn prepare_for_sort(&mut self, algorithm: SortingAlgorithm) {
        self.curr_algorithm = algorithm;
        self.initial_arr.copy_from_slice(&self.arr);
        self.op_buffer.clear();
    }

    /// Prepares the array for sorting, using the provided slice as the initial
    /// array.
    pub fn prepare_for_sort_with(
        &mut self,
        init_arr: &[usize],
        algorithm: SortingAlgorithm,
    ) {
        assert_eq!(init_arr.len(), self.len(), "Mismatched array lengths");

        self.arr.copy_from_slice(init_arr);
        self.prepare_for_sort(algorithm);
    }

    /// Generates a [`SortCapture`] from the current array state by *cloning*
    /// the internal data.
    ///
    /// In other words, this method ensures that the `SortArray` maintains
    /// its internal state after creating a capture. If you don't need this
    /// behavior, use [`Self::dump_capture`] instead.
    pub fn create_capture(&self) -> SortCapture {
        SortCapture::create(
            self.initial_arr.clone(),
            self.op_buffer.clone(),
            self.curr_algorithm,
        )
    }

    /// Returns a [`SortCapture`] from the current array state, consuming
    /// the internal data.
    pub fn dump_capture(&mut self) -> SortCapture {
        use std::mem::swap;

        let mut init = vec![];
        let mut op = vec![];

        swap(&mut self.initial_arr, &mut init);
        swap(&mut self.op_buffer, &mut op);

        SortCapture::create(init, op, self.curr_algorithm)
    }

    /// Resizes the sorting array.
    pub fn resize(&mut self, new_size: usize) {
        self.arr = (0..new_size).collect();
        self.initial_arr = (0..new_size).collect();
    }

    /// Force-sorts the array.
    pub fn force_sort(&mut self) {
        self.arr.sort_unstable();
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
    /// This method should *not* be used by sorting algorithms, as it bypasses the 
    /// operation recording.
    pub unsafe fn inner(&self) -> &[usize] {
        &self.arr
    }

    fn push(&mut self, op: SortOperation) {
        self.op_buffer.push(op);
    }
}
