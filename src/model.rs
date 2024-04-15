use super::*;
use nannou::prelude::*;
use std::f32::consts::{FRAC_PI_2, TAU};

pub const NUM_SLICES: usize = 2048;
pub const CIRCLE_RADIUS: f32 = 300.0;

pub struct Model {
    window_id: WindowId,
    process: Process,

    // arrays
    vertex_arr: Vec<Vec3>,
    index_arr: Vec<usize>,
    color_arr: Vec<Rgb<f32>>,
    color_index_arr: Vec<usize>,
    pub sorting_arr: SortArray,
}

impl Model {
    pub fn new(app: &App) -> Self {
        let window_id = app
            .new_window()
            .view(super::view)
            .key_pressed(key_pressed)
            .size(800, 800)
            .build()
            .expect("failed to initialize main window");

        let sorting_vec: Vec<usize> = (0..NUM_SLICES).collect();
        let sorting_arr = Arc::new(Mutex::new(sorting_vec.clone()));

        let mut s = Self {
            window_id,
            process: Process::new(Arc::clone(&sorting_arr))
                .with_algorithm(SortingAlgorithm::InPlaceRadixLSD4),

            vertex_arr: vec![Vec3::ZERO; NUM_SLICES + 1],
            index_arr: (0..NUM_SLICES * 3).collect(),
            color_arr: vec![Rgb::new(0.0, 0.0, 0.0); NUM_SLICES],
            color_index_arr: sorting_vec,
            sorting_arr,
        };

        s.set_mesh_vertices();
        s.set_color_array();

        s
    }

    pub fn scramble_sort_arr(&mut self) {
        if let Ok(mut guard) = self.sorting_arr.lock() {
            let len = guard.len();
            for i in 0..len {
                let idx_1 = random_range(0, len);
                let idx_2 = random_range(0, len);
                guard.swap(idx_1, idx_2);
            }
        }
    }

    pub fn sort_arr(&mut self) {
        let mut elapsed = 0.0;
        if let Ok(mut guard) = self.sorting_arr.lock() {
            let t = std::time::Instant::now();
            guard.sort_unstable();
            elapsed = t.elapsed().as_secs_f64() * 1000.0;
        }

        println!("sort took {elapsed:.6}ms");
    }

    pub fn update(&mut self, app: &App, update: &Update) {
        if let Ok(guard) = self.sorting_arr.lock() {
            self.color_index_arr.copy_from_slice(&guard);
        }
    }

    pub fn draw(&self, draw: &Draw, frame: &Frame) {
        draw.mesh()
            .indexed_colored(
                (0..NUM_SLICES * 3).map(|i| {
                    let color = self.color_arr[self.color_index_arr[i / 3]];

                    if i % 3 == 0 {
                        (self.vertex_arr[0], color)
                    }
                    else if i % 3 == 1 {
                        (self.vertex_arr[i / 3 + 1], color)
                    }
                    else {
                        let vert_idx = if (i / 3 + 2) > NUM_SLICES {
                            1
                        }
                        else {
                            i / 3 + 2
                        };
                        (self.vertex_arr[vert_idx], color)
                    }
                }),
                self.index_arr.iter().copied(),
            )
            .xy(Vec2::ZERO);
    }

    fn set_mesh_vertices(&mut self) {
        // this is the centre point which each triangle connects to.
        self.vertex_arr[0] = Vec3::ZERO;

        for i in 0..NUM_SLICES {
            let theta = (i as f32 / NUM_SLICES as f32) * TAU + FRAC_PI_2;
            let (y, x) = theta.sin_cos();

            // the first vertex is always at the centre.
            self.vertex_arr[i + 1] =
                Vec3::new(-x * CIRCLE_RADIUS, y * CIRCLE_RADIUS, 0.0);
        }
    }

    fn set_color_array(&mut self) {
        for i in 0..NUM_SLICES {
            let t = i as f32 / NUM_SLICES as f32;
            let h = t * 360.0;
            self.color_arr[i] = hsl_to_rgb(h, 1.0, 0.5);
        }
    }
}

/// [Source](https://www.rapidtables.com/convert/color/hsl-to-rgb.html)
#[allow(clippy::many_single_char_names)]
pub fn hsl_to_rgb(mut h: f32, s: f32, l: f32) -> Rgb<f32> {
    h = h.clamp(0.0, 360.0);
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = l - c * 0.5;

    let c = c + m;
    let x = x + m;

    if (0.0..60.0).contains(&h) {
        Rgb::new(c, x, m)
    }
    else if (60.0..120.0).contains(&h) {
        Rgb::new(x, c, m)
    }
    else if (120.0..180.0).contains(&h) {
        Rgb::new(m, c, x)
    }
    else if (180.0..240.0).contains(&h) {
        Rgb::new(m, x, c)
    }
    else if (240.0..300.0).contains(&h) {
        Rgb::new(x, m, c)
    }
    else if (300.0..=360.0).contains(&h) {
        Rgb::new(c, m, x)
    }
    else {
        unreachable!()
    }
}

pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.scramble_sort_arr(),
        Key::S => model.sort_arr(),
        _ => {}
    }
}
