use super::*;
use std::collections::HashMap;
use std::fmt::Debug;

use SortingAlgorithm as SA;

mod bogo;
mod bubble;
mod radix;
mod scramble;
mod selection;

use bogo::Bogo;
use bubble::Bubble;
use radix::*;
use scramble::Scramble;
use selection::Selection;

pub trait SortAlgorithm: Debug {
    /// Creates a new algorithm.
    // fn new() -> Self;

    /// A single sorting step.
    fn step(&mut self, slice: &mut [usize]) -> Option<AlgorithmStep>;
    /// The target number of steps per second for this algorithm.
    fn steps_per_second(&mut self) -> usize;

    /// Whether the sort has finished or not.
    fn finished(&self) -> bool;
    /// Resets the sorting algorithm state.
    fn reset(&mut self);

    /// Progresses the algorithm based on `delta_time`.
    fn progress(
        &mut self,
        delta_time: f32,
        slice: &mut [usize],
    ) -> Option<AlgorithmOutput> {
        let steps =
            ((self.steps_per_second() as f32) * delta_time).round() as usize;

        if self.finished() {
            return None;
        }

        Some(AlgorithmOutput::from_steps(
            &(0..steps)
                .filter_map(|_| self.step(slice))
                .collect::<Vec<AlgorithmStep>>(),
            slice,
        ))
    }
}

/// Information about a single sorting step.
#[derive(Debug, Default, Clone, Copy)]
pub struct AlgorithmStep {
    /// The number of sorting operations for this step.
    pub(super) num_ops: usize,
    /// The average of all indices processed in this step.
    pub(super) average_idx: usize,
}

impl AlgorithmStep {
    /// Returns the average position between `0.0` and `1.0` of the indices processed in the slice.
    pub fn interp_in_slice(&self, slice: &[usize]) -> f32 {
        let len = slice.len() as f32;
        let out = self.average_idx as f32 / len;

        out.clamp(0.0, 1.0)
    }
}

#[derive(Debug)]
pub struct AlgorithmOutput {
    num_iters: usize,
    average_pos: f32,
}

impl AlgorithmOutput {
    pub fn from_steps(steps: &[AlgorithmStep], slice: &[usize]) -> Self {
        let mut num_iters = 0;
        let mut average_pos = 0.0;

        for step in steps {
            num_iters += step.num_ops;
            average_pos += step.interp_in_slice(slice);
        }

        Self { num_iters, average_pos: average_pos / steps.len() as f32 }
    }

    pub const fn num_iters(&self) -> usize {
        self.num_iters
    }

    pub const fn average_pos(&self) -> f32 {
        self.average_pos
    }
}

/// A particular sorting algorithm.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum SortingAlgorithm {
    // RADIX
    RadixLSD4,
    RadixLSD10,
    InPlaceRadixLSD4,
    InPlaceRadixLSD10,
    RadixMSD4,
    RadixMSD10,

    Bogo,
    Bubble,
    Selection,

    #[default]
    Scramble,
}

impl SortingAlgorithm {
    pub const fn steps(self) -> usize {
        match self {
            Self::RadixLSD4 => 100,
            Self::RadixLSD10 => 100,
            Self::InPlaceRadixLSD4 => 100,
            Self::InPlaceRadixLSD10 => 100,
            Self::RadixMSD4 => 100,
            Self::RadixMSD10 => 100,

            Self::Bogo => 20000,
            Self::Bubble => 150,
            Self::Selection => 150,

            Self::Scramble => 1500,
        }
    }

    pub fn next(&mut self) {
        match self {
            Self::RadixLSD4 => *self = Self::RadixLSD10,
            Self::RadixLSD10 => *self = Self::InPlaceRadixLSD4,
            Self::InPlaceRadixLSD4 => *self = Self::InPlaceRadixLSD10,
            Self::InPlaceRadixLSD10 => *self = Self::RadixMSD4,
            Self::RadixMSD4 => *self = Self::RadixMSD10,
            Self::RadixMSD10 => *self = Self::Bogo,
            Self::Bogo => *self = Self::Bubble,
            Self::Bubble => *self = Self::Selection,
            Self::Selection => *self = Self::RadixLSD4,
            Self::Scramble => *self = Self::Scramble,
        }
    }
}

impl std::fmt::Display for SortingAlgorithm {
    #[allow(clippy::use_self)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SA::RadixLSD4 => f.write_str("LSD Radix sort, Base 4"),
            SA::RadixLSD10 => f.write_str("LSD Radix sort, Base 10"),
            SA::InPlaceRadixLSD4 => {
                f.write_str("In-place LSD Radix sort, Base 4")
            }
            SA::InPlaceRadixLSD10 => {
                f.write_str("In-place LSD Radix sort, Base 10")
            }
            SA::RadixMSD4 => f.write_str("MSD Radix sort, Base 4"),
            SA::RadixMSD10 => f.write_str("MSD Radix sort, Base 10"),
            SA::Bogo => f.write_str("Bogosort"),
            SA::Bubble => f.write_str("Bubble sort"),
            SA::Selection => f.write_str("Selection sort"),
            SA::Scramble => f.write_str("Randomisation"),
        }
    }
}

#[derive(Debug)]
pub struct Algorithms {
    algos: HashMap<SortingAlgorithm, Box<dyn SortAlgorithm>>,
}

impl Algorithms {
    pub fn new() -> Self {
        let arr: [(SA, Box<dyn SortAlgorithm>); 2] = [
            (SA::Bogo, Box::new(Bogo::new())),
            (SA::Bubble, Box::new(Bubble::new())),
        ];

        Self { algos: HashMap::from(arr) }
    }

    pub fn progress(
        &mut self,
        algorithm: SortingAlgorithm,
        delta_time: f32,
        slice: &mut [usize],
    ) -> Option<AlgorithmOutput> {
        self.algos
            .get_mut(&algorithm)
            .and_then(|algo| algo.progress(delta_time, slice))
    }

    pub fn step(
        &mut self,
        algorithm: SortingAlgorithm,
        slice: &mut [usize],
    ) -> Option<AlgorithmStep> {
        self.algos
            .get_mut(&algorithm)
            .and_then(|algo| algo.step(slice))
    }

    pub fn finished(&self, algorithm: SortingAlgorithm) -> bool {
        self.algos.get(&algorithm).map_or(false, |a| a.finished())
    }

    pub fn reset(&mut self, algorithm: SortingAlgorithm) {
        if let Some(algo) = self.algos.get_mut(&algorithm) {
            algo.reset();
        }
    }
}
