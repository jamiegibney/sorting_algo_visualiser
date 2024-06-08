use super::*;
use std::cmp::Ordering as Ord;
use std::collections::HashMap;
use std::fmt::Debug;

use SortingAlgorithm as SA;

mod bogo;
mod bubble;
mod insertion;
mod radix;
mod selection;
mod shuffle;

use bogo::Bogo;
use bubble::Bubble;
use insertion::Insertion;
use radix::*;
use selection::Selection;
use shuffle::Scramble;

pub trait SortAlgorithm: Debug {
    fn process(&mut self, arr: &mut SortArray);
}

/// A particular sorting algorithm.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum SortingAlgorithm {
    // RADIX
    RadixLSD2,
    RadixLSD4,
    RadixLSD5,
    #[default]
    RadixLSD10,
    InPlaceRadixLSD4,
    InPlaceRadixLSD10,
    RadixMSD4,
    RadixMSD10,

    Bogo,
    Bubble,
    Selection,
    Insertion,

    Shuffle,
}

impl SortingAlgorithm {
    pub fn cycle_next(&mut self) {
        match self {
            Self::RadixLSD2 => *self = Self::RadixLSD4,
            Self::RadixLSD4 => *self = Self::RadixLSD5,
            Self::RadixLSD5 => *self = Self::RadixLSD10,
            Self::RadixLSD10 => *self = Self::InPlaceRadixLSD4,
            Self::InPlaceRadixLSD4 => *self = Self::InPlaceRadixLSD10,
            Self::InPlaceRadixLSD10 => *self = Self::RadixMSD4,
            Self::RadixMSD4 => *self = Self::RadixMSD10,
            Self::RadixMSD10 => *self = Self::Bogo,
            Self::Bogo => *self = Self::Bubble,
            Self::Bubble => *self = Self::Selection,
            Self::Selection => *self = Self::Insertion,
            Self::Insertion => *self = Self::RadixLSD2,
            Self::Shuffle => *self = Self::Bubble,
        }
    }

    pub fn cycle_prev(&mut self) {
        match self {
            Self::RadixLSD2 => *self = Self::Insertion,
            Self::RadixLSD4 => *self = Self::RadixLSD2,
            Self::RadixLSD5 => *self = Self::RadixLSD4,
            Self::RadixLSD10 => *self = Self::RadixLSD5,
            Self::InPlaceRadixLSD4 => *self = Self::RadixLSD10,
            Self::InPlaceRadixLSD10 => *self = Self::InPlaceRadixLSD4,
            Self::RadixMSD4 => *self = Self::InPlaceRadixLSD10,
            Self::RadixMSD10 => *self = Self::RadixMSD4,
            Self::Bogo => *self = Self::RadixMSD10,
            Self::Bubble => *self = Self::Bogo,
            Self::Selection => *self = Self::Bubble,
            Self::Insertion => *self = Self::Selection,
            Self::Shuffle => *self = Self::Bubble,
        }
    }
}

impl std::fmt::Display for SortingAlgorithm {
    #[allow(clippy::use_self)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SA::RadixLSD2 => f.write_str("LSD Radix sort, Base 2"),
            SA::RadixLSD4 => f.write_str("LSD Radix sort, Base 4"),
            SA::RadixLSD5 => f.write_str("LSD Radix sort, Base 5"),
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
            SA::Shuffle => f.write_str("Shuffle"),
        }
    }
}

#[derive(Debug)]
pub struct Algorithms {
    algos: HashMap<SortingAlgorithm, Box<dyn SortAlgorithm>>,
}

impl Algorithms {
    pub fn new() -> Self {
        let arr: [(SA, Box<dyn SortAlgorithm>); 13] = [
            (SA::RadixLSD2, Box::new(RadixBase::lsd_with_base(2))),
            (SA::RadixLSD4, Box::new(RadixLSD4::new())),
            (SA::RadixLSD5, Box::new(RadixBase::lsd_with_base(5))),
            (SA::RadixLSD10, Box::new(RadixLSD10::new())),
            (SA::InPlaceRadixLSD4, Box::new(InPlaceRadixLSD4::new())),
            (SA::InPlaceRadixLSD10, Box::new(InPlaceRadixLSD10::new())),
            (SA::RadixMSD4, Box::new(RadixMSD4::new())),
            (SA::RadixMSD10, Box::new(RadixMSD10::new())),
            (SA::Bogo, Box::new(Bogo::new())),
            (SA::Bubble, Box::new(Bubble::new())),
            (SA::Selection, Box::new(Selection::new())),
            (SA::Insertion, Box::new(Insertion::new())),
            (SA::Shuffle, Box::new(Scramble::new())),
        ];

        Self { algos: HashMap::from(arr) }
    }

    pub fn process(
        &mut self,
        algorithm: SortingAlgorithm,
        arr: &mut SortArray,
    ) {
        if let Some(algo) = self.algos.get_mut(&algorithm) {
            algo.process(arr);
        }
    }
}
