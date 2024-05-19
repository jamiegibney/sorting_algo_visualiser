use super::*;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Clone, Copy, Debug)]
pub struct NoteEvent {
    /// The frequency of this note.
    pub freq: f32,
    /// The amplitude of this note.
    pub amp: f32,
    /// The buffer sample offset.
    pub timing: u32,
}

impl NoteEvent {
    /// Creates a new `NoteEvent`.
    pub const fn new(freq: f32, amp: f32, timing: u32) -> Self {
        Self { freq, amp, timing }
    }

    /// Returns the frequency of this event in Hz.
    pub const fn freq(self) -> f32 {
        self.freq
    }

    /// Returns the amplitude of this event.
    pub const fn amp(&self) -> f32 {
        self.amp
    }

    /// Returns the sample offset of this event.
    pub const fn sample_offset(self) -> u32 {
        self.timing
    }
}
