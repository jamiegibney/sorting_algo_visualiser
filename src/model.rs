#![allow(clippy::suboptimal_flops)]

use super::*;
use crate::{prelude::*, thread_pool::ThreadPool};
use nannou_audio::Stream;

pub struct Model {
    _window_id: WindowId,

    current_algorithm: Arc<Atomic<SortingAlgorithm>>,
    previous_algorithm: Arc<Mutex<Option<SortingAlgorithm>>>,

    process: Arc<Mutex<Process>>,
    color_wheel: ColorWheel,
    ui: Ui,
    sort_arr: Arc<Mutex<SortArray>>,
    player: Arc<Mutex<Player>>,

    target_arr: Vec<usize>,

    thread_pool: ThreadPool,

    audio_stream: Stream<Audio>,
    audio_voice_counter: Arc<AtomicU32>,
    dsp_load: Arc<Atomic<f32>>,
    audio_playing: bool,

    sorted: bool,
    resolution: usize,

    is_shuffling: bool,
    computing: Arc<AtomicBool>,
    auto_play_ch: (Arc<Sender<()>>, Receiver<()>),

    sort_after_shuffle: bool,

    update_data: UpdateData,
}

impl Model {
    /// Creates a new app model.
    pub fn new(app: &App) -> Self {
        let _window_id = app
            .new_window()
            .view(super::view)
            .title("Sorting Algorithms")
            .key_pressed(key_pressed)
            .size(800, 800)
            .resizable(false)
            .build()
            .expect("failed to initialize main window");

        let color_wheel = ColorWheel::new();
        let (note_tx, note_rx) =
            bounded(if cfg!(debug_assertions) { 96 } else { 512 });

        let audio_voice_counter = Arc::new(AtomicU32::new(0));

        let audio_model = Audio::new(note_rx, Arc::clone(&audio_voice_counter));
        let audio_callback_timer = Arc::clone(audio_model.callback_timer());
        let dsp_load = Arc::clone(audio_model.dsp_load());

        let (ap_tx, ap_rx) = bounded(0);

        let algo = Arc::new(Atomic::new(SortingAlgorithm::default()));

        Self {
            _window_id,

            process: Arc::new(Mutex::new(Process::new(Arc::clone(&algo)))),
            current_algorithm: algo,
            previous_algorithm: Arc::new(Mutex::new(None)),

            color_wheel,
            ui: Ui::new(),
            sort_arr: Arc::new(Mutex::new(SortArray::new(DEFAULT_RESOLUTION))),
            player: Arc::new(Mutex::new(Player::new(
                note_tx, audio_callback_timer,
            ))),

            target_arr: (0..DEFAULT_RESOLUTION).collect(),
            resolution: DEFAULT_RESOLUTION,

            thread_pool: ThreadPool::build(1, None, Some(&["sorting"]))
                .expect("failed to allocate sorting thread"),

            sorted: true,

            computing: Arc::new(AtomicBool::new(false)),
            auto_play_ch: (Arc::new(ap_tx), ap_rx),

            sort_after_shuffle: false,
            is_shuffling: false,

            update_data: UpdateData {
                last_frame: Instant::now(),
                delta_time: 0.0,
            },

            audio_stream: audio_model.into_stream(),
            audio_voice_counter,
            dsp_load,
            audio_playing: true,
        }
    }

