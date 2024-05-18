use super::*;
use std::collections::HashMap;
use std::fmt::Debug;

use SortingAlgorithm as SA;

mod bogo;
mod bubble;
mod insertion;
mod radix;
mod scramble;
mod selection;

use bogo::Bogo;
use bubble::Bubble;
use insertion::Insertion;
use radix::*;
use scramble::Scramble;
use selection::Selection;

pub trait SortAlgorithm: Debug {
    /// Creates a new algorithm.
    // fn new() -> Self;

    /// A single sorting step.
    fn step(&mut self, arr: &mut SortArray);
    /// The target number of steps per second for this algorithm.
    fn steps_per_second(&mut self) -> usize;

    /// Whether the sort has finished or not.
    fn finished(&self) -> bool;
    /// Resets the sorting algorithm state.
    fn reset(&mut self);

    /// Progresses the algorithm based on `delta_time`.
    fn progress(&mut self, delta_time: f32, arr: &mut SortArray) {
        let steps =
            ((self.steps_per_second() as f32) * delta_time).round() as usize;

        if self.finished() {
            return;
        }

        for _ in 0..steps {
            self.step(arr);
        }
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
    Insertion,

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
            Self::Insertion => 150,

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
            Self::Selection => *self = Self::Insertion,
            Self::Insertion => *self = Self::RadixLSD4,
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
            SA::Insertion => f.write_str("Insertion sort"),
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
        let arr: [(SA, Box<dyn SortAlgorithm>); 11] = [
            (SA::RadixLSD4, Box::new(RadixLSD4::new())),
            (SA::RadixLSD10, Box::new(RadixLSD10::new())),
            (SA::InPlaceRadixLSD4, Box::new(InPlaceRadixLSD4::new())),
            (SA::InPlaceRadixLSD10, Box::new(InPlaceRadixLSD10::new())),
            (SA::RadixMSD4, Box::new(RadixMSD4::new())),
            (SA::RadixMSD10, Box::new(RadixMSD10::new())),
            (SA::Bogo, Box::new(Bogo::new())),
            (SA::Bubble, Box::new(Bubble::new())),
            (SA::Selection, Box::new(Selection::new())),
            (SA::Insertion, Box::new(Insertion::new())),
            (SA::Scramble, Box::new(Scramble::new())),
        ];

        Self { algos: HashMap::from(arr) }
    }

    pub fn progress(
        &mut self,
        algorithm: SortingAlgorithm,
        delta_time: f32,
        arr: &mut SortArray,
    ) {
        if let Some(algo) = self.algos.get_mut(&algorithm) {
            algo.progress(delta_time, arr);
        }
    }

    pub fn step(&mut self, algorithm: SortingAlgorithm, arr: &mut SortArray) {
        if let Some(algo) = self.algos.get_mut(&algorithm) {
            algo.step(arr);
        }
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
