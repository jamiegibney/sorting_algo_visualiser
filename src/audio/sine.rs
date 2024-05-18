//! A sine wave oscillator.
use std::f32::consts::TAU;

/// A simple sine wave oscillator.
#[derive(Debug)]
pub struct SineOsc {
    inc: f32,
    phase: f32,
}

impl SineOsc {
    /// Creates a new sine wave oscillator at `freq_hz` Hz.
    pub fn new(freq_hz: f32, sample_rate: f32) -> Self {
        // println!(
        //     "creating new sine oscillator at {freq_hz} hz (phase increment: {})",
        //     freq_hz / sample_rate
        // );
        Self { inc: freq_hz / sample_rate, phase: 0.0 }
    }

    /// Sets the frequency of the oscillator.
    pub fn set_frequency(&mut self, new_freq: f32, sample_rate: f32) {
        self.inc = new_freq / sample_rate;
    }

    /// Returns the next sample from the oscillator, updating its state.
    pub fn tick(&mut self) -> f32 {
        let output = (self.phase * TAU).sin();
        self.phase = (self.phase + self.inc).fract();

        output
    }
}
