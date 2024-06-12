use crate::prelude::*;
use std::{os::unix::fs::DirBuilderExt, rc::Rc};

#[derive(Debug)]
struct AudioState {
    callback_timer: Arc<Atomic<InstantTime>>,
    note_event_sender: Sender<NoteEvent>,
}

#[derive(Debug)]
pub struct Player {
    capture: Option<SortCapture>,

    playback_time: f32,
    speed_mult: f32,

    is_playing: bool,

    audio: AudioState,

    ops_last_frame: Rc<[SortOperation]>,
}

impl Player {
    pub const DEFAULT_PLAYBACK_TIME: f32 = 2.0;

    pub fn new(
        len: usize,
        note_event_sender: Sender<NoteEvent>,
        callback_timer: Arc<Atomic<InstantTime>>,
    ) -> Self {
        Self {
            capture: None,

            playback_time: Self::DEFAULT_PLAYBACK_TIME,
            speed_mult: 1.0,

            is_playing: false,

            audio: AudioState { callback_timer, note_event_sender },

            ops_last_frame: [].into(),
        }
    }

    /// Sets the `SortCapture` for the player.
    pub fn set_capture(&mut self, capture: SortCapture) {
        self.is_playing = false;
        self.capture = Some(capture);
    }

    /// Removes the player's current `SortCapture`.
    pub fn clear_capture(&mut self) {
        self.is_playing = false;
        self.capture = None;
    }

    /// Whether the player currently has a capture loaded.
    pub const fn has_capture(&self) -> bool {
        self.capture.is_some()
    }

    /// The time it takes for the player to complete the array playback from
    /// start to finished.
    pub const fn playback_time(&self) -> f32 {
        self.playback_time
    }

    /// Sets the base playback speed of the player such that it will complete
    /// the array playback in `time_to_complete` seconds.
    pub fn set_playback_time(&mut self, time_to_complete: f32) {
        self.playback_time = time_to_complete;
    }

    /// Resets the playback time to the default value
    /// ([`Self::DEFAULT_PLAYBACK_TIME`]).
    pub fn reset_playback_time(&mut self) {
        self.playback_time = Self::DEFAULT_PLAYBACK_TIME;
    }

    /// The speed multiplier of the player.
    pub const fn speed(&self) -> f32 {
        self.speed_mult
    }

    /// Sets the playback speed. This acts as a multiplier to
    /// [`Self::playback_time`].
    pub fn set_speed(&mut self, speed: f32) {
        self.speed_mult = speed;
    }

    /// Resets the speed multiplier, honoring [`Self::playback_time`].
    pub fn reset_speed(&mut self) {
        self.speed_mult = 1.0;
    }

    /// Begins playback.
    pub fn play(&mut self) {
        self.is_playing = true;
    }

    /// Pauses playback at the current position.
    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    /// Pauses playback, and resets the playback position to the beginning.
    pub fn stop(&mut self) {
        self.is_playing = false;

        if let Some(cap) = self.capture.as_mut() {
            cap.reset_progress();
        }
    }

    /// Whether the player is at the end of the capture.
    pub fn at_end(&self) -> bool {
        self.capture.as_ref().map_or(false, |c| c.is_done())
    }

    /// Whether the player is playing.
    pub const fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn is_sorted(&self) -> bool {
        self.capture.as_ref().map_or(false, |c| c.is_sorted())
    }

    pub fn sort_data(&self) -> Option<SortData> {
        self.capture.as_ref().map(|c| c.data)
    }

    pub fn algorithm(&self) -> Option<SortingAlgorithm> {
        self.capture.as_ref().map(|c| c.algorithm())
    }

    /// Copies the internal array state to the provided array.
    ///
    /// # Panics
    ///
    /// Panics if `arr.len()` is not equal to the capture's array length.
    pub fn copy_arr_to(&mut self, arr: &mut [usize]) {
        if (self.capture.is_none()) {
            return;
        }

        let cap = self.capture.as_ref().unwrap();
        assert_eq!(cap.len(), arr.len(), "mismatched lengths");

        arr.copy_from_slice(cap.arr());
    }

    pub fn ops_last_frame(&self) -> Rc<[SortOperation]> {
        Rc::clone(&self.ops_last_frame)
    }

    pub fn post_audio(&mut self) {
        for &op in self.ops_last_frame.iter() {
            match op {
                SortOperation::Write { idx, value } => todo!(),
                SortOperation::Read { idx } => todo!(),
                SortOperation::Swap { a, b } => todo!(),
                SortOperation::Compare { a, b, res } => todo!(),
            }
        }
    }

    fn send_note_event(&self, op: SortOperation, timing: u32) {
        assert!(
            self.capture.is_some(),
            "attempted to post note event with no valid capture"
        );

        // This will not panic, as we know capture is Some
        let len_f = self.capture.as_ref().unwrap().len() as f32;
        let (mut freq, mut amp) = (0.5, 1.0);

        match op {
            SortOperation::Write { idx, .. } => {
                freq = idx as f32 / len_f * 0.5;
                amp = 0.6;
            }
            SortOperation::Read { idx } => {
                freq = idx as f32 / len_f;
                amp = 0.5;
            }
            SortOperation::Swap { a, b } => {
                freq = (a as f32 / len_f + b as f32 / len_f) * 0.25;
                amp = 0.7;
            }
            SortOperation::Compare { a, b, .. } => {
                freq = (a as f32 / len_f + b as f32 / len_f) * 0.5;
                amp = 0.4;
            }
        }

        _ = self.audio.note_event_sender.try_send(NoteEvent {
            freq: Self::map_freq(freq),
            amp,
            timing,
        });
    }

    fn map_freq(average_idx: f32) -> f32 {
        const MIN_NOTE: f32 = 48.0;
        const MAX_NOTE: f32 = 100.0;

        let x = average_idx.clamp(0.0, 1.0).powf(1.5);
        let note = (MAX_NOTE - MIN_NOTE).mul_add(x, MIN_NOTE).round();
        let quantized = Audio::quantize_to_scale(&MAJ_PENTATONIC, note, 63.0);

        Audio::note_to_freq(quantized)
    }

    fn buffer_sample_offset(&self) -> u32 {
        use std::sync::atomic::Ordering::Relaxed;

        let samples_exact = self
            .audio
            .callback_timer
            .load(Relaxed)
            .elapsed()
            .as_secs_f32()
            * SAMPLE_RATE as f32;

        samples_exact.round() as u32 % BUFFER_SIZE as u32
    }
}

impl Updatable for Player {
    fn update(&mut self, app: &App, update: UpdateData) {
        if !self.is_playing || self.capture.is_none() {
            return;
        }

        let cap = unsafe { self.capture.as_mut().unwrap_unchecked() };

        if cap.is_done() {
            println!("finished playing sort");
            self.ops_last_frame = [].into();
            self.is_playing = false;
            return;
        }

        let progress_per_second = self.playback_time.recip() * self.speed_mult;
        let progress_per_frame = progress_per_second * update.delta_time;

        let curr_progress = cap.playback_progress();

        self.ops_last_frame =
            cap.set_progress(curr_progress + progress_per_frame);
    }
}
