#![allow(clippy::wildcard_imports, clippy::needless_range_loop)]
#![feature(portable_simd)]

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use nannou::prelude::*;

mod algorithms;
mod audio;
mod color_wheel;
mod message;
mod model;
mod prelude;
mod process;
mod sorting;
mod thread_pool;
mod ui;

use audio::*;
use color_wheel::*;
use message::NoteEvent;
use model::Model;
use prelude::*;
use process::*;
use ui::{Ui, UiData};

const ENVELOPE_DATA_PATH: &str = "src/audio/envelope_data";

// TODO: move this to the audio module
fn generate_envelope_data() {
    let sr = 48000.0;
    let attack_len = 0.01;
    let release_len = 0.035;

    let attack = (attack_len * sr).round() as usize;
    let release = (release_len * sr).round() as usize;

    let mut start = vec![0.0; attack];
    let mut end = vec![0.0; release];

    for i in 0..attack {
        let x = i as f32 / attack as f32;
        start[i] = x.clamp(0.0, 1.0);
    }
    for i in 0..release {
        let x = (release - i) as f32 / release as f32;
        end[i] = (x.powf(1.5)).clamp(0.0, 1.0);
    }

    start.append(&mut end);

    std::fs::write(ENVELOPE_DATA_PATH, unsafe {
        std::slice::from_raw_parts(
            start.as_ptr().cast::<u8>(),
            start.len() * std::mem::size_of::<f32>(),
        )
    }).expect("failed to write envelope data");
}

#[derive(Clone, Copy, Debug)]
pub struct UpdateData {
    pub last_frame: Instant,
    pub delta_time: f32,
}

pub trait Updatable {
    fn update(&mut self, app: &App, update: UpdateData);
}

pub trait Drawable: Updatable {
    fn draw(&self, draw: &Draw, update: UpdateData);
}

fn update(app: &App, model: &mut Model, _: Update) {
    model.update(app);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    generate_envelope_data();
    nannou::app(Model::new).update(update).run();
}
