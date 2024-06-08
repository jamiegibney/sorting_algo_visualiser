use crate::prelude::*;

#[derive(Debug)]
struct AudioState {
    callback_timer: Arc<Atomic<InstantTime>>,
    note_event_sender: Sender<NoteEvent>,
}

#[derive(Debug)]
pub struct Player {
    capture: Option<SortCapture>,

    audio: AudioState,
}

impl Player {
    pub fn new(
        len: usize,
        note_event_sender: Sender<NoteEvent>,
        callback_timer: Arc<Atomic<InstantTime>>,
    ) -> Self {
        Self {
            audio: AudioState { callback_timer, note_event_sender },
            capture: None,
        }
    }

    pub fn set_capture(&mut self, capture: SortCapture) {
        self.capture = Some(capture);
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
            SortOperation::Noop => {}
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
