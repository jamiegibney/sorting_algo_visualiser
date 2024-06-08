#![allow(clippy::suboptimal_flops)]

use super::*;
use crate::prelude::*;
use nannou::prelude::*;
use nannou_audio::Stream;

pub struct Model {
    window_id: WindowId,

    process: Process,
    color_wheel: ColorWheel,
    ui: Ui,
    sort_arr: SortArray,

    target_arr: Vec<usize>,
    previous_algo: Option<SortingAlgorithm>,

    audio_stream: Stream<Audio>,
    resolution: usize,

    speed_scale: f32,

    results: SortResults,
    audio_voice_counter: Arc<AtomicU32>,
    sorted: bool,

    num_iters: usize,
}

impl Model {
    /// Creates a new app model.
    pub fn new(app: &App) -> Self {
        let window_id = app
            .new_window()
            .view(super::view)
            .title("Sorting Algorithms")
            .key_pressed(key_pressed)
            .size(800, 800)
            .build()
            .expect("failed to initialize main window");

        let color_wheel = ColorWheel::new();
        let (note_tx, note_rx) = bounded(24);

        let audio_voice_counter = Arc::new(AtomicU32::new(0));

        let audio_model = Audio::new(note_rx, Arc::clone(&audio_voice_counter));
        let audio_callback_timer = Arc::clone(audio_model.callback_timer());

        Self {
            window_id,

            process: Process::new(),
            color_wheel,
            ui: Ui::new(),
            sort_arr: SortArray::new(
                DEFAULT_RESOLUTION, note_tx, audio_callback_timer,
            ),

            previous_algo: None,

            target_arr: (0..DEFAULT_RESOLUTION).collect(),
            resolution: DEFAULT_RESOLUTION,

            speed_scale: 1.0,

            results: SortResults::default(),
            sorted: true,
            audio_voice_counter,

            audio_stream: audio_model.into_stream(),
            num_iters: 0,
        }
    }

    pub fn set_speed_scale(&mut self, factor: f32) {
        const MAX_SPEED_SCALE: f32 = 1000.0;
        const MIN_SPEED_SCALE: f32 = 0.01;
        self.speed_scale = factor.clamp(MIN_SPEED_SCALE, MAX_SPEED_SCALE);
    }

    // Increases the sorting speed by 20 %.
    pub fn increase_speed(&mut self) {
        self.set_speed_scale(self.speed_scale * 1.2);
    }

    // Decreases the sorting speed by 20 %.
    pub fn decrease_speed(&mut self) {
        self.set_speed_scale(self.speed_scale * 0.8);
    }

    pub fn set_resolution(&mut self, new_resolution: usize) {
        self.process.stop();

        self.target_arr = (0..new_resolution).collect();
        self.sort_arr.resize(new_resolution);
        self.color_wheel.resize(new_resolution);
        self.resolution = new_resolution;

        self.sorted = true;
    }

    pub fn increase_resolution(&mut self) {
        self.set_resolution((self.resolution * 8 / 6).min(MAX_RESOLUTION));
    }

    pub fn decrease_resolution(&mut self) {
        self.set_resolution((self.resolution * 6 / 8).max(3));
    }

    pub fn double_resolution(&mut self) {
        self.set_resolution((self.resolution * 2).min(MAX_RESOLUTION));
    }

    pub fn halve_resolution(&mut self) {
        self.set_resolution((self.resolution / 2).max(3));
    }

    pub fn next_algorithm(&mut self) {
        if self.is_running() {
            return;
        }

        self.process.current_algorithm.cycle_next();
        self.sort_arr
            .set_current_algorithm(self.process.current_algorithm);
    }

    pub fn previous_algorithm(&mut self) {
        if self.is_running() {
            return;
        }

        self.process.current_algorithm.cycle_prev();
        self.sort_arr
            .set_current_algorithm(self.process.current_algorithm);
    }

    /// Updates the app state.
    pub fn update(&mut self) {
        if self.process.update(&mut self.sort_arr, self.speed_scale) {
            if !matches!(
                self.process.current_algorithm,
                SortingAlgorithm::Shuffle
            ) {
                self.results.add_from(&self.sort_arr.sort_results());
                self.sorted = self.is_sorted();
            }

            if let Some(prev) = self.previous_algo.take() {
                self.process.set_algorithm(prev);
            }
        }

        self.results.add_from(&self.sort_arr.sort_results());

        self.color_wheel.update(self.sort_arr.as_slice());
        self.color_wheel
            .overlay_from(self.sort_arr.take_op_buffer());

        self.ui.update_text(UiData {
            algorithm: self.process.current_algorithm,
            results: self.results,
            resolution: self.resolution,
            speed: self.speed_scale,
            num_voices: self
                .audio_voice_counter
                .load(atomic::Ordering::Relaxed),
            sorted: self.sorted,
        });
    }

    /// Draws the app visuals to the provided `Draw` instance.
    pub fn draw(&self, draw: &Draw) {
        self.color_wheel.draw(draw);
        self.ui.draw(draw);
    }

    /// Forces the color wheel to be sorted via `std::sort_unstable`.
    pub fn force_sort(&mut self) {
        self.process.stop();
        self.sort_arr.force_sort();
        self.sorted = true;
    }

    /// Returns `true` if the sorting array is correctly sorted.
    pub fn is_sorted(&self) -> bool {
        self.target_arr.as_slice() == self.sort_arr.as_slice()
    }

    /// Starts a sort.
    pub fn start_sort(&mut self) {
        self.results.reset();
        self.process.run();
        self.sorted = false;
    }

    /// Whether a sort/shuffle is currently in progress.
    pub const fn is_running(&self) -> bool {
        self.process.is_running()
    }

    /// Starts a shuffle.
    pub fn shuffle(&mut self) {
        self.previous_algo = Some(self.process.current_algorithm);
        self.process.set_algorithm(SortingAlgorithm::Shuffle);
        self.process.run();

        self.sorted = false;
        self.results.reset();
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
            if !model.is_running() {
                model.start_sort();
            }
        }
        Key::R => {
            if !model.is_running() {
                model.shuffle();
            }
        }
        Key::T => model.toggle(),
        Key::Return => {
            if app.keys.mods.shift() {
                model.previous_algorithm();
            }
            else {
                model.next_algorithm();
            }
        }
        Key::Plus | Key::Equals => model.increase_resolution(),
        Key::Underline | Key::Minus => model.decrease_resolution(),
        Key::Period => model.increase_speed(),
        Key::Comma => model.decrease_speed(),
        Key::F => {
            println!("Forcing-sorting the wheel...");
            model.force_sort();
        }
        Key::V => {
            let s = if model.is_sorted() { "" } else { "NOT " };
            println!("The array is {s}correctly sorted.");
        }
        _ => {}
    }
}
