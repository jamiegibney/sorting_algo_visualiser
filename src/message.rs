use super::*;

/// An audio note event, used to create new voices.
#[derive(Clone, Copy, Debug)]
pub struct NoteEvent {
    /// The oscillator type of this note.
    pub osc: OscillatorType,
    /// The frequency of this note.
    pub freq: f32,
    /// The amplitude of this note.
    pub amp: f32,
    /// The buffer sample offset.
    pub timing: u32,
    /// The panning amount of this note.
    pub pan: f32,
}

impl NoteEvent {
    /// Creates a new `NoteEvent`.
    pub fn new(freq: f32, amp: f32, timing: u32, pan: f32) -> Self {
        Self {
            osc: OscillatorType::default(),
            freq,
            amp,
            timing,
            pan: pan.clamp(-1.0, 1.0),
        }
    }

    /// Provides an oscillator type to this note.
    pub const fn with_type(mut self, osc_type: OscillatorType) -> Self {
        self.osc = osc_type;
        self
    }

    /// Returns the oscillator type of this event.
    pub const fn osc(self) -> OscillatorType {
        self.osc
    }

    /// Returns the frequency of this event in Hz.
    pub const fn freq(self) -> f32 {
        self.freq
    }

    /// Returns the amplitude of this event.
    pub const fn amp(self) -> f32 {
        self.amp
    }

    /// Returns the sample offset of this event.
    pub const fn sample_offset(self) -> u32 {
        self.timing
    }

    pub const fn pan(self) -> f32 {
        self.pan
    }
}
