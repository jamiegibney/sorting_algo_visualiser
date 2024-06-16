pub use super::*;
pub use crate::audio::{
    Audio, AudioEffect, Filter, BUFFER_SIZE, MAJOR_SCALE, MAJ_PENT_SCALE,
    MINOR_SCALE, MIN_PENT_SCALE, NUM_VOICES, SAMPLE_RATE,
};
pub use crate::sorting::*;
pub use crate::Drawable;
pub use algorithms::SortingAlgorithm;
pub use atomic::Atomic;
pub use crossbeam_channel::{bounded, Receiver, Sender};
pub use nannou::prelude::*;
pub use nannou_audio::Buffer;
pub use parking_lot::Mutex;
pub use sorting::*;
pub use std::f32::consts::{FRAC_PI_2, TAU};
pub use std::simd::{f32x64, num::SimdFloat, simd_swizzle, Simd, StdFloat};
pub use std::sync::atomic::{AtomicBool, AtomicU32, Ordering::Relaxed};
pub use std::time::Instant;
pub use std::{cmp::Ordering, sync::Arc};

/// An `f32x64` where each element is `0.0` through `63.0`.
pub const SIMD_STAGGER: f32x64 = f32x64::from_array([
    0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0,
    14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0,
    26.0, 27.0, 28.0, 29.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0,
    38.0, 39.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0, 47.0, 48.0, 49.0,
    50.0, 51.0, 52.0, 53.0, 54.0, 55.0, 56.0, 57.0, 58.0, 59.0, 60.0, 61.0,
    62.0, 63.0,
]);

/// An `f32x64` where are elements are `64.0`.
pub const SIMD_STEP: f32x64 = f32x64::from_array([
    64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0,
    64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0,
    64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0,
    64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0,
    64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0, 64.0,
    64.0, 64.0, 64.0, 64.0,
]);

/// `1.0` as an f32x64.
///
/// # Example
///
/// ```
/// assert_eq!(f32x64::splat(1.0), ONE);
/// ```
pub const SIMD_ONE: f32x64 = f32x64::from_array([
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0,
]);

/// `2.0` as an f32x64.
///
/// # Example
///
/// ```
/// assert_eq!(f32x64::splat(2.0), TWO);
/// ```
pub const SIMD_TWO: f32x64 = f32x64::from_array([
    2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
    2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
    2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
    2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
    2.0, 2.0, 2.0, 2.0,
]);

/// `0.5` as an f32x64.
///
/// # Example
///
/// ```
/// assert_eq!(f32x64::splat(0.5), HALF);
/// ```
pub const SIMD_HALF: f32x64 = f32x64::from_array([
    0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
    0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
    0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
    0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
    0.5, 0.5, 0.5, 0.5,
]);

/// `TAU` as an f32x64.
///
/// # Example
///
/// ```
/// assert_eq!(f32x64::splat(TAU), SIMD_TAU);
/// ```
pub const SIMD_TAU: f32x64 = f32x64::from_array([
    TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU,
    TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU,
    TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU,
    TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU, TAU,
    TAU, TAU, TAU, TAU,
]);
