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
    pub current_algorithm: SortingAlgorithm,

    last: Instant,

    iters_last_update: usize,
}

impl Process {
    pub fn new() -> Self {
        Self {
            algorithms: Algorithms::new(),
            current_algorithm: SortingAlgorithm::default(),

            last: Instant::now(),

            iters_last_update: 0,
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
    pub fn sort(&mut self, arr: &mut SortArray) {
        self.algorithms.process(self.current_algorithm, arr);
    }
}
