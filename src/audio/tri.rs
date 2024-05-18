use super::*;

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
