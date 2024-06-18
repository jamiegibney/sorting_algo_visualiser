use super::*;

/// A simple ramp amplitude envelope.
#[derive(Debug)]
pub struct AmpEnvelope {
    data: &'static [f32],
    read_pos: usize,
    simd: f32x2,
}

impl AmpEnvelope {
    /// Creates a new envelope.
    pub const fn new(envelope_data: &[u8]) -> Self {
        let f32_size = std::mem::size_of::<f32>();

        Self {
            data: unsafe {
                std::slice::from_raw_parts(
                    envelope_data.as_ptr().cast::<f32>(),
                    envelope_data.len() / f32_size,
                )
            },
            read_pos: 0,
            simd: f32x2::from_array([0.0, 0.0]),
        }
    }

    /// Returns the next envelope sample. Returns `None` if the envelope has
    /// reached its end point.
    pub fn next(&mut self) -> Option<f32> {
        if !self.is_active() {
            return None;
        }

        let pos = self.read_pos;
        self.read_pos += 1;

        Some(self.data[pos])
    }

    /// Returns the next envelope sample as a `f32x2` SIMD type. If the envelope
    /// has reached its end, then `Self::next_simd().as_array() == &[0.0, 0.0]`.
    #[inline]
    pub fn next_simd(&mut self) -> f32x2 {
        let val = self.next().unwrap_or_default();

        self.simd[CH_L] = val;
        self.simd[CH_R] = val;

        self.simd
    }

    /// Whether the envelope is active.
    pub const fn is_active(&self) -> bool {
        self.read_pos < self.data.len()
    }
}
