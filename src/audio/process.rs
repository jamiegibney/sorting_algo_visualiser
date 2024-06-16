use super::*;
use nannou_audio::Buffer;

const BUFFER_TIME: f32 = BUFFER_SIZE as f32 / SAMPLE_RATE as f32;

/// The audio processing callback.
pub fn process(audio: &mut Audio, buffer: &mut Buffer) {
    let buf_start = Instant::now();
    if !audio.running {
        buffer.fill(0.0);
        return;
    }

    if audio.process_voices(buffer) {
        process_effects(audio, buffer);
    }

    update_callback_timer(audio);

    let elapsed = buf_start.elapsed().as_secs_f32();
    audio.dsp_load.store(elapsed / BUFFER_TIME, Relaxed);
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
    Audio::process_buffer(buffer, |ch, smp| {
        *smp = audio.hp.tick(ch, *smp);
        // *smp = audio.lp.tick(ch, *smp);
        *smp = audio.compressor.tick(ch, *smp);

        // TODO: for safety...
        *smp = (*smp).clamp(-1.0, 1.0);
    });
}
