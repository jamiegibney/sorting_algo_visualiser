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
    player: Player,

    target_arr: Vec<usize>,

    audio_stream: Stream<Audio>,
    resolution: usize,

    audio_voice_counter: Arc<AtomicU32>,
    sorted: bool,

    update_data: UpdateData,
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
            sort_arr: SortArray::new(DEFAULT_RESOLUTION),
            player: Player::new(
                DEFAULT_RESOLUTION, note_tx, audio_callback_timer,
            ),

            target_arr: (0..DEFAULT_RESOLUTION).collect(),
            resolution: DEFAULT_RESOLUTION,

            sorted: true,
            audio_voice_counter,

            update_data: UpdateData {
                last_frame: Instant::now(),
                delta_time: 0.0,
            },

            audio_stream: audio_model.into_stream(),
        }
    }

    pub fn set_resolution(&mut self, new_resolution: usize) {
        self.target_arr = (0..new_resolution).collect();
        self.sort_arr.resize(new_resolution);
        self.color_wheel.resize(new_resolution);
        self.resolution = new_resolution;
        self.player.clear_capture();

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
        self.process.current_algorithm.cycle_next();
    }

    pub fn previous_algorithm(&mut self) {
        self.process.current_algorithm.cycle_prev();
    }

    // *** *** *** //

    /// Updates the app state.
    pub fn update(&mut self, app: &App) {
        self.update_data.delta_time =
            self.update_data.last_frame.elapsed().as_secs_f32();

        self.player.update(app, self.update_data);
        self.player.post_audio();

        self.color_wheel
            .set_overlay_ops(self.player.ops_last_frame());
        self.color_wheel.update(app, self.update_data);
        self.player.copy_arr_to(self.color_wheel.arr_mut());

        self.ui.update_text(UiData {
            algorithm: self.process.current_algorithm,
            data: self.player.sort_data(),
            resolution: self.resolution,
            speed: self.player.speed(),
            num_voices: self.audio_voice_counter.load(Relaxed),
            sorted: self.player.is_sorted(),
        });

        self.update_data.last_frame = Instant::now();
    }

    /// Draws the app visuals to the provided `Draw` instance.
    pub fn draw(&self, draw: &Draw) {
        self.color_wheel.draw(draw, self.update_data);
        self.ui.draw(draw);
    }

    // *** *** *** //

    /// Forces the color wheel to be sorted via `std::sort_unstable`.
    pub fn force_sort(&mut self) {
        self.player.clear_capture();
        self.sort_arr
            .prepare_for_sort(self.process.current_algorithm);
        self.sort_arr.force_sort();
        self.player.set_capture(self.sort_arr.create_capture());
    }

    /// Returns `true` if the sorting array is correctly sorted.
    pub fn is_sorted(&self) -> bool {
        self.player.is_sorted()
    }

    pub fn compute(&mut self) {
        // prepare the array
        self.sort_arr
            .prepare_for_sort(self.process.current_algorithm);

        // perform the sort
        self.process.sort(&mut self.sort_arr);

        // dump the captured data to the player
        self.player.set_capture(self.sort_arr.create_capture());

        self.player.play();
    }

    /// Starts a shuffle.
    pub fn shuffle(&mut self) {
        let algo = self.process.current_algorithm;

        self.process.set_algorithm(SortingAlgorithm::Shuffle);
        self.compute();
        self.process.set_algorithm(algo);
    }

    pub fn increase_speed(&mut self) {
        let speed = self.player.speed();
        self.player.set_speed((speed + 0.02).min(5.0));
    }

    pub fn decrease_speed(&mut self) {
        let speed = self.player.speed();
        self.player.set_speed((speed - 0.02).max(-5.0));
    }

    pub fn play(&mut self) {
        if self.player.at_end() {
            self.player.stop();
        }

        self.player.play();
    }

    pub fn pause(&mut self) {
        self.player.pause();
    }

    pub fn stop(&mut self) {
        self.player.stop();
    }

    pub const fn is_playing(&self) -> bool {
        self.player.is_playing()
    }

    pub fn current_algorithm(&self) -> String {
        self.process.current_algorithm.to_string()
    }

    pub fn stop_audio(&self) {
        _ = self.audio_stream.send(audio::Audio::stop);
    }

    pub fn resume_audio(&self) {
        _ = self.audio_stream.send(audio::Audio::start);
    }
}

/// The callback for key-down presses.
pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        // "play/pause"
        Key::Space => {
            if model.is_playing() {
                model.pause();
            }
            else {
                model.play();
            }
        }
        // "stop"
        Key::Back | Key::Delete => model.stop(),
        // "recompute"
        Key::R => model.compute(),
        // "shuffle"
        Key::S => model.shuffle(),
        Key::Return => {
            if app.keys.mods.shift() {
                model.previous_algorithm();
            }
            else {
                model.next_algorithm();
            }
        }
        // increase res
        Key::Plus | Key::Equals => model.increase_resolution(),
        // decrease res
        Key::Underline | Key::Minus => model.decrease_resolution(),
        // increase speed
        Key::Period => model.increase_speed(),
        // decrease speed
        Key::Comma => model.decrease_speed(),
        // "force-sort"
        Key::F => {
            println!("Forcing-sorting the wheel...");
            model.force_sort();
        }
        // "verify"
        Key::V => {
            let s = if model.is_sorted() { "" } else { "NOT " };
            println!("The array is {s}correctly sorted.");
        }
        _ => {}
    }
}
