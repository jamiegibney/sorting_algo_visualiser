use std::fmt::Debug;

use super::*;
use nannou_audio::Buffer;

mod delay;
mod ring_buf;
mod verb;

use delay::DelayLine;
use ring_buf::RingBuffer;
use verb::Reverb;

/// Trait for audio processing effects.
pub trait AudioEffect: Debug {
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
#[derive(Debug)]
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
