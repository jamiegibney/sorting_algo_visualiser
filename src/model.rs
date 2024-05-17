#![allow(clippy::suboptimal_flops)]

use super::*;
use crate::algorithms::SortingAlgorithm;
use nannou::prelude::*;
use std::f32::consts::{FRAC_PI_2, TAU};

pub struct Model {
    window_id: WindowId,
    process: Process,
    color_wheel: ColorWheel,
    sort_arr: SortArray,
    target_arr: Vec<usize>,
    previous_algo: Option<SortingAlgorithm>,

    resolution: usize,

    num_iters: usize,
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

        let display = ColorWheel::new();

        Self {
            window_id,
            process: Process::new(display.sort_arr_ref())
                .with_algorithm(SortingAlgorithm::Selection),
            previous_algo: None,

            sort_arr: display.sort_arr_ref(),
            target_arr: (0..DEFAULT_RESOLUTION).collect(),

            color_wheel: display,
            resolution: DEFAULT_RESOLUTION,

            num_iters: 0,
        }
    }

    pub fn set_resolution(&mut self, new_resolution: usize) {
        self.target_arr = (0..new_resolution).collect();
        self.color_wheel.resize(new_resolution);
        self.resolution = new_resolution;

        println!("Set resolution to {new_resolution} slices");
    }

    pub fn double_resolution(&mut self) {
        self.set_resolution((self.resolution * 2).min(MAX_RESOLUTION));
    }

    pub fn halve_resolution(&mut self) {
        self.set_resolution((self.resolution / 2).max(3));
    }

    /// Updates the app state, i.e. the internal sorting process and then the color wheel.
    pub fn update(&mut self) {
        let sorted = self.process.update();
        self.num_iters += self.process.iters_last_update();

        if sorted {
            if !matches!(
                self.process.current_algorithm,
                SortingAlgorithm::Scramble
            ) {
                let not = if self.is_sorted() { "" } else { "NOT " };
                println!(
                    "Done in ~{} iterations. The array is {not}correctly sorted.",
                    self.num_iters
                );
            }

            self.num_iters = 0;

            if let Some(prev) = self.previous_algo.take() {
                self.process.set_algorithm(prev);
            }
        }

        self.color_wheel.update();
    }

    /// Draws the color wheel to the provided `Draw` instance.
    pub fn draw(&self, draw: &Draw) {
        self.color_wheel.draw(draw);
    }

    /// Forces the color wheel to be sorted via `std::sort_unstable`.
    pub fn force_sort(&mut self) {
        self.color_wheel.sort();
        self.color_wheel.update();
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

    pub fn start_sort(&mut self) {
        self.process.run();
    }

    pub fn scramble(&mut self) {
        if self.process.is_running() {
            return;
        }

        self.previous_algo = Some(self.process.current_algorithm);
        self.process.set_algorithm(SortingAlgorithm::Scramble);
        self.process.run();
    }

    pub fn toggle(&mut self) {
        self.process.toggle();
    }

    pub fn current_algorithm(&self) -> String {
        self.process.current_algorithm.to_string()
    }
}

/// The callback for key-down presses.
pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            println!("Sorting with {}...", model.current_algorithm());
            model.start_sort();
        }
        Key::R => {
            println!("Randomising...");
            model.scramble();
        }
        Key::F => {
            println!("Forcing-sorting the wheel...");
            model.force_sort();
        }
        Key::T => {
            model.toggle();
        }
        Key::V => {
            let s = if model.is_sorted() { "" } else { "NOT " };
            println!("The array is {s}correctly sorted.");
        }
        Key::Plus | Key::Equals => model.double_resolution(),
        Key::Underline | Key::Minus => model.halve_resolution(),
        _ => {}
    }
}
