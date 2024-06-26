use std::fmt::Debug;

use super::*;
use nannou_audio::Buffer;

pub mod ballistics_filter;
pub mod compressor;
pub mod filter;

pub use ballistics_filter::BallisticsFilter;
pub use filter::{Filter, FilterType};
pub use compressor::Compressor;

/// Trait for SIMD audio processing effects, which *only* support two channels.
pub trait SimdAudioEffect: Debug + Clone {
    fn tick(&mut self, sample: f32x2) -> f32x2;
    fn sample_rate(&self) -> f32;
}

/// Trait for audio processing effects.
pub trait AudioEffect: Debug + Clone {
    /// Processes a single sample of audio.
    fn tick(&mut self, channel: usize, sample: f32) -> f32;
    /// The sample rate of this audio effect.
    fn sample_rate(&self) -> f32;
    /// The number of channels this audio effect supports.
    fn num_channels(&self) -> usize;

    /// Processes a block of audio samples.
    fn process_block(&mut self, block: &mut Buffer) {
        for fr in block.frames_mut() {
            for (ch, smp) in fr.iter_mut().enumerate() {
                *smp = self.tick(ch, *smp);
            }
        }
    }
}

/// A stereo wrapper of two mono effects.
#[derive(Debug, Clone)]
pub struct StereoEffect<E: AudioEffect> {
    /// The effect for the left stereo channel.
    pub l: E,
    /// The effect for the right stereo channel.
    pub r: E,
}

impl<E: AudioEffect> StereoEffect<E> {
    /// Creates a new `StereoEffect` from two `AudioEffect`s.
    pub const fn new(l: E, r: E) -> Self {
        Self { l, r }
    }

    pub fn splat(effect: E) -> Self {
        Self { l: effect.clone(), r: effect }
    }

    /// Unwraps the stored audio effects.
    pub fn unwrap(self) -> (E, E) {
        (self.l, self.r)
    }
}

impl<E: AudioEffect> AudioEffect for StereoEffect<E> {
    fn tick(&mut self, channel: usize, sample: f32) -> f32 {
        match channel {
            0 => self.l.tick(channel, sample),
            1 => self.r.tick(channel, sample),
            _ => sample,
        }
    }

    fn sample_rate(&self) -> f32 {
        self.l.sample_rate()
    }

    fn num_channels(&self) -> usize {
        2
    }
}
