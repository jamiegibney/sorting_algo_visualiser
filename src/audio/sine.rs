use super::*;

#[derive(Debug)]
pub struct SineOscSimd {
    inc: f32x2,
    phase: f32x2,
}

impl SineOscSimd {
    pub fn new(freq_hz: f32, sample_rate: f32) -> Self {
        debug_assert!(0.0 < freq_hz && freq_hz <= sample_rate * 0.5);

        Self {
            inc: f32x2::splat(freq_hz / sample_rate),
            phase: f32x2::splat(0.0),
        }
    }
}

impl SimdOscillator for SineOscSimd {
    fn set_frequency(&mut self, freq_hz: f32, sample_rate: f32) {
        debug_assert!(0.0 < freq_hz && freq_hz <= sample_rate * 0.5);

        let inc = freq_hz / sample_rate;

        self.inc[CH_L] = inc;
        self.inc[CH_R] = inc;
    }

    #[inline]
    fn tick(&mut self) -> f32x2 {
        let output = (self.phase * SIMD_TAU).sin();
        self.phase = (self.phase + self.inc).fract();

        output
    }
}

/// A simple sine wave oscillator.
#[derive(Debug)]
pub struct SineOsc {
    inc: f32,
    phase: f32,
}

impl SineOsc {
    /// Creates a new sine wave oscillator at `freq_hz` Hz.
    pub fn new(freq_hz: f32, sample_rate: f32) -> Self {
        debug_assert!(0.0 < freq_hz && freq_hz <= sample_rate * 0.5);

        Self { inc: freq_hz / sample_rate, phase: 0.0 }
    }
}

impl Oscillator for SineOsc {
    /// Sets the frequency of the oscillator.
    fn set_frequency(&mut self, new_freq: f32, sample_rate: f32) {
        self.inc = new_freq / sample_rate;
    }

    /// Returns the next sample from the oscillator, updating its state.
    fn tick(&mut self) -> f32 {
        let output = (self.phase * TAU).sin();
        self.phase = (self.phase + self.inc).fract();

        output
    }
}
