# Sorting algorithm audio-visualiser

A sorting algorithm "audio-visualiser", written in Rust and the [nannou](https://github.com/nannou-org/nannou) library.

This app draws a colour wheel, which is made up of a variable number of segments. These segments can be randomly shuffled, and then re-sorted via various different sorting algorithms.

The sorting operations (writes, swaps, comparisons, reads) are tracked and used to update the colour wheel display *and* send messages to the audio thread, which can quantise note values to a musical scale, or simply map array positions to frequency. Up to 2048 audio voices are supported.

## TODO

#### Fixes
- [ ] Fix crash when resizing array during playback
- [ ] Fix sort operation slice bounds

#### Refactors
- [x] Prevent sending audio generation task to thread pool if it's not needed (the challenge is *how* to know when it's not needed).
- [ ] Separate the sorting array to a separate type, which is held by a "manager" which offers methods like prepare, capture dumping, resizing etc. This prevents sorting algorithms from modifying the actual array beyond the usual sorting operations.
- [ ] Better names for certain types/functions
- [ ] Manage which audio threads receive which incoming events more intelligently.
- [ ] Further optimise audio processing (SIMD?) to increase max voice count.
- [ ] Add a more efficient blocking method to the thread pools.

#### Features
- [x] Parallelise the audio processing, so multiple threads can generate groups of voices and then sum the result on the main audio thread.
- [x] Allow user to mute audio
- [x] Swap/comparison operations should post note events for both array positions, rather than an average.
- [x] Add the ability for an "auto" command where the array is shuffled, the next algorithm is selected, and then the array is sorted with that algorithm.
- [ ] Add certain array size restrictions for particular algorithms (such as bogo or stooge sort)
- [ ] Add sorting algorithms
    - [ ] Bucket sort
    - [ ] Bingo sort (sort of there...)
    - [ ] Strand sort?
    - [ ] Tree sort?
    - [ ] Bitonic sort?
- [ ] UI controls
    - [ ] Playback controls
    - [ ] Menu for sorting algorithms
    - [ ] Buttons for sorting/shuffling/resetting...
    - [ ] Different font, because why not
