use super::*;
use atomic::Atomic;
use bytemuck::NoUninit;
use nannou_audio::*;
use std::sync::mpsc::Receiver;
use std::time::Instant;

mod effects;
mod envelope;
mod process;
mod sine;
mod voice;

use effects::*;
use voice::VoiceHandler;

/// The default sample rate.
pub const SAMPLE_RATE: u32 = 48000;
/// The default buffer size.
pub const BUFFER_SIZE: usize = 1 << 8; // 256

#[derive(Debug, Clone, Copy)]
pub struct InstantTime(Instant);

unsafe impl NoUninit for InstantTime {}
impl std::ops::Deref for InstantTime {
    type Target = Instant;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The app's audio state.
#[derive(Debug)]
pub struct Audio {
    /// A receiver for income audio note events.
    note_receiver: Receiver<NoteEvent>,
    /// The sample rate.
    sample_rate: u32,
    /// The audio voice handler.
    voice_handler: VoiceHandler,

    callback_timer: Arc<Atomic<InstantTime>>,
}

impl Audio {
    /// Creates a new `AudioModel`.
    pub fn new(note_receiver: Receiver<NoteEvent>) -> Self {
        Self {
            note_receiver,
            sample_rate: SAMPLE_RATE,
            voice_handler: VoiceHandler::new(SAMPLE_RATE as f32),
            callback_timer: Arc::new(Atomic::new(InstantTime(Instant::now()))),
        }
    }

    /// Returns a reference to the audio note receiver.
    pub const fn note_receiver(&self) -> &Receiver<NoteEvent> {
        &self.note_receiver
    }

    /// Returns a reference to the callback timer.
    pub const fn callback_timer(&self) -> &Arc<Atomic<InstantTime>> {
        &self.callback_timer
    }

    /// Maps an average sorting position to frequency in Hz.
    pub fn map_pos_to_freq(average_pos: f32) -> f32 {
        const MIN_FREQ: f32 = 80.0;
        const MAX_FREQ: f32 = 1300.0;
        let x = average_pos.clamp(0.0, 1.0).powi(4);

        (MAX_FREQ - MIN_FREQ).mul_add(x, MIN_FREQ)
    }

    /// Converts the `AudioModel` into a CPAL audio stream.
    pub fn into_stream(self) -> Stream<Self> {
        let audio_host = Host::new();
        let sr = self.sample_rate;

        let stream = audio_host
            .new_output_stream(self)
            .render(process::process)
            .channels(2)
            .sample_rate(sr)
            .frames_per_buffer(BUFFER_SIZE)
            .build()
            .unwrap();

        stream.play().unwrap();

        stream
    }
}
