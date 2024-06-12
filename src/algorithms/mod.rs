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
mod cycle;
mod gnome;
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
use bucket::Bucket;
use cocktail::Cocktail;
use comb::Comb;
use cycle::Cycle;
use gnome::Gnome;
use heap::Heap;
use insertion::Insertion;
use merge::Merge;
use pancake::Pancake;
use quick::QuickSort;
use radix::*;
use selection::Selection;
use shell::Shell;
use shuffle::Shuffle;
use stooge::Stooge;
use timsort::Timsort;

/// Trait for sorting algorithms.
pub trait SortAlgorithm: Debug + Send + Sync {
    /// The sorting process. This should mutate the provided array to "sort"
    /// it — however that is defined for the algorithm.
    fn process(&mut self, arr: &mut SortArray);
}

/// A particular sorting algorithm.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, FromPrimitive)]
pub enum SortingAlgorithm {
    RadixLSD2,
    RadixLSD4,
    RadixLSD5,
    RadixLSD10,
    InPlaceRadixLSD4,
    InPlaceRadixLSD10,
    RadixMSD4,
    RadixMSD10,

    Bogo,
    Gnome,
    #[default]
    Stooge,
    Bubble,
    Pancake,
    Selection,
    Insertion,
    Shell,
    Comb,
    Cocktail,
    Cycle,

    Merge,
    Heap,
    QuickSort,

    // TODO:
    // Bucket,
    // Timsort,
    // Strand,
    // Bitonic,
    // Sleep,
    // Tag,
    // Tree,
    // Counting,
    // Bingo,
    // Pigeonhole,
    Shuffle,
}

unsafe impl bytemuck::NoUninit for SortingAlgorithm {}

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
    #[allow(clippy::enum_glob_use)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SortingAlgorithm::*;

        let mut write = |s| f.write_str(s);

        match self {
            RadixLSD2 => write("LSD Radix sort, Base 2"),
            RadixLSD4 => write("LSD Radix sort, Base 4"),
            RadixLSD5 => write("LSD Radix sort, Base 5"),
            RadixLSD10 => write("LSD Radix sort, Base 10"),
            InPlaceRadixLSD4 => write("In-place LSD Radix sort, Base 4"),
            InPlaceRadixLSD10 => write("In-place LSD Radix sort, Base 10"),
            RadixMSD4 => write("MSD Radix sort, Base 4"),
            RadixMSD10 => write("MSD Radix sort, Base 10"),
            Bogo => write("Bogosort"),
            Bubble => write("Bubble sort"),
            Pancake => write("Pancake sort"),
            Gnome => write("Gnome sort"),
            Stooge => write("Stooge sort"),
            Selection => write("Selection sort"),
            Insertion => write("Insertion sort"),
            Merge => write("Merge sort"),
            Heap => write("Heap sort"),
            Cycle => write("Cycle sort"),
            Shell => write("Shell sort"),
            Comb => write("Comb sort"),
            Cocktail => write("Cocktail sort"),
            QuickSort => write("QuickSort"),
            Shuffle => write("Shuffle"),
        }
    }
}

/// A struct which dynamically dispatches to the correct sorting algorithm.
#[derive(Debug)]
pub struct Algorithms {
    algos: HashMap<SortingAlgorithm, Box<dyn SortAlgorithm>>,
}

impl Algorithms {
    /// Creates and initializes all sorting algorithms.
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
            (SA::Gnome, Box::new(Gnome::new())),
            (SA::Stooge, Box::new(Stooge::new())),
            (SA::Bubble, Box::new(Bubble::new())),
            (SA::Pancake, Box::new(Pancake::new())),
            (SA::Selection, Box::new(Selection::new())),
            (SA::Insertion, Box::new(Insertion::new())),
            (SA::Merge, Box::new(Merge::new())),
            (SA::Heap, Box::new(Heap)),
            (SA::Shell, Box::new(Shell::new())),
            (SA::Comb, Box::new(Comb::new())),
            (SA::Cocktail, Box::new(Cocktail::new())),
            (SA::Cycle, Box::new(Cycle::new())),
            (SA::QuickSort, Box::new(QuickSort::new())),
            (SA::Shuffle, Box::new(Shuffle::new())),
        ];

        Self { algos: HashMap::from(arr) }
    }

    /// Processes the provided array via the process implemented for
    /// `algorithm`.
    pub fn process(
        &mut self,
        algorithm: SortingAlgorithm,
        arr: &mut SortArray,
    ) {
        self.algos
            .get_mut(&algorithm)
            .expect("Failed to find algorithm in Algorithms HashMap")
            .process(arr);
    }
}
