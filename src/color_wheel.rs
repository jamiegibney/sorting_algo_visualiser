#![allow(clippy::suboptimal_flops)]

use super::*;
use nannou::{draw::background::new, prelude::*};
use std::{
    f32::consts::{FRAC_PI_2, TAU},
    marker::PhantomData as PD,
};

pub const DEFAULT_RESOLUTION: usize = 256;
pub const MAX_RESOLUTION: usize = 8192;
pub const CIRCLE_RADIUS: f32 = 300.0;

// pub const WRITE_COLOR: Rgb<f32> =
//     Rgb { red: 1.0, green: 1.0, blue: 1.0, standard: PD };
// pub const READ_COLOR: Rgb<f32> =
//     Rgb { red: 0.0, green: 0.8, blue: 1.0, standard: PD };
// pub const SWAP_COLOR: Rgb<f32> =
//     Rgb { red: 0.0, green: 0.0, blue: 0.0, standard: PD };
pub const COMPARE_COLOR: Rgb<f32> =
    Rgb { red: 1.0, green: 1.0, blue: 1.0, standard: PD };

#[derive(Clone, Copy, Debug)]
pub enum Overlay {
    Override(Rgb<f32>),
    Invert,
    Darken,
}

/// The color wheel display.
#[derive(Debug)]
pub struct ColorWheel {
    /// The vertices of each slice of the color wheel.
    vertices: Vec<Vec3>,
    /// The mesh indices for the color wheel.
    indices: Vec<usize>,
    /// Any overlay colors for the sorting process.
    overlay_colors: Vec<Option<Overlay>>,
    /// The original array of colors.
    colors: Vec<Rgb<f32>>,
    /// The indices for each slice's color — copied from the sorting array.
    color_indices: Vec<usize>,
}

impl ColorWheel {
    /// Creates a new `ColorWheel`.
    pub fn new() -> Self {
        let mut s = Self {
            vertices: vec![Vec3::ZERO; DEFAULT_RESOLUTION + 1],
            indices: (0..DEFAULT_RESOLUTION * 3).collect(),
            overlay_colors: vec![None; DEFAULT_RESOLUTION],
            colors: vec![Rgb::new(0.0, 0.0, 0.0); DEFAULT_RESOLUTION],
            color_indices: (0..DEFAULT_RESOLUTION).collect(),
        };

        s.set_mesh_vertices();
        s.set_color_array();

        s
    }

    /// Resizes the color wheel.
    pub fn resize(&mut self, new_resolution: usize) {
        self.vertices = vec![Vec3::ZERO; new_resolution + 1];
        self.indices = (0..new_resolution * 3).collect();
        self.overlay_colors = vec![None; new_resolution];
        self.colors = vec![Rgb::new(0.0, 0.0, 0.0); new_resolution];
        self.color_indices = (0..new_resolution).collect();

        self.set_mesh_vertices();
        self.set_color_array();
    }

    /// Updates the color wheel from the sorting array (see
    /// [`Self::sort_arr_ref`]).
    pub fn update(&mut self, arr: &[usize]) {
        self.color_indices.copy_from_slice(arr);
        self.overlay_colors.fill(None);
    }

    /// Sets any overlay colors from a set of operations.
    pub fn overlay_from(&mut self, operations: Vec<SortOperation>) {
        for op in operations {
            match op {
                SortOperation::Compare { a, b, res } => {
                    if res {
                        self.overlay_colors[a] =
                            Some(Overlay::Override(COMPARE_COLOR));
                        self.overlay_colors[b] =
                            Some(Overlay::Override(COMPARE_COLOR));
                    }
                }
                SortOperation::Swap { a, b } => {
                    self.overlay_colors[a] = Some(Overlay::Darken);
                    self.overlay_colors[b] = Some(Overlay::Darken);
                }
                SortOperation::Write { idx, value } => {
                    self.overlay_colors[idx] = Some(Overlay::Darken);
                }
                SortOperation::Read { idx } => {
                    self.overlay_colors[idx] =
                        Some(Overlay::Override(COMPARE_COLOR));
                }
                SortOperation::Noop => (),
            }
        }
    }

    /// Draws the color wheel to the provided `Draw` instance.
    pub fn draw(&self, draw: &Draw) {
        draw.mesh()
            .indexed_colored(
                (0..self.resolution() * 3).map(|i| {
                    let color_idx = self.color_indices[i / 3];

                    let mut color = self.overlay_colors[color_idx].map_or(
                        self.colors[color_idx],
                        |o| match o {
                            Overlay::Override(c) => c,
                            Overlay::Invert => {
                                Self::invert_color(self.colors[color_idx])
                            }
                            Overlay::Darken => {
                                Self::darken_color(self.colors[color_idx])
                            }
                        },
                    );

                    if i % 3 == 0 {
                        (self.vertices[0], color)
                    }
                    else if i % 3 == 1 {
                        (self.vertices[i / 3 + 1], color)
                    }
                    else {
                        let off = i / 3 + 2;
                        let idx = if off > self.resolution() { 1 } else { off };
                        (self.vertices[idx], color)
                    }
                }),
                self.indices.iter().copied(),
            )
            .xy(Vec2::ZERO);
    }

    /// Precomputes the positions of all of the circle's vertices.
    fn set_mesh_vertices(&mut self) {
        self.vertices[0] = Vec3::ZERO;

        for i in 0..self.resolution() {
            let theta = (i as f32 / self.resolution() as f32) * TAU + FRAC_PI_2;
            let (y, x) = theta.sin_cos();

            self.vertices[i + 1] =
                Vec3::new(-x * CIRCLE_RADIUS, y * CIRCLE_RADIUS, 0.0);
        }
    }

    /// Precomputes the color array — this is the ordered, constant array
    /// of color values.
    fn set_color_array(&mut self) {
        for i in 0..self.resolution() {
            let t = i as f32 / self.resolution() as f32;
            let h = t * 360.0;
            self.colors[i] = hsl_to_rgb(h, 1.0, 0.5);
        }
    }

    /// Returns the resolution of the color wheel.
    fn resolution(&self) -> usize {
        self.colors.len()
    }

    fn invert_color(mut color: Rgb<f32>) -> Rgb<f32> {
        color.red = 1.0 - color.red;
        color.green = 1.0 - color.green;
        color.blue = 1.0 - color.blue;

        color
    }

    fn darken_color(mut color: Rgb<f32>) -> Rgb<f32> {
        // TODO: use HSL?
        color.red *= 0.7;
        color.green *= 0.7;
        color.blue *= 0.7;

        color
    }
}

/// Converts a set of `h` (hue), `s` (saturation), and `l` (luminance)
/// values to an RGB value.
///
/// [Source](https://www.rapidtables.com/convert/color/hsl-to-rgb.html)
#[rustfmt::skip]
#[allow(clippy::many_single_char_names)]
fn hsl_to_rgb(mut h: f32, s: f32, l: f32) -> Rgb<f32> {
    h = h.clamp(0.0, 360.0);

    let mut c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let mut x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = l - c * 0.5;

    c += m;
    x += m;

    match h {
        h if (0.0..=60.0).contains(&h)    => Rgb::new(c, x, m),
        h if (60.0..=120.0).contains(&h)  => Rgb::new(x, c, m),
        h if (120.0..=180.0).contains(&h) => Rgb::new(m, c, x),
        h if (180.0..=240.0).contains(&h) => Rgb::new(m, x, c),
        h if (240.0..=300.0).contains(&h) => Rgb::new(x, m, c),
        h if (300.0..=360.0).contains(&h) => Rgb::new(c, m, x),
        _ => unreachable!(),
    }
}