    pub fn set_resolution(&mut self, new_resolution: usize) {
        // println!("setting resolution to {new_resolution}");

        let mut player = self.player.lock();
        player.clear_capture();
        player.clear_ops();
        drop(player);

        self.target_arr = (0..new_resolution).collect();
        self.sort_arr.lock().resize(new_resolution);
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

    pub fn next_algorithm(&self) {
        let mut curr = self.current_algorithm.load(Relaxed);
        curr.cycle_next();
        self.current_algorithm.store(curr, Relaxed);
    }

    pub fn previous_algorithm(&self) {
        let mut curr = self.current_algorithm.load(Relaxed);
        curr.cycle_prev();
        self.current_algorithm.store(curr, Relaxed);
    }

    // *** *** *** //

    /// Updates the app state.
    pub fn update(&mut self, app: &App) {
        self.update_data.delta_time =
            self.update_data.last_frame.elapsed().as_secs_f32();

        let mut player = self.player.lock();
        let computing = self.computing.load(Relaxed);

        if self.auto_play_ch.1.try_recv().is_ok() {
            player.play();
        }

        if !computing && !player.is_playing() {
            if self.sort_after_shuffle {
                self.compute();
                self.sort_after_shuffle = false;
            }

            if self.is_shuffling {
                self.is_shuffling = false;
            }
        }

        player.update(app, self.update_data);

        self.color_wheel.set_overlay_ops(player.ops_last_frame());
        self.color_wheel.update(app, self.update_data);
        player.copy_arr_to(self.color_wheel.arr_mut());

        self.ui.update_text(UiData {
            algorithm: self.current_algorithm.load(Relaxed),
            data: player.sort_data(),
            resolution: self.resolution,
            player_time: player.playback_time(),
            speed: player.speed(),
            num_voices: self.audio_voice_counter.load(Relaxed),
            dsp_load: self.dsp_load.load(Relaxed),
            sorted: player.is_sorted(),
            computing,
            shuffling: self.is_shuffling,
        });

        drop(player);

        self.update_data.last_frame = Instant::now();
    }

    /// Draws the app visuals to the provided `Draw` instance.
    pub fn draw(&self, draw: &Draw) {
        self.color_wheel.draw(draw, self.update_data);
        self.ui.draw(draw);
    }

    // *** *** *** //

    /// Forces the color wheel to be sorted via `std::sort_unstable`.
    pub fn force_sort(&self) {
        let mut player = self.player.lock();
        let mut sort_arr = self.sort_arr.lock();

        player.clear_capture();
        sort_arr.prepare_for_sort(self.current_algorithm.load(Relaxed));
        sort_arr.force_sort();
        player.set_capture(sort_arr.dump_capture());
    }

    /// Returns `true` if the sorting array is correctly sorted.
    pub fn is_sorted(&self) -> bool {
        self.player.lock().is_sorted()
    }

    /// Computes the sort.
    pub fn compute(&self) {
        self.computing.store(true, Relaxed);

        // prepare the array
        self.sort_arr
            .lock()
            .prepare_for_sort(self.current_algorithm.load(Relaxed));

        let player = Arc::clone(&self.player);
        let arr = Arc::clone(&self.sort_arr);
        let process = Arc::clone(&self.process);
        let computing = Arc::clone(&self.computing);
        let ap_tx = Arc::clone(&self.auto_play_ch.0);
        let curr_algo = Arc::clone(&self.current_algorithm);
        let prev = Arc::clone(&self.previous_algorithm);

        self.thread_pool.execute(move || {
            let mut arr = arr.lock();
            process.lock().sort(&mut arr);
            player.lock().set_capture(arr.dump_capture());

            drop(arr);

            if let Some(prev) = prev.lock().take() {
                curr_algo.store(prev, Relaxed);
            }

            computing.store(false, Relaxed);
            _ = ap_tx.send(());
        });
    }

    /// Starts a shuffle.
    pub fn shuffle(&mut self) {
        *self.previous_algorithm.lock() = Some(
            self.current_algorithm
                .swap(SortingAlgorithm::Shuffle, Relaxed),
        );

        self.is_shuffling = true;

        self.compute();
    }

    pub fn increase_speed(&self) {
        let mut player = self.player.lock();

        let speed = player.speed();
        player.set_speed((speed + 0.02).min(5.0));
    }

    pub fn decrease_speed(&self) {
        let mut player = self.player.lock();

        let speed = player.speed();
        player.set_speed((speed - 0.02).max(-5.0));
    }

    pub fn play(&self) {
        let mut player = self.player.lock();
        if player.at_end() {
            player.stop();
        }

        player.play();
    }

    pub fn pause(&self) {
        self.player.lock().pause();
    }

    pub fn stop(&self) {
        self.player.lock().stop();
    }

    pub fn is_playing(&self) -> bool {
        self.player.lock().is_playing()
    }

    pub fn current_algorithm(&self) -> String {
        self.current_algorithm.load(Relaxed).to_string()
    }

    pub fn toggle_audio_processing(&mut self) {
        // TODO: rather than stopping/starting audio processing, this should
        // probably just control a volume level and/or prevent voices from being
        // generated.

        self.audio_playing = !self.audio_playing;
        if self.audio_playing {
            println!("Unmuted audio");
            _ = self.audio_stream.send(Audio::start);
        }
        else {
            _ = self.audio_stream.send(Audio::stop);
            self.audio_voice_counter.store(0, Relaxed);
            self.dsp_load.store(0.0, Relaxed);
            println!("Muted audio");
        }
    }

    pub fn shuffle_and_sort(&mut self) {
        self.shuffle();
        self.sort_after_shuffle = true;
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
        Key::F => model.force_sort(),
        Key::M => model.toggle_audio_processing(),
        Key::N => {
            if app.keys.mods.shift() {
                model.previous_algorithm();
            }
            else {
                model.next_algorithm();
            }
            model.shuffle_and_sort();
        }
        _ => {}
    }
}
