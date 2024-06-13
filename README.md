# Sorting algorithm audio-visualiser

A sorting algorithm "audio-visualiser", written in Rust and the [nannou](https://github.com/nannou-org/nannou) library.

This app draws a colour wheel, which is made up of a variable number of segments. These segments can be randomly shuffled, and then re-sorted via various different sorting algorithms.

The sorting operations (writes, swaps, comparisons, reads) are tracked and used to update the colour wheel display *and* send messages to the audio thread, which quantises note values to a musical scale. The audio processing is polyphonic.

## TODO

- [ ] Fix sort operation slice bounds
- [ ] Fix crash when resizing array during playback
- [ ] Separate the sorting array to a separate type, which is held by a "manager" which offers methods like prepare, capture dumping, resizing etc. This prevents sorting algorithms from modifying the actual array beyond the usual sorting operations.
- [ ] Add certain array size restrictions for particular algorithms (such as bogo or stooge sort)
- [ ] Add new sorting algorithms
    - [ ] Bingo sort
    - [ ] Radix sorts
    - [ ] Bucket sort
    - [ ] TimSort
- [ ] UI controls
    - [ ] Playback controls
    - [ ] Menu for sorting algorithms
    - [ ] Buttons for sorting/shuffling/resetting...
    - [ ] Different font, because why not
