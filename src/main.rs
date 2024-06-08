#![allow(unused, clippy::wildcard_imports)]

use std::sync::{Arc, Mutex};

use nannou::prelude::*;
use nannou_audio;

// pub type SortBuffer = Arc<Mutex<Vec<usize>>>;

mod algorithms;
mod audio;
mod color_wheel;
mod message;
mod model;
mod prelude;
mod process;
mod sorting;
mod ui;

use audio::*;
use color_wheel::*;
use message::NoteEvent;
use model::Model;
use prelude::*;
use process::*;
use ui::{Ui, UiData};

fn generate_envelope_data() {
    let sr = 48000.0;
    let attack_len = 0.02;
    let release_len = 0.03;

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

    std::fs::write("src/audio/envelope_data", unsafe {
        std::slice::from_raw_parts(
            start.as_ptr().cast::<u8>(),
            start.len() * std::mem::size_of::<f32>(),
        )
    });
}

fn main() {
    // generate_envelope_data();
    nannou::app(Model::new).update(update).run();
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
