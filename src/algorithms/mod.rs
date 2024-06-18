use super::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::fmt::{Debug, Display};

use SortingAlgorithm as SA;

mod bingo;
mod bogo;
mod bubble;
mod bucket;
mod cocktail;
mod comb;
mod counting;
mod cycle;
mod gnome;
mod heap;
mod insertion;
mod merge;
mod pancake;
mod pigeonhole;
mod quick;
mod radix;
mod selection;
mod shell;
mod shuffle;
mod sleep;
mod stooge;
mod timsort;

use bingo::Bingo;
use bogo::Bogo;
use bubble::Bubble;
use bucket::Bucket;
use cocktail::Cocktail;
use comb::Comb;
use counting::Counting;
use cycle::Cycle;
use gnome::Gnome;
use heap::Heap;
use insertion::Insertion;
use merge::Merge;
use pancake::Pancake;
use pigeonhole::Pigeonhole;
use quick::QuickSort;
use radix::*;
use selection::Selection;
use shell::Shell;
use shuffle::Shuffle;
use sleep::Sleep;
use stooge::Stooge;
use timsort::Timsort;

/// Trait for sorting algorithms.
pub trait SortProcessor: Debug + Send + Sync {
    /// The sorting process. This should mutate the provided array to "sort"
    /// it, whatever that may mean for the algorithm.
    fn process(&mut self, arr: &mut SortArray);
}

/// A particular sorting algorithm.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, FromPrimitive)]
pub enum SortingAlgorithm {
    #[default]
    Bogo,
    Stooge,

    Gnome,
    Bubble,
    Selection,
    Insertion,
    Pancake,
    Shell,
    Comb,
    Cocktail,

    Bingo,
    Cycle,
    // TODO: Bucket sort...
    Counting,
    Pigeonhole,

    Merge,
    Heap,
    Timsort,
    QuickSort,

    // TODO: none of the radix sorts are currently implemented. refer to this
    // LSD sort:
    // https://github.com/w0rthy/ArrayVisualizer/blob/master/src/array/visualizer/sort/RadixLSD.java
    // and this bubble sort to understand the API:
    // https://github.com/w0rthy/ArrayVisualizer/blob/master/src/array/visualizer/sort/BubbleSort.java
    RadixLSD2,
    RadixLSD4,
    RadixLSD5,
    RadixLSD10,
    RadixLSD32,
    InPlaceRadixLSD2,
    InPlaceRadixLSD4,
    InPlaceRadixLSD10,
    InPlaceRadixLSD32,
    RadixMSD4,
    RadixMSD10,
    RadixMSD32,

    Sleep,

    // TODO: Bitonic sort requires arrays with a power of two length.
    // Bitonic,
    // TODO: Strand sort is certainly feasible, but might be quite boring as
    // it uses an input & output buffer.
    // Strand,

    // NOTE: Shuffle MUST be the last variant in order for the cycling methods
    // to function.
    Shuffle,
}

unsafe impl bytemuck::NoUninit for SortingAlgorithm {}

impl SortingAlgorithm {
    /// Cycles to the next sorting algorithm. This never cycles over
    /// [`SortingAlgorithm::Shuffle`], and if the current algorithm is
    /// [`SortingAlgorithm::Shuffle`] then this method will cycle to
    /// [`SortingAlgorithm::Bubble`].
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

    /// Cycles to the previous sorting algorithm. This never cycles over
    /// [`SortingAlgorithm::Shuffle`], and if the current algorithm is
    /// [`SortingAlgorithm::Shuffle`] then this method will cycle to
    /// [`SortingAlgorithm::Bubble`].
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

impl Display for SortingAlgorithm {
    #[allow(clippy::enum_glob_use)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SortingAlgorithm::*;

        let mut write = |s| f.write_str(s);

        match self {
            RadixLSD2 => write("LSD Radix sort, Base 2"),
            RadixLSD4 => write("LSD Radix sort, Base 4"),
            RadixLSD5 => write("LSD Radix sort, Base 5"),
            RadixLSD10 => write("LSD Radix sort, Base 10"),
            RadixLSD32 => write("LSD Radix sort, Base 32"),
            InPlaceRadixLSD2 => write("In-place LSD Radix sort, Base 2"),
            InPlaceRadixLSD4 => write("In-place LSD Radix sort, Base 4"),
            InPlaceRadixLSD10 => write("In-place LSD Radix sort, Base 10"),
            InPlaceRadixLSD32 => write("In-place LSD Radix sort, Base 32"),
            RadixMSD4 => write("MSD Radix sort, Base 4"),
            RadixMSD10 => write("MSD Radix sort, Base 10"),
            RadixMSD32 => write("MSD Radix sort, Base 32"),
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
            Counting => write("Counting sort"),
            Pigeonhole => write("Pigeonhole sort"),
            QuickSort => write("QuickSort"),
            Sleep => write("Sleep sort (not stable)"),
            Shuffle => write("Shuffle"),
            Bingo => write("Bingo sort"),
            // Bucket => write("Bucket sort"),
            Timsort => write("TimSort"),
        }
    }
}

/// A struct which dynamically dispatches to the correct sorting algorithm.
#[derive(Debug)]
pub struct Algorithms {
    algos: HashMap<SortingAlgorithm, Box<dyn SortProcessor>>,
}

impl Algorithms {
    /// Creates and initializes all sorting algorithms.
    pub fn new() -> Self {
        let arr = [
            (SA::Bogo, Box::new(Bogo::new()) as Box<dyn SortProcessor>),
            (SA::Stooge, Box::new(Stooge::new())),
            (SA::Gnome, Box::new(Gnome::new())),
            (SA::Bubble, Box::new(Bubble::new())),
            (SA::Selection, Box::new(Selection::new())),
            (SA::Insertion, Box::new(Insertion::new())),
            (SA::Pancake, Box::new(Pancake::new())),
            (SA::Shell, Box::new(Shell::new())),
            (SA::Comb, Box::new(Comb::new())),
            (SA::Cocktail, Box::new(Cocktail::new())),
            (SA::Bingo, Box::new(Bingo::new())),
            (SA::Cycle, Box::new(Cycle::new())),
            // (SA::Bucket, Box::new(Bucket::new())),
            (SA::Counting, Box::new(Counting::new())),
            (SA::Pigeonhole, Box::new(Pigeonhole::new())),
            (SA::Merge, Box::new(Merge::new())),
            (SA::Heap, Box::new(Heap)),
            (SA::Timsort, Box::new(Timsort::new())),
            (SA::QuickSort, Box::new(QuickSort::new())),
            (SA::RadixLSD2, Box::new(RadixLSD::new(2))),
            (SA::RadixLSD4, Box::new(RadixLSD::new(4))),
            (SA::RadixLSD5, Box::new(RadixLSD::new(5))),
            (SA::RadixLSD10, Box::new(RadixLSD::new(10))),
            (SA::RadixLSD32, Box::new(RadixLSD::new(32))),
            (SA::InPlaceRadixLSD2, Box::new(RadixLSDInPlace::new(2))),
            (SA::InPlaceRadixLSD4, Box::new(RadixLSDInPlace::new(4))),
            (SA::InPlaceRadixLSD10, Box::new(RadixLSDInPlace::new(10))),
            (SA::InPlaceRadixLSD32, Box::new(RadixLSDInPlace::new(32))),
            (SA::RadixMSD4, Box::new(RadixMSD::new(4))),
            (SA::RadixMSD10, Box::new(RadixMSD::new(10))),
            (SA::RadixMSD32, Box::new(RadixMSD::new(32))),
            (SA::Sleep, Box::new(Sleep::new())),
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
