use super::algorithms::*;
use super::*;
use atomic::Atomic;
use std::sync::{mpsc::Sender, Arc, Mutex};
use std::time::Instant;

const NOTE_POST_TIME: f32 = 0.010;

/// The sorting algorithm process.
#[derive(Debug)]
pub struct Process {
    algorithms: Algorithms,
    pub current_algorithm: SortingAlgorithm,

    running: bool,
    last: Instant,

    iters_last_update: usize,

    note_sender: Sender<NoteEvent>,
    audio_callback_timer: Arc<Atomic<InstantTime>>,
    note_post_timer: f32,
}

impl Process {
    pub fn new(
        note_sender: Sender<NoteEvent>,
        audio_callback_timer: Arc<Atomic<InstantTime>>,
    ) -> Self {
        Self {
            algorithms: Algorithms::new(),
            current_algorithm: SortingAlgorithm::default(),

            running: false,
            last: Instant::now(),

            iters_last_update: 0,

            note_sender,
            audio_callback_timer,
            note_post_timer: 0.0,
        }
    }

    pub fn with_algorithm(mut self, algorithm: SortingAlgorithm) -> Self {
        self.set_algorithm(algorithm);
        self
    }

    pub fn set_algorithm(&mut self, algorithm: SortingAlgorithm) {
        self.current_algorithm = algorithm;
    }

    /// Processes the currently-selected algorithm if it can.
    ///
    /// Returns `true` if the algorithm has finished sorting *and* the process is running.
    pub fn update(&mut self, arr: &mut SortArray) -> bool {
        let delta_time = self.last.elapsed().as_secs_f32();
        self.note_post_timer += delta_time;
        self.iters_last_update = 0;
        self.last = Instant::now();

        if self.running {
            if self.algorithms.finished(self.current_algorithm) {
                // if the algorithm has finished and we want to run, reset it before starting
                // again.
                self.algorithms.reset(self.current_algorithm);
            }
        }
        else {
            // if the process isn't running...
            return false;
        }

        // progress the algorithm...
        self
            .algorithms
            .progress(self.current_algorithm, delta_time, arr);

        // self.iters_last_update = output.num_iters();

        // if we've just sorted the slice...
        if self.algorithms.finished(self.current_algorithm) {
            self.stop();
            return true;
        }

        // if the slice has yet to be sorted...
        false
    }

    pub const fn is_running(&self) -> bool {
        self.running
    }

    pub const fn iters_last_update(&self) -> usize {
        self.iters_last_update
    }

    pub fn toggle(&mut self) {
        self.running = !self.running;
    }

    pub fn run(&mut self) {
        self.running = true;
        self.note_post_timer = 0.0;
    }

    pub fn stop(&mut self) {
        self.running = false;
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
