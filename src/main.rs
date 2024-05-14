#![allow(unused)]

use std::sync::{Arc, Mutex};

pub use nannou::prelude::*;

pub type SortArray = Arc<Mutex<Vec<usize>>>;

mod model;
mod display;
mod process;

use model::Model;
use process::*;
use display::*;

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
