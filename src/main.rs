#![allow(unused, clippy::wildcard_imports)]

use std::sync::{Arc, Mutex};

pub use nannou::prelude::*;

pub type SortArray = Arc<Mutex<Vec<usize>>>;

mod model;
mod color_wheel;
mod process;
mod algorithms;

use model::Model;
use process::*;
use color_wheel::*;

fn main() {
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
