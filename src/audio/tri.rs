use super::*;

#[derive(Debug)]
pub struct TriOscSimd {
    inc: f32x2,
    phase: f32x2,
}

impl TriOscSimd {
    pub fn new(freq_hz: f32, sample_rate: f32) -> Self {
        debug_assert!(0.0 < freq_hz && freq_hz <= sample_rate * 0.5);

        let inc = freq_hz / sample_rate;

        Self { inc: f32x2::splat(inc), phase: f32x2::splat(0.0) }
    }
}

impl SimdOscillator for TriOscSimd {
    fn set_frequency(&mut self, freq_hz: f32, sample_rate: f32) {
        debug_assert!(0.0 < freq_hz && freq_hz <= sample_rate * 0.5);

        let inc = freq_hz / sample_rate;

        self.inc[CH_L] = inc;
        self.inc[CH_R] = inc;
    }

    #[inline]
    fn tick(&mut self) -> f32x2 {
        let x = self.phase.mul_add(SIMD_TWO, -SIMD_ONE);
        self.phase = (self.phase + self.inc).fract();

        (x.abs() - SIMD_HALF) * SIMD_TWO
    }
}

/// A simple triangle wave oscillator.
#[derive(Debug)]
pub struct TriOsc {
    inc: f32,
    phase: f32,
}

impl TriOsc {
    pub fn new(freq_hz: f32, sample_rate: f32) -> Self {
        debug_assert!(0.0 < freq_hz && freq_hz <= sample_rate * 0.5);

        Self { inc: freq_hz / sample_rate, phase: 0.0 }
    }
}

impl Oscillator for TriOsc {
    fn set_frequency(&mut self, freq_hz: f32, sample_rate: f32) {
        debug_assert!(0.0 < freq_hz && freq_hz <= sample_rate * 0.5);
        self.inc = freq_hz / sample_rate;
    }

    fn tick(&mut self) -> f32 {
        let x = self.phase.mul_add(2.0, -1.0);
        self.phase = (self.phase + self.inc).fract();

        (x.abs() - 0.5) * 2.0
    }
}
