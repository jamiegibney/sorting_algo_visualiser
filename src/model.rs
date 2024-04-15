use super::*;
use nannou::prelude::*;
use std::f32::consts::{FRAC_PI_2, TAU};

pub const NUM_SLICES: usize = 20;
pub const CIRCLE_RADIUS: f32 = 300.0;

pub struct Model {
    window_id: WindowId,
    process: Process,

    // arrays
    vertex_arr: Vec<Vec3>,
    index_arr: Vec<usize>,
    color_arr: Vec<Rgb<f32>>,
    color_index_arr: Vec<usize>,
    sorting_arr: SortArray,
}

impl Model {
    pub fn new(app: &App) -> Self {
        let window_id = app
            .new_window()
            .view(super::view)
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
            index_arr: vec![0; NUM_SLICES * 3],
            color_arr: vec![Rgb::new(0.0, 0.0, 0.0); NUM_SLICES],
            color_index_arr: sorting_vec,
            sorting_arr,
        };

        s.set_mesh_vertices();
        s.set_mesh_indices();
        s.set_color_array();

        s
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

    fn set_mesh_indices(&mut self) {
        let get_idx = |mut i: usize| {
            if i > NUM_SLICES {
                i -= NUM_SLICES;
            }

            i
        };

        for i in 0..NUM_SLICES {
            self.index_arr[i * 3] = 0;
            self.index_arr[i * 3 + 1] = get_idx(i + 1);
            self.index_arr[i * 3 + 2] = get_idx(i + 2);
        }
    }

    fn set_color_array(&mut self) {
        for i in 0..NUM_SLICES {
            let t = i as f32 / NUM_SLICES as f32;
            let h = t * 360.0;
            self.color_arr[i] = hsl_to_rgb(h, 1.0, 0.5);
        }
    }

    pub fn update(&mut self, app: &App, update: &Update) {
        if let Ok(guard) = self.sorting_arr.lock() {
            self.color_index_arr.copy_from_slice(&guard);
        }
    }

    pub fn draw(&self, draw: &Draw, frame: &Frame) {
        // TODO(jamiegibney): currently the centre point is always red, but should be the same
        // colour as the slice. this would require more vertices/indices in the below iterators
        draw.mesh()
            .indexed_colored(
                (0..=NUM_SLICES).map(|i| {
                    if i == 0 {
                        (self.vertex_arr[0], self.color_arr[0])
                    }
                    else {
                        let vert = self.vertex_arr[i];
                        let color_idx = self.color_index_arr[i - 1];
                        let color = self.color_arr[self.color_index_arr[i - 1]];

                        (vert, color)
                    }
                }),
                self.index_arr.iter().copied(),
            )
            .xy(Vec2::ZERO);
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
