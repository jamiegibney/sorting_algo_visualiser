use crate::prelude::*;
use crate::thread_pool::ThreadPool;
use std::{os::unix::fs::DirBuilderExt, rc::Rc, thread, time::Duration};

const MAX_AUDIO_NOTES_PER_SECOND: usize = 5000;

#[derive(Debug)]
struct AudioState {
    callback_timer: Arc<Atomic<InstantTime>>,
    note_event_sender: Arc<Sender<NoteEvent>>,
}

#[derive(Debug)]
pub struct Player {
    capture: Option<SortCapture>,

    playback_time: f32,
    speed_mult: f32,

    is_playing: bool,

    audio: AudioState,

    ops_last_frame: Arc<[SortOperation]>,

    audio_msg_thread: ThreadPool,
}

impl Player {
    pub const DEFAULT_PLAYBACK_TIME: f32 = 8.0;

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

            audio: AudioState {
                callback_timer,
                note_event_sender: Arc::new(note_event_sender),
            },

            ops_last_frame: [].into(),

            audio_msg_thread: ThreadPool::build(1)
                .expect("failed to allocate audio msg thread"),
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

    pub fn ops_last_frame(&self) -> Arc<[SortOperation]> {
        Arc::clone(&self.ops_last_frame)
    }

    fn send_note_events(&self, delta_time: f32) {
        let audio_ops_this_frame =
            (MAX_AUDIO_NOTES_PER_SECOND as f32 * delta_time) as usize;
        let time_between = delta_time / (audio_ops_this_frame as f32) * 0.8;

        // This will not panic, as we know capture is Some
        let len_f = self.capture.as_ref().unwrap().len() as f32;
        let ops_last_frame = Arc::clone(&self.ops_last_frame);
        let event_sender = Arc::clone(&self.audio.note_event_sender);
        let callback_timer = Arc::clone(&self.audio.callback_timer);

        self.audio_msg_thread.execute(move || {
            let map = |x: f32| (x * 2.0 - 1.0).clamp(-1.0, 1.0) * 0.5;

            for &op in ops_last_frame.iter().take(audio_ops_this_frame) {
                let (mut freq, mut amp, mut pan) = (0.5, 1.0, 0.0);
                let mut osc = OscillatorType::default();

                match op {
                    SortOperation::Write { idx, .. } => {
                        let i = idx as f32 / len_f;
                        freq = i * 0.5;
                        amp = 0.6;
                        pan = i;
                    }
                    SortOperation::Read { idx } => {
                        let i = idx as f32 / len_f;
                        freq = idx as f32 / len_f;
                        amp = 0.5;
                        pan = i;
                        osc = OscillatorType::Tri;
                    }
                    SortOperation::Swap { a, b } => {
                        let a_f = a as f32 / len_f;
                        let b_f = b as f32 / len_f;
                        freq = (a_f + b_f) * 0.25;
                        amp = 0.7;
                        pan = (a_f + b_f) * 0.5;
                    }
                    SortOperation::Compare { a, b, .. } => {
                        let a_f = a as f32 / len_f;
                        let b_f = b as f32 / len_f;
                        freq = (a_f + b_f) * 0.5;
                        amp = 0.4;
                        pan = freq;
                        osc = OscillatorType::Tri;
                    }
                }

                thread::sleep(Duration::from_secs_f32(time_between));

                let samples_exact =
                    callback_timer.load(Relaxed).elapsed().as_secs_f32()
                        * SAMPLE_RATE as f32;

                _ = event_sender.try_send(NoteEvent {
                    osc,
                    freq: Self::map_freq(freq),
                    amp,
                    timing: samples_exact.round() as u32 % BUFFER_SIZE as u32,
                    pan: map(pan + random_range(-0.5, 0.5)),
                });
            }
        });
    }

    fn map_freq(freq: f32) -> f32 {
        const MIN_NOTE: f32 = 36.0;
        const MAX_NOTE: f32 = 104.0;

        // let x = freq.clamp(0.0, 1.0).powf(1.1);
        // let x = 1.0 - (1.0 - freq.clamp(0.0, 1.0)).powf(1.2);
        let n = 5.0;
        let x = ((n - 1.0) * freq.clamp(0.0, 1.0) + 1.0).log(n);
        let note = (MAX_NOTE - MIN_NOTE).mul_add(x, MIN_NOTE);
        // let quantized = Audio::quantize_to_scale(&MINOR_SCALE, note, 59.0);

        Audio::note_to_freq(note)
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
            // println!("Sorting done");
            self.ops_last_frame = [].into();
            self.is_playing = false;
            return;
        }

        let progress_per_second =
            if matches!(cap.algorithm(), SortingAlgorithm::Shuffle) {
                0.5
            }
            else {
                self.playback_time.recip() * self.speed_mult
            };
        let progress_per_frame = progress_per_second * update.delta_time;

        let curr_progress = cap.playback_progress();

        self.ops_last_frame =
            cap.set_progress(curr_progress + progress_per_frame);

        self.send_note_events(update.delta_time);
    }
}
