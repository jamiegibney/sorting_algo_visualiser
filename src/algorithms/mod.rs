use super::*;
use std::collections::HashMap;
use std::fmt::Debug;

use SortingAlgorithm as SA;

mod bogo;
mod radix;
mod scramble;
mod selection;

use bogo::Bogo;
use radix::*;
use scramble::Scramble;
use selection::Selection;

pub trait SortAlgorithm: Debug {
    /// Creates a new algorithm.
    fn new() -> Self;

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

        Self { num_iters, average_pos }
    }

    pub fn num_iters(&self) -> usize {
        self.num_iters
    }

    pub fn average_pos(&self) -> f32 {
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
            Self::Selection => 150,

            Self::Scramble => 1500,
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
            SA::Selection => f.write_str("Selection sort"),
            SA::Scramble => f.write_str("Randomisation"),
        }
    }
}

#[derive(Debug)]
pub struct Algorithms {
    radix_inplace_lsd4: InPlaceRadixLSD4,
    radix_inplace_lsd10: InPlaceRadixLSD10,
    radix_lsd4: RadixLSD4,
    radix_lsd10: RadixLSD10,
    radix_msd4: RadixMSD4,
    radix_msd10: RadixMSD10,

    bogo: Bogo,
    selection: Selection,

    scramble: Scramble,
}

impl Algorithms {
    pub fn new() -> Self {
        Self {
            radix_inplace_lsd4: InPlaceRadixLSD4::new(),
            radix_inplace_lsd10: InPlaceRadixLSD10::new(),
            radix_lsd4: RadixLSD4::new(),
            radix_lsd10: RadixLSD10::new(),
            radix_msd4: RadixMSD4::new(),
            radix_msd10: RadixMSD10::new(),

            bogo: Bogo::new(),
            selection: Selection::new(),

            scramble: Scramble::new(),
        }
    }

    pub fn progress(
        &mut self,
        algorithm: SortingAlgorithm,
        delta_time: f32,
        slice: &mut [usize],
    ) -> Option<AlgorithmOutput> {
        match algorithm {
            SA::RadixLSD4 => self.radix_lsd4.progress(delta_time, slice),
            SA::RadixLSD10 => self.radix_lsd10.progress(delta_time, slice),
            SA::InPlaceRadixLSD4 => {
                self.radix_inplace_lsd4.progress(delta_time, slice)
            }
            SA::InPlaceRadixLSD10 => {
                self.radix_inplace_lsd10.progress(delta_time, slice)
            }
            SA::RadixMSD4 => self.radix_msd4.progress(delta_time, slice),
            SA::RadixMSD10 => self.radix_msd10.progress(delta_time, slice),

            SA::Bogo => self.bogo.progress(delta_time, slice),
            SA::Selection => self.selection.progress(delta_time, slice),

            SA::Scramble => self.scramble.progress(delta_time, slice),
        }
    }

    pub fn step(&mut self, algorithm: SortingAlgorithm, slice: &mut [usize]) {
        match algorithm {
            SA::RadixLSD4 => self.radix_lsd4.step(slice),
            SA::RadixLSD10 => self.radix_lsd10.step(slice),
            SA::InPlaceRadixLSD4 => self.radix_inplace_lsd4.step(slice),
            SA::InPlaceRadixLSD10 => self.radix_inplace_lsd10.step(slice),
            SA::RadixMSD4 => self.radix_msd4.step(slice),
            SA::RadixMSD10 => self.radix_msd10.step(slice),

            SA::Bogo => self.bogo.step(slice),
            SA::Selection => self.selection.step(slice),

            SA::Scramble => self.scramble.step(slice),
        };
    }

    pub fn finished(&self, algorithm: SortingAlgorithm) -> bool {
        match algorithm {
            SA::RadixLSD4 => self.radix_lsd4.finished(),
            SA::RadixLSD10 => self.radix_lsd10.finished(),
            SA::InPlaceRadixLSD4 => self.radix_inplace_lsd4.finished(),
            SA::InPlaceRadixLSD10 => self.radix_inplace_lsd10.finished(),
            SA::RadixMSD4 => self.radix_msd4.finished(),
            SA::RadixMSD10 => self.radix_msd10.finished(),

            SA::Bogo => self.bogo.finished(),
            SA::Selection => self.selection.finished(),

            SA::Scramble => self.scramble.finished(),
        }
    }

    pub fn reset(&mut self, algorithm: SortingAlgorithm) {
        match algorithm {
            SA::RadixLSD4 => self.radix_lsd4.reset(),
            SA::RadixLSD10 => self.radix_lsd10.reset(),
            SA::InPlaceRadixLSD4 => self.radix_inplace_lsd4.reset(),
            SA::InPlaceRadixLSD10 => self.radix_inplace_lsd10.reset(),
            SA::RadixMSD4 => self.radix_msd4.reset(),
            SA::RadixMSD10 => self.radix_msd10.reset(),

            SA::Bogo => self.bogo.reset(),
            SA::Selection => self.selection.reset(),

            SA::Scramble => self.scramble.reset(),
        }
    }
}
