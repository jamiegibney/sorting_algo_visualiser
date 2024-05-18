use super::*;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Clone, Copy, Debug)]
pub struct NoteEvent {
    /// The average sorting position between `0.0` and `1.0`.
    pub average_pos: f32,
    /// The buffer sample offset.
    pub timing: u32,
}

impl NoteEvent {
    /// Creates a new `NoteEvent`.
    pub const fn new(average_pos: f32, timing: u32) -> Self {
        Self { average_pos, timing }
    }

    /// Returns the average sorting position of this event.
    pub const fn average_pos(self) -> f32 {
        self.average_pos
    }

    /// Returns the sample offset of this event.
    pub const fn sample_offset(self) -> u32 {
        self.timing
    }
}
