# Sorting algorithm audio-visualiser

A sorting algorithm "audio-visualiser", written in Rust and the [nannou](https://github.com/nannou-org/nannou) library.

This app draws a colour wheel, which is made up of a variable number of segments. These segments can be randomly shuffled, and then re-sorted via various different sorting algorithms.

The sorting operations (writes, swaps, comparisons, reads) are tracked and used to update the colour wheel display *and* send messages to the audio thread, which can quantise note values to a musical scale, or simply map array positions to frequency. Up to 256 voices are supported.

## TODO

#### Fixes
- [ ] Fix sort operation slice bounds
- [ ] Fix crash when resizing array during playback

#### Refactors
- [ ] Separate the sorting array to a separate type, which is held by a "manager" which offers methods like prepare, capture dumping, resizing etc. This prevents sorting algorithms from modifying the actual array beyond the usual sorting operations.
- [ ] Documentation for more of the core API
- [ ] Better names for certain types/functions

#### Features
- [ ] Parallelise the audio processing, so multiple threads can generate groups of voices and then sum the result on the main audio thread.
- [ ] Allow user to mute audio
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
