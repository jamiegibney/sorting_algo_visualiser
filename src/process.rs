use super::algorithms::*;
use super::*;
use atomic::Atomic;
use std::sync::{mpsc::Sender, Arc, Mutex};
use std::time::Instant;

// const NOTE_POST_TIME: f32 = 0.010;

/// The sorting algorithm process.
#[derive(Debug)]
pub struct Process {
    algorithms: Algorithms,
    current_algorithm: Arc<Atomic<SortingAlgorithm>>,

    last: Instant,

    iters_last_update: usize,
}

impl Process {
    pub fn new(current_algorithm: Arc<Atomic<SortingAlgorithm>>) -> Self {
        Self {
            algorithms: Algorithms::new(),
            current_algorithm,

            last: Instant::now(),

            iters_last_update: 0,
        }
    }

    /// Processes the currently-selected algorithm if it can.
    pub fn sort(&mut self, arr: &mut SortArray) {
        self.algorithms
            .process(self.current_algorithm.load(Relaxed), arr);
    }
}
