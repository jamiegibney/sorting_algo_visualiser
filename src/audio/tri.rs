use super::*;

#[derive(Debug)]
pub struct TriOscSimd {
    inc: f32x64,
    phase: f32x64,
}

impl TriOscSimd {
    pub fn new(freq_hz: f32, sample_rate: f32) -> Self {
        debug_assert!(0.0 < freq_hz && freq_hz <= sample_rate * 0.5);

        let inc = freq_hz / sample_rate;
        let inc = f32x64::splat(inc);

        Self { inc, phase: inc * SIMD_STAGGER }
    }
}

impl SimdOscillator for TriOscSimd {
    fn set_frequency(&mut self, freq_hz: f32, sample_rate: f32) {
        debug_assert!(0.0 < freq_hz && freq_hz <= sample_rate * 0.5);

        let inc = freq_hz / sample_rate;
        self.inc = f32x64::splat(inc);

        let stagger = f32x64::from_array(std::array::from_fn(|i| i as f32));
        self.phase = self.inc * stagger;
    }

    fn tick(&mut self) -> f32x64 {
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
