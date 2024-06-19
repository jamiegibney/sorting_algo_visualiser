# Sorting algorithm audio-visualiser

[Full video](https://drive.google.com/file/d/1UTanxeJtD_La77ys0kDYRH2Z_1uQbJi4/view?usp=sharing)

A sorting algorithm "audio-visualiser", written in Rust and the [nannou](https://github.com/nannou-org/nannou) library.

This app draws a colour wheel, which is made up of a variable number of segments. These segments can be randomly shuffled, and then re-sorted via various different sorting algorithms.

The sorting operations (writes, swaps, comparisons, reads) are "recorded" and used to update the colour wheel display *and* send audio note messages. Up to 2048 audio voices are available, though this is only a hard limit that could certainly be raised.

The audio generation utilises SIMD and multi-threading optimisations to generate voices with SIMD operations in parallel on up to 16 threads. The audio FX processors (a high-pass filter and compressor) also use SIMD operations.

The SIMD optimisation does not drastically improve performance, and is mainly used to handle stereo processing in single steps. The multi-threaded voice generation, however, improves audio performance by approximately 10x based on some rough tests.

Due to the use of SIMD, this project requires the nightly Rust compiler when building from source.

## Keymap

Currently, the only way to interact with the program is via keymaps. A mouse-based UI is currently a WIP.

- `Space`: toggle algorithm playback (or restart playback if the end has been reached)
- `Backspace` or `Delete`: stop and reset playback to the beginning
- `Return`: cycle to the next algorithm, or hold Shift to cycle to the previous algorithm
- `R`: "run" a sorting algorithm
- `S`: "shuffle" the current wheel
- `F`: "force-sort" the current wheel
- `M`: toggle audio mute
- `N`: "next" algorithm: this shuffles the current wheel, and then runs the next algorithm when done (or press Shift-N to run the previous algorithm when done)
- `-`: decrease wheel resolution, i.e. the number of array elements
- `+`: increase wheel resolution, i.e. the number of array elements
- `,` or `<`: decrease playback speed
- `.` or `>`: increase playback speed

## Implemented sorting algorithms (in order)

- Bogosort (the stupid sort)
- Stooge sort
- Gnome sort
- Bubble sort
- Selection sort
- Insertion sort
- Pancake sort
- Shell sort
- Comb sort
- Cocktail sort
- Bingo sort
- Cycle sort
- Counting sort
- Pigeonhole sort
- Merge sort
- Heap sort
- TimSort
- QuickSort
- Radix sorts:
    - LSD (least significant digit), base 2
    - LSD, base 5
    - LSD, base 10
    - LSD, base 32
    - LSD, base 1000
    - In-place LSD, base 2
    - In-place LSD, base 10
    - In-place LSD, base 32
    - In-place LSD, base 1000
    - MSD (most significant digit), base 10
    - MSD, base 32
    - MSD, base 1000
- Sleep sort (currently not guaranteed to sort the array, just in here for fun)

## TODO

#### Fixes
- [ ] Fix cases where "Shuffling" text is not cleared but should be ([#1](https://github.com/jamiegibney/sorting_algo_visualiser/issues/1))
- [ ] Fix crash when resizing array during playback ([#2](https://github.com/jamiegibney/sorting_algo_visualiser/issues/2))
- [ ] Fix sort operation slice bounds ([#3](https://github.com/jamiegibney/sorting_algo_visualiser/issues/3))

#### Features
- [x] Parallelise the audio processing, so multiple threads can generate groups of voices and then sum the result on the main audio thread.
- [x] Allow user to mute audio
- [x] Swap/comparison operations should post note events for both array positions, rather than an average.
- [x] Add the ability for an "auto" command where the array is shuffled, the next algorithm is selected, and then the array is sorted with that algorithm.
- [ ] Add array size restrictions for particular algorithms (such as bogo or stooge sort)
- [ ] UI controls
    - [ ] Playback controls
    - [ ] Menu for sorting algorithms
    - [ ] Buttons for sorting/shuffling/resetting
    - [ ] Different font?
- [ ] Implement sorting algorithms
    - [x] Bingo sort
    - [ ] Bucket sort
    - [ ] Strand sort? Normally uses an input & output buffer, so might be boring to visualise.
    - [ ] Tree sort? Uses a node-based data structure, which might take a while to implement.
    - [ ] Bitonic sort? Normally requires arrays with a power-of-two size.

#### Refactors
- [x] Further optimise audio processing (SIMD?) to increase max voice count.
- [ ] Separate the sorting array to a separate type, which is held by a "manager" which offers methods like prepare, capture dumping, resizing etc. This prevents sorting algorithms from modifying the actual array beyond the usual sorting operations.
- [ ] Manage which audio threads receive which incoming events more intelligently.
- [ ] Add a more efficient blocking method to the thread pools (e.g. spin-lock?).
