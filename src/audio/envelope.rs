use super::*;

static ENVELOPE_BYTES: &[u8; 134784] = include_bytes!("envelope_data");

/// A simple ramp amplitude envelope.
#[derive(Debug)]
pub struct AmpEnvelope {
    data: &'static [f32],
    read_pos: usize,
    is_active: bool,
}

impl AmpEnvelope {
    /// Creates a new envelope of `len_secs` seconds.
    pub fn with_length(len_secs: f32, sample_rate: f32) -> Self {
        let num_samples = (len_secs * sample_rate).ceil() as usize;

        let f32_size = std::mem::size_of::<f32>();

        Self {
            data: unsafe {
                std::slice::from_raw_parts(
                    ENVELOPE_BYTES.as_ptr().cast::<f32>(),
                    ENVELOPE_BYTES.len() / f32_size,
                )
            },
            read_pos: 0,
            is_active: true,
        }
    }

    /// Returns the next envelope sample. Returns `None` if the envelope has reached
    /// its end point.
    pub fn next(&mut self) -> Option<f32> {
        if !self.is_active {
            return None;
        }

        let pos = self.read_pos;
        self.read_pos += 1;

        if self.read_pos == self.data.len() {
            self.is_active = false;
        }

        Some(self.data[pos])
    }

    /// Whether the envelope is active or not.
    pub const fn is_active(&self) -> bool {
        self.is_active
    }
}
