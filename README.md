# Sorting algorithm audio-visualiser

A sorting algorithm "audio-visualiser", written in Rust and the [nannou](https://github.com/nannou-org/nannou) library.

This app draws a colour wheel, which is made up of a variable number of segments. These segments can be randomly shuffled, and then re-sorted via various different sorting algorithms.

The sorting operations (writes, swaps, comparisons, reads) are tracked and used to update the colour wheel display *and* send messages to the audio thread, which quantises note values to a musical scale. The audio processing is polyphonic.

## TODO

- [ ] Reimplement sorting algorithms with `SortArray` methods
- [ ] Add new sorting algorithms
    - [ ] Merge sort
    - [ ] Quick sort
    - [ ] Heap sort
    - [ ] Radix sorts
    - [ ] Bucket sort
    - [ ] Shell sort
    - [ ] TimSort
    - [ ] Gnome sort
    - [ ] ...
- [ ] Send audio messages from `SortArray`, synchronised with the audio thread
- [ ] Send/receive GUI messages from `SortArray` to update visual
- [ ] Add some basic UI elements
    - [ ] Text displays for algorithm information
    - [ ] UI controls for sorting/scrambling/changing algorithm
