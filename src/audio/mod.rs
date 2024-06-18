use super::*;
use atomic::Atomic;
use bytemuck::NoUninit;
use compressor::Compressor;
use crossbeam_channel::Receiver;
use nannou_audio::*;
use std::sync::atomic::AtomicU32;
use std::time::Instant;
use thread_pool::{AudioThreadPool, AudioThreadPoolReferences, MAX_BLOCK_SIZE};

pub use effects::*;
pub use effects::{AudioEffect, StereoEffect};
pub use voice::{VoiceHandler, NUM_VOICES};

mod compressor;
pub mod effects;
mod envelope;
mod process;
mod sine;
mod thread_pool;
mod tri;
mod voice;

pub const CH_L: usize = 0;
pub const CH_R: usize = 1;

// Musical scales
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

/// The number of threads used for concurrent audio generation.
const NUM_AUDIO_THREADS: usize = 16;
/// The number of voices per `VoiceHandler`.
const VOICES_PER_HANDLER: usize = NUM_VOICES / NUM_AUDIO_THREADS;

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

/// Trait for SIMD oscillators.
pub trait SimdOscillator: std::fmt::Debug {
    fn set_frequency(&mut self, freq_hz: f32, sample_rate: f32);
    fn tick(&mut self) -> f32x2;
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
    // TODO: these don't need to be stored here, and can be moved into the
    // audio threads.
    voice_handlers: Vec<Arc<Mutex<VoiceHandler>>>,
    /// The audio voice buffers.
    voice_buffers: Vec<Arc<Mutex<Vec<f32x2>>>>,
    /// The counters for the number of active voices for each voice handler.
    voice_counters: Vec<Arc<AtomicU32>>,
    /// The buffers which were modified (i.e. written to) for this block.
    modified_buffers: Vec<Arc<AtomicBool>>,
    /// The audio voice thread pool.
    thread_pool: AudioThreadPool,

    /// The "main" audio buffer for the audio model, which uses SIMD values and
    /// is copied to the main buffer.
    main_buffer: Vec<f32x2>,

    callback_timer: Arc<Atomic<InstantTime>>,

    voice_counter: Arc<AtomicU32>,

    running: bool,
    compressor: Compressor,
    lp: Filter,
    hp: Filter,
    dsp_load: Arc<Atomic<f32>>,
}

