use super::algorithms::*;
use super::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// The sorting algorithm process.
#[derive(Debug)]
pub struct Process {
    sort_arr: SortArray,
    aux_arr: Vec<usize>,

    algorithms: Algorithms,
    pub current_algorithm: SortingAlgorithm,

    running: bool,

    last: Instant,
}

impl Process {
    pub fn new(sort_arr: SortArray) -> Self {
        let len = sort_arr.lock().unwrap().len();

        Self {
            sort_arr,
            aux_arr: Vec::with_capacity(len),

            algorithms: Algorithms::new(),

            current_algorithm: SortingAlgorithm::default(),

            running: false,

            last: Instant::now(),
        }
    }

    pub fn with_algorithm(mut self, algorithm: SortingAlgorithm) -> Self {
        self.set_algorithm(algorithm);
        self
    }

    pub fn set_algorithm(&mut self, algorithm: SortingAlgorithm) {
        self.current_algorithm = algorithm;
    }

    pub fn update(&mut self) -> bool {
        let delta_time = self.last.elapsed().as_secs_f32();

        if self.running {
            if self.algorithms.finished(self.current_algorithm) {
                self.algorithms.reset(self.current_algorithm);
            }
        }
        else {
            self.last = Instant::now();
            return false;
        }

        if let Ok(mut guard) = self.sort_arr.lock() {
            self.algorithms.progress(
                self.current_algorithm,
                delta_time,
                guard.as_mut_slice(),
            );
        }

        if self.algorithms.finished(self.current_algorithm) {
            self.stop();
            self.last = Instant::now();
            return true;
        }

        self.last = Instant::now();
        false
    }

    pub const fn is_running(&self) -> bool {
        self.running
    }

    pub fn toggle(&mut self) {
        self.running = !self.running;
    }

    pub fn run(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}
