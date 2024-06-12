use super::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::cmp::Ordering as Ord;
use std::collections::HashMap;
use std::fmt::Debug;

use SortingAlgorithm as SA;

mod bogo;
mod bubble;
mod bucket;
mod cocktail;
mod comb;
mod heap;
mod insertion;
mod merge;
mod pancake;
mod quick;
mod radix;
mod selection;
mod shell;
mod shuffle;
mod stooge;
mod timsort;

use bogo::Bogo;
use bubble::Bubble;
use cocktail::Cocktail;
use comb::Comb;
use heap::Heap;
use insertion::Insertion;
use merge::Merge;
use quick::QuickSort;
use radix::*;
use selection::Selection;
use shell::Shell;
use shuffle::Shuffle;
use timsort::Timsort;

pub trait SortAlgorithm: Debug {
    fn process(&mut self, arr: &mut SortArray);
}

/// A particular sorting algorithm.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, FromPrimitive)]
pub enum SortingAlgorithm {
    // RADIX
    RadixLSD2 = 0,
    RadixLSD4,
    RadixLSD5,
    RadixLSD10,
    InPlaceRadixLSD4,
    InPlaceRadixLSD10,
    RadixMSD4,
    RadixMSD10,

    Bogo,
    Bubble,
    Selection,
    Insertion,
    Merge,
    Heap,
    #[default]
    Shell,
    Comb,
    Cocktail,
    QuickSort,

    Shuffle,
}

impl SortingAlgorithm {
    pub fn cycle_next(&mut self) {
        if matches!(*self, Self::Shuffle) {
            *self = Self::Bubble;
        }

        let max = Self::Shuffle as usize;
        let n = (*self as usize + 1) % max;

        if let Some(next) = FromPrimitive::from_usize(n) {
            *self = next;
        }
    }

    pub fn cycle_prev(&mut self) {
        if matches!(*self, Self::Shuffle) {
            *self = Self::Bubble;
        }

        let max = Self::Shuffle as usize;
        let s = *self as usize;
        let n = if s == 0 { max - 1 } else { s - 1 };

        if let Some(next) = FromPrimitive::from_usize(n) {
            *self = next;
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
            SA::Merge => f.write_str("Merge sort"),
            SA::Heap => f.write_str("Heap sort"),
            SA::Shell => f.write_str("Shell sort"),
            SA::Comb => f.write_str("Comb sort"),
            SA::Cocktail => f.write_str("Cocktail sort"),
            SA::QuickSort => f.write_str("QuickSort"),
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
        let arr = [
            (
                SA::RadixLSD2,
                Box::new(RadixLSD::new(2)) as Box<dyn SortAlgorithm>,
            ),
            (SA::RadixLSD4, Box::new(RadixLSD::new(4))),
            (SA::RadixLSD5, Box::new(RadixLSD::new(5))),
            (SA::RadixLSD10, Box::new(RadixLSD::new(10))),
            (SA::InPlaceRadixLSD4, Box::new(RadixLSDInPlace::new(4))),
            (SA::InPlaceRadixLSD10, Box::new(RadixLSDInPlace::new(10))),
            (SA::RadixMSD4, Box::new(RadixMSD::new(4))),
            (SA::RadixMSD10, Box::new(RadixMSD::new(10))),
            (SA::Bogo, Box::new(Bogo::new())),
            (SA::Bubble, Box::new(Bubble::new())),
            (SA::Selection, Box::new(Selection::new())),
            (SA::Insertion, Box::new(Insertion::new())),
            (SA::Merge, Box::new(Merge::new())),
            (SA::Heap, Box::new(Heap)),
            (SA::Shell, Box::new(Shell::new())),
            (SA::Comb, Box::new(Comb::new())),
            (SA::Cocktail, Box::new(Cocktail::new())),
            (SA::QuickSort, Box::new(QuickSort::new())),
            (SA::Shuffle, Box::new(Shuffle::new())),
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
