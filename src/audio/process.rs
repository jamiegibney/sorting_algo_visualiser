use super::*;
use nannou_audio::Buffer;

/// The maximum block size for audio processing.
pub const MAX_BLOCK_SIZE: usize = 64;

/// The audio processing callback.
pub fn process(audio: &mut Audio, buffer: &mut Buffer) {
    let buffer_len = buffer.len_frames();

    let mut next_event = audio.note_receiver().try_recv().ok();

    let mut block_start = 0;
    let mut block_end = MAX_BLOCK_SIZE.min(buffer_len);

    // handle polyphonic voices
    while block_start < buffer_len {
        'events: loop {
            match next_event {
                // if we've snapped the block to an event
                Some(event)
                    if (event.sample_offset() as usize) <= block_start =>
                {
                    audio.voice_handler.new_voice(event.freq(), event.amp());

                    next_event = audio.note_receiver().try_recv().ok();
                }
                // if an event is within this block, snap to the event
                Some(event) if (event.sample_offset() as usize) < block_end => {
                    block_end = event.sample_offset() as usize;
                    break 'events;
                }
                // if no new events are available
                _ => break 'events,
            }
        }

        // TODO(jamiegibney): master gain control?
        let mut gain = [0.03; MAX_BLOCK_SIZE];

        // process voices and clean any which are finished
        audio
            .voice_handler
            .process_block(buffer, block_start, block_end, gain);
        audio.voice_handler.free_finished_voices();

        block_start = block_end;
        block_end = (block_end + MAX_BLOCK_SIZE).min(buffer_len);
    }

    process_effects(audio, buffer);
    update_callback_timer(audio);

    audio.update_voice_counter();
}

fn update_callback_timer(audio: &Audio) {
    use std::sync::atomic::Ordering::Relaxed;

    let timer = audio.callback_timer();

    if timer.load(Relaxed).elapsed().as_secs_f32() >= 0.0001 {
        timer.store(InstantTime(Instant::now()), Relaxed);
    }
}

/// Processes any audio effects.
fn process_effects(audio: &mut Audio, buffer: &mut Buffer) {
    // TODO(jamiegibney): add audio effects.
}
