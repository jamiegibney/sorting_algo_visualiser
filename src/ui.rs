use super::algorithms::SortingAlgorithm;
use super::*;
use crate::prelude::*;
use nannou::text::*;

#[derive(Clone, Copy, Debug)]
pub struct UiData {
    pub algorithm: SortingAlgorithm,
    pub data: Option<SortData>,
    pub resolution: usize,
    pub player_time: f32,
    pub speed: f32,
    pub num_voices: u32,
    pub dsp_load: f32,
    pub sorted: bool,
    pub computing: bool,
    pub shuffling: bool,
}

#[derive(Debug)]
pub struct Ui {
    text: String,
}

impl Ui {
    pub const fn new() -> Self {
        Self { text: String::new() }
    }

    pub fn update_text(&mut self, ui_data: UiData) {
        let UiData {
            algorithm,
            data,
            resolution,
            player_time,
            speed,
            num_voices,
            dsp_load,
            sorted,
            computing,
            shuffling,
        } = ui_data;

        let info = if computing {
            String::from("Computing...")
        }
        else {
            data.map_or_else(|| String::from("No data — no algorithm has been captured"), |data| {
            let SortData { writes, reads, swaps, comparisons } = data;
            format!(
                "Writes: {writes}, reads: {reads}, swaps: {swaps}, comparisons: {comparisons}"
            )
        })
        };
        let algo = format!(
            "Algorithm: {}",
            if shuffling {
                String::from("Shuffling...")
            }
            else {
                algorithm.to_string()
            }
        );
        let res = format!("{resolution} segments");
        let sorted = format!("Sorted: {}", if sorted { "yes" } else { "no" });
        let speed = format!(
            "Speed: {speed:.2}x ({:.2}s playback time)",
            player_time * speed.recip()
        );
        let voices =
            format!("Active voices: {num_voices}/{}", super::audio::NUM_VOICES);
        let dsp = format!("DSP load: {:.1}%", dsp_load * 100.0);

        self.text = format!(
            "{algo}\n{res}\n{speed}\n{info}\n{sorted}\n{voices}\n{dsp}"
        );
    }

    pub fn draw(&self, draw: &Draw) {
        draw.text(&self.text)
            .layout(&default_layout())
            .xy(vec2(-135.0, -320.0))
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
