use super::*;
use atomic::Atomic;
use bytemuck::NoUninit;
use crossbeam_channel::Receiver;
use nannou_audio::*;
use std::sync::atomic::AtomicU32;
use std::time::Instant;
use thread_pool::ThreadPool;

mod compressor;
pub mod effects;
mod envelope;
mod process;
mod sine;
mod tri;
mod voice;

use compressor::Compressor;
pub use effects::*;
pub use effects::{AudioEffect, StereoEffect};
use process::MAX_BLOCK_SIZE;
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
pub const BUFFER_SIZE: usize = 1 << 9; // 512

const VOICES_PER_HANDLER: usize = NUM_VOICES / NUM_AUDIO_THREADS;
/// The number of threads used for concurrent audio generation.
const NUM_AUDIO_THREADS: usize = 8;

/// The types of oscillators available.
#[derive(Clone, Copy, Debug, Default)]
pub enum OscillatorType {
    #[default]
    Sine,
    Tri,
}

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
    note_receiver: Arc<Receiver<NoteEvent>>,
    /// The sample rate.
    sample_rate: u32,

    /// The audio voice handlers.
    voice_handlers: Vec<Arc<Mutex<VoiceHandler>>>,
    /// The audio voice buffers.
    voice_buffers: Vec<Arc<Mutex<Vec<f32>>>>,
    /// The buffers which were modified (i.e. written to) for this block.
    modified_voice_buffers: Vec<Arc<AtomicBool>>,
    /// The audio voice thread pool.
    thread_pool: ThreadPool,

    callback_timer: Arc<Atomic<InstantTime>>,

    voice_counter: Arc<AtomicU32>,

    running: bool,
    compressor: Compressor,
    low_pass: StereoEffect<Lowpass>,
    dsp_load: Arc<Atomic<f32>>,
}

impl Audio {
    /// Creates a new `AudioModel`.
    pub fn new(
        note_receiver: Receiver<NoteEvent>,
        voice_counter: Arc<AtomicU32>,
    ) -> Self {
        const {
            assert!(BUFFER_SIZE.is_power_of_two());
        }
        const {
            assert!(NUM_AUDIO_THREADS.is_power_of_two());
        }
        const {
            assert!(NUM_VOICES.is_power_of_two());
        }

        let sr = SAMPLE_RATE as f32;

        Self {
            note_receiver: Arc::new(note_receiver),
            sample_rate: SAMPLE_RATE,

            voice_handlers: (0..NUM_AUDIO_THREADS)
                .map(|_| {
                    Arc::new(Mutex::new(
                        VoiceHandler::new::<VOICES_PER_HANDLER>(sr),
                    ))
                })
                .collect(),

            voice_buffers: (0..NUM_AUDIO_THREADS)
                .map(|_| {
                    Arc::new(Mutex::new(vec![0.0; NUM_CHANNELS * BUFFER_SIZE]))
                })
                .collect(),

            modified_voice_buffers: (0..NUM_AUDIO_THREADS)
                .map(|_| Arc::new(AtomicBool::new(false)))
                .collect(),

            thread_pool: {
                let names: Vec<String> = (0..NUM_AUDIO_THREADS)
                    .map(|i| format!("audio thread #{i}"))
                    .collect();
                let name_refs: Vec<&str> =
                    names.iter().map(String::as_str).collect();

                ThreadPool::build(
                    NUM_AUDIO_THREADS,
                    Some(thread_priority::ThreadPriority::Max),
                    Some(&name_refs),
                )
                .expect("failed to allocate audio threads")
            },

            callback_timer: Arc::new(Atomic::new(InstantTime(Instant::now()))),
            voice_counter,
            running: true,
            compressor: Compressor::new(NUM_CHANNELS, sr)
                .with_threshold_db(-18.0)
                .with_ratio(100.0)
                .with_knee_width(12.0),
            low_pass: StereoEffect::splat(Lowpass::new(sr).with_freq(8000.0)),
            dsp_load: Arc::new(Atomic::new(0.0)),
        }
    }

    /// Returns a reference to the audio note receiver.
    pub const fn note_receiver(&self) -> &Arc<Receiver<NoteEvent>> {
        &self.note_receiver
    }

