use super::algorithms::*;
use super::*;
use atomic::Atomic;
use std::sync::Arc;

/// The sorting algorithm process.
#[derive(Debug)]
pub struct Process {
    algorithms: Algorithms,
    current_algorithm: Arc<Atomic<SortingAlgorithm>>,
}

impl Process {
    pub fn new(current_algorithm: Arc<Atomic<SortingAlgorithm>>) -> Self {
        Self { algorithms: Algorithms::new(), current_algorithm }
    }

    /// Processes the currently-selected algorithm if it can.
    pub fn sort(&mut self, arr: &mut SortArray) {
        self.algorithms
            .process(self.current_algorithm.load(Relaxed), arr);
    }
}
