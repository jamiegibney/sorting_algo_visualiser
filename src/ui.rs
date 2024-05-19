use super::algorithms::SortingAlgorithm;
use super::sorting_array::SortResults;
use super::*;
use nannou::text::*;

#[derive(Debug)]
pub struct Ui {
    text: String,
}

impl Ui {
    pub const fn new() -> Self {
        Self { text: String::new() }
    }

    pub fn update(
        &mut self,
        algorithm: SortingAlgorithm,
        results: &SortResults,
        resolution: usize,
        speed: f32,
        num_voices: u32,
        sorted: bool,
    ) {
        let SortResults { writes, reads, swaps, comparisons } = results;
        let algo = format!("Algorithm: {algorithm}",);
        let res = format!("{resolution} segments");
        let sorted = format!("Sorted: {}", if sorted { "yes" } else { "no" });
        let info = format!("Writes: {writes}, reads: {reads}, swaps: {swaps}, comparisons: {comparisons}");
        let speed = format!(
            "Speed: {speed:.2}x ({} iterations per second)",
            (algorithm.steps() as f32 * speed) as u32
        );
        let voices = format!(
            "Active audio voices: {num_voices}/{}",
            super::audio::NUM_VOICES
        );
        self.text =
            format!("{algo}\n{res}\n{info}\n{speed}\n{sorted}\n{voices}");
    }

    pub fn draw(&self, draw: &Draw) {
        draw.text(&self.text)
            .layout(&default_layout())
            .xy(vec2(-135.0, -330.0))
            .wh(vec2(500.0, 300.0))
            .color(WHITE);
    }
}

fn default_layout() -> Layout {
    Layout {
        justify: Justify::Left,
        font_size: 16,
        line_spacing: 3.0,
        ..Default::default()
    }
}
