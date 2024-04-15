#![allow(unused)]

use std::sync::{Arc, Mutex};

pub use nannou::prelude::*;

pub type SortArray = Arc<Mutex<Vec<usize>>>;

mod model;
mod process;

use model::Model;
use process::*;

fn main() {
    nannou::app(Model::new).update(update).run();
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.update(app, &update);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.draw(&draw, &frame);

    draw.to_frame(app, &frame).unwrap();
}
