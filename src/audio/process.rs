use super::*;
use nannou_audio::Buffer;

const BUFFER_TIME: f32 = BUFFER_SIZE as f32 / SAMPLE_RATE as f32;

/// The audio processing callback.
pub fn process(audio: &mut Audio, buffer: &mut Buffer) {
    if !audio.running {
        audio.dsp_load.store(0.0, Relaxed);
        return;
    }

    let buf_start = Instant::now();

    audio.process(buffer);

    update_callback_timer(audio);

    let elapsed = buf_start.elapsed().as_secs_f32();
    audio.dsp_load.store(elapsed / BUFFER_TIME, Relaxed);
}

fn update_callback_timer(audio: &Audio) {
    let timer = audio.callback_timer();

    if timer.load(Relaxed).elapsed().as_secs_f32() >= 0.0001 {
        timer.store(InstantTime(Instant::now()), Relaxed);
    }
}
