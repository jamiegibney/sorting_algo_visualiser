use super::*;

/// A simple ramp amplitude envelope.
#[derive(Debug)]
pub struct AmpEnvelope {
    data: &'static [f32],
    read_pos: usize,
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
        }
    }

    /// Returns the next envelope sample. Returns `None` if the envelope has reached
    /// its end point.
    pub fn next(&mut self) -> Option<f32> {
        if !self.is_active() {
            return None;
        }

        let pos = self.read_pos;
        self.read_pos += 1;

        Some(self.data[pos])
    }

    /// Whether the envelope is active.
    pub const fn is_active(&self) -> bool {
        self.read_pos < self.data.len()
    }
}
