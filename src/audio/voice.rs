use super::*;
use envelope::AmpEnvelope;
use nannou_audio::Buffer;
use sine::SineOsc;

/// The maximum number of polyphonic audio voices.
pub const NUM_VOICES: usize = 64;
/// The length of each voice's amplitude envelope.
const ENVELOPE_LENGTH: f32 = 1.0;

/// A single voice.
#[derive(Debug)]
struct Voice {
    id: u64,
    sample_rate: f32,
    osc: SineOsc,
    freq: f32,
    envelope: AmpEnvelope,
    pan: f32,
}

impl Voice {
    /// Sets the frequency of the voice.
    pub fn set_frequency(&mut self, new_freq: f32) {
        self.freq = new_freq;
        self.osc.set_frequency(self.freq, self.sample_rate);
    }

    /// Returns `true` when the voice has finished producing audio.
    pub const fn is_finished(&self) -> bool {
        !self.envelope.is_active()
    }

    pub fn balance(&self) -> (f32, f32) {
        let left = (1.0 - self.pan).clamp(0.0, 1.0);
        let right = 1.0 - (2.0 - self.pan).clamp(0.0, 1.0);

        (left, right)
    }
}

/// The options available for when all voices are in use.
#[derive(Clone, Copy, Debug, Default)]
pub enum OverrideVoiceBehavior {
    /// Replace the oldest voice.
    #[default]
    ReplaceOldest,
    /// Replace the voice with the lowest frequency.
    ReplaceLowest,
    /// Do not replace any active voices.
    DoNotReplace,
}

/// The voice handler for managing polyphony.
#[derive(Debug)]
pub struct VoiceHandler {
    /// All voices.
    voices: [Option<Voice>; NUM_VOICES],
    /// The sample rate.
    sample_rate: f32,
    /// A counter for keeping track of old voices.
    id_counter: u64,
    /// The behavior for overriding voices when all are in use.
    override_behavior: OverrideVoiceBehavior,
}

impl VoiceHandler {
    /// An empty voice.
    const EMPTY_VOICE: Option<Voice> = None;

    /// Creates a new `VoiceHandler`.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            voices: [Self::EMPTY_VOICE; NUM_VOICES],
            sample_rate,
            id_counter: 0,
            override_behavior: OverrideVoiceBehavior::default(),
        }
    }

    /// Sets the voice override behavior for the `VoiceHandler`.
    pub fn set_override_behavior(
        &mut self,
        new_behavior: OverrideVoiceBehavior,
    ) {
        self.override_behavior = new_behavior;
    }

    /// Processes a block of audio.
    pub fn process_block(
        &mut self,
        buffer: &mut Buffer,
        block_start: usize,
        block_end: usize,
        gain: [f32; super::process::MAX_BLOCK_SIZE],
    ) {
        let block_len = block_end - block_start;

        for voice in self.voices.iter_mut().flatten() {
            for (i, sample) in (block_start..block_end).enumerate() {
                let voice_amp = voice.envelope.next().unwrap_or(0.0);
                let out = voice.osc.tick() * voice_amp * gain[i];
                let (l, r) = voice.balance();

                buffer[sample * 2] += out * l;
                buffer[sample * 2 + 1] += out * r;
            }
        }
    }

    /// Starts a new voice.
    pub fn new_voice(&mut self, freq: f32) {
        // println!("Spawning new voice at frequency {freq} Hz");

        if let Some(free_idx) = self.voices.iter().position(Option::is_none) {
            // self.create_voice() is inlined here in case no voices are free (in
            // which case no new voice is ever created)
            self.voices[free_idx] = Some(self.create_voice(freq));
            return;
        }

        use OverrideVoiceBehavior::*;

        if matches!(self.override_behavior, DoNotReplace) {
            return;
        }

        let new_voice = self.create_voice(freq);

        match self.override_behavior {
            ReplaceOldest => {
                let oldest = unsafe {
                    self.voices
                        .iter_mut()
                        .min_by_key(|v| v.as_ref().unwrap_unchecked().id)
                        .unwrap_unchecked()
                };

                *oldest = Some(new_voice);
            }
            ReplaceLowest => {
                let lowest = unsafe {
                    self.voices
                        .iter_mut()
                        .min_by(|v1, v2| {
                            let v1_freq = v1.as_ref().unwrap_unchecked().freq;
                            let v2_freq = v2.as_ref().unwrap_unchecked().freq;
                            v1_freq.total_cmp(&v2_freq)
                        })
                        .unwrap_unchecked()
                };

                *lowest = Some(new_voice);
            }
            DoNotReplace => (),
        }
    }

    /// Frees any active voices which are finished.
    pub fn free_finished_voices(&mut self) {
        for voice in &mut self.voices {
            match voice {
                Some(v) if v.is_finished() => *voice = Self::EMPTY_VOICE,
                _ => (),
            }
        }
    }

    /// Immediately kills all active voices.
    pub fn kill_active_voices(&mut self) {
        self.voices.iter_mut().for_each(|v| *v = Self::EMPTY_VOICE);
    }

    /// Returns `true` if any voice is active.
    pub fn any_active(&self) -> bool {
        self.voices.iter().any(Option::is_some)
    }

    /// Returns the number of active voices.
    pub fn num_active(&self) -> usize {
        self.voices.iter().filter(|v| v.is_some()).count()
    }

    /// Returns a new voice.
    fn create_voice(&mut self, freq: f32) -> Voice {
        let v = Voice {
            id: self.next_voice_id(),
            sample_rate: self.sample_rate,
            osc: SineOsc::new(freq, self.sample_rate),
            freq,
            envelope: AmpEnvelope::with_length(
                ENVELOPE_LENGTH, self.sample_rate,
            ),
            pan: nannou::rand::random_f32() * 2.0,
        };

        v
    }

    /// Gets the next voice ID.
    fn next_voice_id(&mut self) -> u64 {
        self.id_counter = self.id_counter.wrapping_add(1);
        self.id_counter
    }
}