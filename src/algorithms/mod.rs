use super::*;
use std::collections::HashMap;
use std::fmt::Debug;

use SortingAlgorithm as SA;

mod radix;
mod scramble;
use radix::*;
use scramble::Scramble;

pub trait SortAlgorithm: Debug {
    fn new() -> Self;

    fn step(&mut self, slice: &mut [usize]);
    fn steps_per_second(&mut self) -> usize;

    fn finished(&self) -> bool;
    fn reset(&mut self);

    fn progress(&mut self, delta_time: f32, slice: &mut [usize]) {
        let steps =
            ((self.steps_per_second() as f32) * delta_time).round() as usize;

        if self.finished() {
            return;
        }

        for _ in 0..steps {
            self.step(slice);
        }
    }
}

/// A particular sorting algorithm.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum SortingAlgorithm {
    // RADIX
    #[default]
    RadixLSD4,
    RadixLSD10,
    InPlaceRadixLSD4,
    InPlaceRadixLSD10,
    RadixMSD4,
    RadixMSD10,
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
            Self::Scramble => 1500,
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

            scramble: Scramble::new(),
        }
    }

    pub fn progress(
        &mut self,
        algorithm: SortingAlgorithm,
        delta_time: f32,
        slice: &mut [usize],
    ) {
        match algorithm {
            SA::RadixLSD4 => self.radix_lsd4.progress(delta_time, slice),
            SA::RadixLSD10 => self.radix_lsd10.progress(delta_time, slice),
            SA::InPlaceRadixLSD4 => {
                self.radix_inplace_lsd4.progress(delta_time, slice);
            }
            SA::InPlaceRadixLSD10 => {
                self.radix_inplace_lsd10.progress(delta_time, slice);
            }
            SA::RadixMSD4 => self.radix_msd4.progress(delta_time, slice),
            SA::RadixMSD10 => self.radix_msd10.progress(delta_time, slice),

            SA::Scramble => self.scramble.progress(delta_time, slice),
        }
    }

    pub fn step(&mut self, algorithm: SortingAlgorithm, slice: &mut [usize]) {
        match algorithm {
            SA::RadixLSD4 => self.radix_lsd4.step(slice),
            SA::RadixLSD10 => self.radix_lsd10.step(slice),
            SA::InPlaceRadixLSD4 => {
                self.radix_inplace_lsd4.step(slice);
            }
            SA::InPlaceRadixLSD10 => {
                self.radix_inplace_lsd10.step(slice);
            }
            SA::RadixMSD4 => self.radix_msd4.step(slice),
            SA::RadixMSD10 => self.radix_msd10.step(slice),

            SA::Scramble => self.scramble.step(slice),
        }
    }

    pub fn finished(&self, algorithm: SortingAlgorithm) -> bool {
        match algorithm {
            SA::RadixLSD4 => self.radix_lsd4.finished(),
            SA::RadixLSD10 => self.radix_lsd10.finished(),
            SA::InPlaceRadixLSD4 => self.radix_inplace_lsd4.finished(),
            SA::InPlaceRadixLSD10 => self.radix_inplace_lsd10.finished(),
            SA::RadixMSD4 => self.radix_msd4.finished(),
            SA::RadixMSD10 => self.radix_msd10.finished(),

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

            SA::Scramble => self.scramble.reset(),
        }
    }
}
