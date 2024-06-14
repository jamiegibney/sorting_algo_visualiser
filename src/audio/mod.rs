use super::*;
use atomic::Atomic;
use bytemuck::NoUninit;
use crossbeam_channel::Receiver;
use nannou_audio::*;
use std::sync::atomic::AtomicU32;
use std::time::Instant;

mod compressor;
mod effects;
mod envelope;
mod process;
mod sine;
mod tri;
mod voice;

use compressor::Compressor;
use effects::*;
use effects::{AudioEffect, StereoEffect};
pub use voice::{VoiceHandler, NUM_VOICES};

pub const MAJ_PENT_SCALE: [f32; 5] = [0.0, 2.0, 4.0, 7.0, 9.0];
pub const MIN_PENT_SCALE: [f32; 5] = [0.0, 3.0, 5.0, 7.0, 10.0];
pub const MAJOR_SCALE: [f32; 7] = [0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0];
pub const MINOR_SCALE: [f32; 7] = [0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0];

/// The app's audio sample rate.
pub const SAMPLE_RATE: u32 = 48000;
/// The number of audio channels.
pub const NUM_CHANNELS: usize = 2;
/// The app's audio buffer size.
pub const BUFFER_SIZE: usize = 1 << 8; // 256

/// Trait for oscillators.
pub trait Oscillator: std::fmt::Debug {
    fn set_frequency(&mut self, freq_hz: f32, sample_rate: f32);
    fn tick(&mut self) -> f32;
}

/// An atomic-compatible wrapper around an `Instant`.
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

    voice_counter: Arc<AtomicU32>,

    running: bool,
    compressor: Compressor,
}

impl Audio {
    /// Creates a new `AudioModel`.
    pub fn new(
        note_receiver: Receiver<NoteEvent>,
        voice_counter: Arc<AtomicU32>,
    ) -> Self {
        Self {
            note_receiver,
            sample_rate: SAMPLE_RATE,
            voice_handler: VoiceHandler::new(SAMPLE_RATE as f32),
            callback_timer: Arc::new(Atomic::new(InstantTime(Instant::now()))),
            voice_counter,
            running: true,
            compressor: Compressor::new(NUM_CHANNELS, SAMPLE_RATE as f32)
                .with_threshold_db(-18.0)
                .with_ratio(50.0)
                .with_knee_width(8.0),
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

    /// Updates the voice counter with the current number of active voices.
    pub fn update_voice_counter(&self) {
        self.voice_counter.store(
            self.voice_handler.num_active() as u32,
            atomic::Ordering::Relaxed,
        );
    }

    /// Converts the `AudioModel` into a CPAL audio stream.
    pub fn into_stream(self) -> Stream<Self> {
        let audio_host = Host::new();
        let sr = self.sample_rate;

        let stream = audio_host
            .new_output_stream(self)
            .render(process::process)
            .channels(NUM_CHANNELS)
            .sample_rate(sr)
            .frames_per_buffer(BUFFER_SIZE)
            .build()
            .unwrap();

        stream.play().unwrap();

        stream
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    /// Calculates the frequency value of the provided MIDI note value, relative
    /// to 440 Hz.
    #[inline]
    pub fn note_to_freq(note_value: f32) -> f32 {
        const TUNING_FREQ_HZ: f32 = 440.0;

        ((note_value - 69.0) / 12.0).exp2() * TUNING_FREQ_HZ
    }

    pub fn quantize_to_scale(scale: &[f32], note: f32, root: f32) -> f32 {
        let mut lower = root;

        while !(lower..=(lower + 12.0)).contains(&note) {
            lower += if note > lower { 12.0 } else { -12.0 };
        }

        let mut min = f32::MAX;
        let mut idx = 0;

        for (i, &int) in scale.iter().enumerate() {
            let cur = lower + int;
            let val = (note - cur).abs();

            if val < min {
                min = val;
                idx = i;
            }
        }

        lower + scale[idx]
    }

    /// Calculates amplitude in decibels from a linear power level.
    #[inline]
    pub fn level_to_db(level: f32) -> f32 {
        20.0 * level.abs().log10()
    }

    /// Calculates the linear power level from amplitude as decibels.
    #[inline]
    pub fn db_to_level(db_value: f32) -> f32 {
        10.0f32.powf(db_value / 20.0)
    }
}