    /// Returns a reference to the callback timer.
    pub const fn callback_timer(&self) -> &Arc<Atomic<InstantTime>> {
        &self.callback_timer
    }

    /// Returns a reference to the DSP load level.
    pub const fn dsp_load(&self) -> &Arc<Atomic<f32>> {
        &self.dsp_load
    }

    /// Updates the voice counter with the current number of active voices.
    pub fn update_voice_counter(&self) {
        // self.voice_counter.store(
        //     self.voice_handler.num_active() as u32,
        //     atomic::Ordering::Relaxed,
        // );
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

    /// Processes a buffer "`buf`" by applying `cb` to each sample.
    ///
    /// - The first argument to `cb` is the current audio channel,
    /// - The second argument to `cb` is a mutable reference to the current
    ///   sample.
    pub fn process_buffer(
        buf: &mut Buffer,
        mut cb: impl FnMut(usize, &mut f32),
    ) {
        for fr in buf.frames_mut() {
            for (ch, smp) in fr.iter_mut().enumerate() {
                cb(ch, smp);
            }
        }
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

impl Audio {
    /// Generates voices based on the incoming audio note events.
    pub fn process_voices(&mut self, buffer: &mut Buffer) -> bool {
        let buffer_len = buffer.len_frames();

        for i in 0..NUM_AUDIO_THREADS {
            // TODO: don't execute the voice generation if it's not needed
            let receiver = Arc::clone(self.note_receiver());
            let handler = Arc::clone(&self.voice_handlers[i]);
            let buffer = Arc::clone(&self.voice_buffers[i]);
            let flag = Arc::clone(&self.modified_voice_buffers[i]);

            self.thread_pool.execute(move || {
                let mut next_event = receiver.try_recv().ok();

                let mut handler = handler.lock();

                if next_event.is_none() && !handler.any_active() {
                    return;
                }

                let mut block_start = 0;
                let mut block_end = MAX_BLOCK_SIZE.min(buffer_len);

                // handle polyphonic voices
                while block_start < buffer_len {
                    'events: loop {
                        match next_event {
                            // if we've snapped the block to an event
                            Some(event)
                                if (event.sample_offset() as usize)
                                    <= block_start =>
                            {
                                handler.new_voice(event);
                                next_event = receiver.try_recv().ok();
                            }
                            // if an event is within this block, snap to the
                            // event
                            Some(event)
                                if (event.sample_offset() as usize)
                                    < block_end =>
                            {
                                block_end = event.sample_offset() as usize;
                                break 'events;
                            }
                            // if no new events are available
                            _ => break 'events,
                        }
                    }

                    // TODO(jamiegibney): master gain control?
                    let mut gain = [0.08; MAX_BLOCK_SIZE];

                    // process voices and clean any which are finished
                    handler.process_block(
                        &mut buffer.lock(),
                        block_start,
                        block_end,
                        gain,
                    );
                    handler.free_finished_voices();

                    block_start = block_end;
                    block_end = (block_end + MAX_BLOCK_SIZE).min(buffer_len);
                }

                flag.store(true, Relaxed);

                drop(handler);
            });
        }

        self.thread_pool.block_until_free();

        let mut num_modified = 0;

        // sum modified buffers
        for (i, flag) in self
            .modified_voice_buffers
            .iter()
            .filter(|f| f.load(Relaxed))
            .enumerate()
        {
            num_modified += 1;

            let mut buf = self.voice_buffers[i].lock();
            for (i, sample) in buffer.iter_mut().enumerate() {
                *sample += buf[i];
            }

            buf.fill(0.0);
        }

        // dbg!(self.voice_counter.load(Relaxed));
        // dbg!(num_modified);

        self.voice_counter.store(
            self.voice_handlers
                .iter()
                .map(|h| h.lock().num_active() as u32)
                .sum(),
            Relaxed,
        );

        // dbg!(self.voice_counter.load(Relaxed));
        // dbg!(num_modified);

        if num_modified == 0 {
            return false;
        }

        self.modified_voice_buffers
            .iter_mut()
            .for_each(|f| f.store(false, Relaxed));

        true
    }
}
