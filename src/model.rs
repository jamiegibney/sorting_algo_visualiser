#![allow(clippy::suboptimal_flops)]

use super::*;
use nannou::prelude::*;
use std::f32::consts::{FRAC_PI_2, TAU};

pub struct Model {
    window_id: WindowId,
    process: Process,
    display: Display,
    sort_arr: SortArray,
    target_arr: Vec<usize>,
}

impl Model {
    /// Creates a new app model.
    pub fn new(app: &App) -> Self {
        let window_id = app
            .new_window()
            .view(super::view)
            .key_pressed(key_pressed)
            .size(800, 800)
            .build()
            .expect("failed to initialize main window");

        let display = Display::new();

        Self {
            window_id,
            process: Process::new(display.sort_arr_ref()),
            sort_arr: display.sort_arr_ref(),
            target_arr: (0..NUM_SLICES).collect(),
            display,
        }
    }

    /// Updates the app state, i.e. the internal sorting process and
    /// then the display.
    pub fn update(&mut self) {
        self.process.update();
        self.display.update();
    }

    /// Draws the color wheel to the provided `Draw` instance.
    pub fn draw(&self, draw: &Draw) {
        self.display.draw(draw);
    }

    /// Scrambles the color wheel.
    pub fn scramble(&mut self) {
        self.display.scramble_sort_arr();
    }

    /// Forces the color wheel to be sorted via `std::sort_unstable`.
    pub fn force_sort(&mut self) {
        self.display.sort();
    }

    /// Returns `true` if the sorting array is correctly sorted.
    pub fn is_sorted(&self) -> bool {
        if let Ok(guard) = self.sort_arr.lock() {
            if guard.as_slice() == self.target_arr {
                return true;
            }
        }

        false
    }
}

/// The callback for key-down presses.
pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.scramble(),
        Key::F => model.force_sort(),
        Key::V => {
            let s = if model.is_sorted() { "" } else { "NOT " };
            println!("The array is {s}correctly sorted.");
        }
        _ => {}
    }
}