impl Audio {
    /// Creates a new `AudioModel`.
    pub fn new(
        note_receiver: Receiver<NoteEvent>,
        voice_counter: Arc<AtomicU32>,
    ) -> Self {
        const { assert!(BUFFER_SIZE.is_power_of_two()) }
        const { assert!(NUM_AUDIO_THREADS.is_power_of_two()) }
        const { assert!(NUM_VOICES.is_power_of_two()) }

        let sr = SAMPLE_RATE as f32;
        let note_receiver = Arc::new(note_receiver);

        let voice_handlers: Vec<Arc<Mutex<VoiceHandler>>> = (0
            ..NUM_AUDIO_THREADS)
            .map(|_| {
                Arc::new(Mutex::new(VoiceHandler::new::<VOICES_PER_HANDLER>(
                    sr,
                )))
            })
            .collect();

        // note that this program only supports two channels, so we use f32x2 as
        // the sample type to represent both channels.
        let voice_buffers: Vec<Arc<Mutex<Vec<f32x2>>>> = (0..NUM_AUDIO_THREADS)
            .map(|_| Arc::new(Mutex::new(vec![f32x2::splat(0.0); BUFFER_SIZE])))
            .collect();

        let modified_buffers: Vec<Arc<AtomicBool>> = (0..NUM_AUDIO_THREADS)
            .map(|_| Arc::new(AtomicBool::new(false)))
            .collect();

        let voice_counters: Vec<Arc<AtomicU32>> = (0..NUM_AUDIO_THREADS)
            .map(|_| Arc::new(AtomicU32::new(0)))
            .collect();

        Self {
            sample_rate: SAMPLE_RATE,

            thread_pool: AudioThreadPool::build(
                &AudioThreadPoolReferences {
                    output_buffers: &voice_buffers,
                    voice_handlers: &voice_handlers,
                    voice_counters: &voice_counters,
                    modified_flags: &modified_buffers,
                },
                &note_receiver,
            )
            .expect("failed to create audio thread pool"),

            note_receiver,

            voice_handlers,
            voice_buffers,
            voice_counters,
            modified_buffers,

            main_buffer: vec![f32x2::splat(0.0); BUFFER_SIZE],

            callback_timer: Arc::new(Atomic::new(InstantTime(Instant::now()))),
            voice_counter,
            running: true,
            compressor: Compressor::new(NUM_CHANNELS, sr)
                .with_threshold_db(-18.0)
                .with_ratio(100.0)
                .with_knee_width(12.0),
            lp: Filter::new(sr)
                .with_type(FilterType::Lowpass)
                .with_freq(4000.0),
            hp: Filter::new(sr)
                .with_type(FilterType::Highpass)
                .with_freq(300.0),
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

    pub fn update_callback_timer(&self) {
        let timer = &self.callback_timer;

        if timer.load(Relaxed).elapsed().as_secs_f32() >= 0.0001 {
            timer.store(InstantTime(Instant::now()), Relaxed);
        }
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
        self.voice_buffers
            .iter()
            .for_each(|b| b.lock().fill(f32x2::splat(0.0)));
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

    /// Calculates amplitude in decibels from a SIMD vector of linear power
    /// levels.
    #[inline]
    pub fn level_to_db_simd(level: f32x2) -> f32x2 {
        const SIMD_20: f32x2 = f32x2::from_array([20.0, 20.0]);
        SIMD_20 * level.abs().log10()
    }

    /// Calculates the linear power level from a SIMD vector of amplitude (as
    /// decibels).
    #[inline]
    pub fn db_to_level_simd(db_value: f32x2) -> f32x2 {
        const SIMD_10: f32x2 = f32x2::from_array([10.0, 10.0]);
        const SIMD_20: f32x2 = f32x2::from_array([20.0, 20.0]);

        // there is no `powf()` implemented for SIMD types, but we can use
        // exp2(log2(x) * power) to achieve the same result. the below is
        // therefore equivalent to `SIMD_10.powf(db_value / SIMD_20)`.
        f32x2::exp2(f32x2::log2(SIMD_10) * (db_value / SIMD_20))

        // alternatively...
        // (SIMD_10.log2() * (db_value / SIMD_20)).exp2()
    }

    /// Calculates the linear power level from amplitude as decibels.
    #[inline]
    pub fn db_to_level(db_value: f32) -> f32 {
        10.0f32.powf(db_value / 20.0)
    }

    fn update_voice_counter(&self) {
        self.voice_counter.store(
            self.voice_counters.iter().map(|c| c.load(Relaxed)).sum(),
            Relaxed,
        );
    }
}

//
// Audio processing methods
//

impl Audio {
    /// Generates and processes new audio, and writes it to the provided
    /// `Buffer`.
    pub fn process(&mut self, buffer: &mut Buffer) {
        // if any of these buffers are locked before we call the voice thread
        // pool, then there's a scheduling error in the pool.
        for (i, buf) in self.voice_buffers.iter().enumerate() {
            debug_assert!(
                !buf.is_locked(),
                "voice buffer {i} was locked before dispatching voice threads"
            );
        }

        let any_executed = self.thread_pool.execute();

        self.sum_to_main_buf();

        if any_executed {
            self.process_fx();
        }

        self.copy_to_main_buffer(buffer);
        self.update_voice_counter();
    }

    /// Processes the internal FX on the main SIMD buffer.
    #[inline]
    fn process_fx(&mut self) {
        for sample in &mut self.main_buffer {
            *sample = self.hp.tick(*sample);
            *sample = self.compressor.tick(*sample);

            *sample = sample.simd_clamp(-SIMD_ONE, SIMD_ONE);
        }
    }

    /// Copies the contents of the main SIMD buffer to the main audio buffer.
    #[inline]
    fn copy_to_main_buffer(&mut self, buffer: &mut Buffer) {
        for (i, fr) in buffer.frames_mut().enumerate() {
            fr[CH_L] = self.main_buffer[i][CH_L];
            fr[CH_R] = self.main_buffer[i][CH_R];
        }

        self.main_buffer.fill(f32x2::splat(0.0));
    }

    /// Sums the contents of the modified voices buffers to the main SIMD
    /// buffer.
    #[inline]
    fn sum_to_main_buf(&mut self) {
        for (buf, flag) in self
            .modified_buffers
            .iter()
            .filter(|f| f.load(Relaxed))
            .enumerate()
        {
            let mut buf = self.voice_buffers[buf].lock();

            for (i, sample) in self.main_buffer.iter_mut().enumerate() {
                *sample += buf[i];
            }

            // reset the flag and the buffer for the next frame
            flag.store(false, Relaxed);
        }
    }
}
