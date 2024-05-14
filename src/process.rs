use super::*;
use std::sync::{Arc, Mutex};

trait SortTiming {
    /// This method defines the sorting algorithm's "refresh rate".
    fn steps_per_second(&mut self) -> usize;
}

/// A particular sorting algorithm.
#[derive(Debug, Clone, Copy, Default)]
pub enum SortingAlgorithm {
    // RADIX
    // RadixLSD2,
    // RadixLSD4,
    // RadixLSD10,
    #[default]
    InPlaceRadixLSD4,
    // RadixMSD2,
    // RadixMSD4,
}

impl SortTiming for SortingAlgorithm {
    fn steps_per_second(&mut self) -> usize {
        match self {
            Self::InPlaceRadixLSD4 => 100,
        }
    }
}

/// The sorting algorithm process.
#[derive(Debug)]
pub struct Process {
    sort_arr: SortArray,
    aux_arr: Vec<usize>,

    algorithm_type: SortingAlgorithm,
}

impl Process {
    pub fn new(sort_arr: SortArray) -> Self {
        let len = sort_arr.lock().unwrap().len();
            
        Self {
            sort_arr,
            aux_arr: Vec::with_capacity(len),
            algorithm_type: SortingAlgorithm::default(),
        }
    }

    // pub fn run(&mut self

    pub fn with_algorithm(mut self, algorithm: SortingAlgorithm) -> Self {
        self.set_algorithm(algorithm);
        self
    }

    pub fn set_algorithm(&mut self, algorithm: SortingAlgorithm) {
        self.algorithm_type = algorithm;
    }

    pub fn update(&mut self) {
    }
}
