#![allow(clippy::suboptimal_flops)]

use super::*;
use std::{
    f32::consts::{FRAC_PI_2, TAU},
    marker::PhantomData as PD,
    ops::Rem,
};

pub const DEFAULT_RESOLUTION: usize = 256;
pub const MAX_RESOLUTION: usize = 1 << 14; // 16384
pub const CIRCLE_RADIUS: f32 = 300.0;

pub const SWAP_COLOR: Rgb<f32> =
    Rgb { red: 0.9, green: 1.0, blue: 0.9, standard: PD };
pub const COMPARE_TRUE_COLOR: Rgb<f32> =
    Rgb { red: 1.0, green: 1.0, blue: 1.0, standard: PD };
pub const COMPARE_FALSE_COLOR: Rgb<f32> =
    Rgb { red: 0.0, green: 0.0, blue: 0.0, standard: PD };

#[derive(Clone, Copy, Debug)]
pub enum Overlay {
    Override(Rgb<f32>),
    Invert,
    Darken(f32),
    Lighten(f32),
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
    overlay_operations: Arc<[SortOperation]>,
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
            overlay_operations: [].into(),
        };

        s.set_mesh_vertices();
        s.set_color_array();

        s
    }

    /// Resizes the color wheel.
    pub fn resize(&mut self, new_resolution: usize) {
        self.overlay_operations = [].into();

        self.vertices = vec![Vec3::ZERO; new_resolution + 1];
        self.indices = (0..new_resolution * 3).collect();
        self.overlay_colors = vec![None; new_resolution];
        self.colors = vec![Rgb::new(0.0, 0.0, 0.0); new_resolution];
        self.color_indices = (0..new_resolution).collect();

        self.set_mesh_vertices();
        self.set_color_array();
    }

    /// Provides a slice of operations which will be used to draw an overlay.
    pub fn set_overlay_ops(&mut self, operations: Arc<[SortOperation]>) {
        self.overlay_operations = operations;
    }

    /// Returns a mutable reference to the color index array.
    pub fn arr_mut(&mut self) -> &mut [usize] {
        &mut self.color_indices
    }

    /// Clears the overlay colors.
    pub fn clear_overlay(&mut self) {
        self.overlay_colors.fill(None);
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

    fn invert_color(color: Rgb<f32>) -> Rgb<f32> {
        let mut hsl = rgb_to_hsl(color);
        hsl.0 = (hsl.0 + 180.0).rem(360.0);
        rgb_from_hsl(hsl)
    }

    fn darken_color(color: Rgb<f32>, darken_amount: f32) -> Rgb<f32> {
        let mut hsl = rgb_to_hsl(color);
        hsl.2 *= 1.0 - darken_amount.clamp(0.0, 1.0);
        rgb_from_hsl(hsl)
    }

    fn lighten_color(color: Rgb<f32>, lighten_amount: f32) -> Rgb<f32> {
        let mut hsl = rgb_to_hsl(color);
        hsl.2 *= 1.0 + lighten_amount.clamp(0.0, 1.0);
        rgb_from_hsl(hsl)
    }
}

impl Updatable for ColorWheel {
    fn update(&mut self, _: &App, _: UpdateData) {
        self.clear_overlay();

        for &op in self.overlay_operations.iter() {
            match op {
                SortOperation::Compare { a, b, res } => {
                    let overlay = if res {
                        Overlay::Lighten(0.5)
                    }
                    else {
                        Overlay::Darken(0.2)
                    };

                    self.overlay_colors[a] = Some(overlay);
                    self.overlay_colors[b] = Some(overlay);
                }
                SortOperation::Swap { a, b } => {
                    let overlay = Overlay::Lighten(0.1);
                    self.overlay_colors[a] = Some(overlay);
                    self.overlay_colors[b] = Some(overlay);
                }
                SortOperation::Write { idx, .. } => {
                    self.overlay_colors[idx] = Some(Overlay::Darken(0.7));
                }
                SortOperation::Read { idx } => {
                    self.overlay_colors[idx] = Some(Overlay::Lighten(0.3));
                }
            }
        }
    }
}

impl Drawable for ColorWheel {
    fn draw(&self, draw: &Draw, _: UpdateData) {
        draw.translate(vec3(0.0, 50.0, 0.0))
            .mesh()
            .indexed_colored(
                (0..self.resolution() * 3).map(|i| {
                    let color_idx = self.color_indices[i / 3];

                    let color = self.overlay_colors[color_idx].map_or(
                        self.colors[color_idx],
                        |o| match o {
                            Overlay::Override(c) => c,
                            Overlay::Invert => {
                                Self::invert_color(self.colors[color_idx])
                            }
                            Overlay::Darken(amt) => {
                                Self::darken_color(self.colors[color_idx], amt)
                            }
                            Overlay::Lighten(amt) => {
                                Self::lighten_color(self.colors[color_idx], amt)
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
}

fn rgb_from_hsl(hsl: (f32, f32, f32)) -> Rgb<f32> {
    hsl_to_rgb(hsl.0, hsl.1, hsl.2)
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

/// Converts a set of `r` (red), `g` (green), and `b` (blue) values
/// values to an HSL value.
///
/// [Source](https://www.rapidtables.com/convert/color/rgb-to-hsl.html)
fn rgb_to_hsl(color: Rgb<f32>) -> (f32, f32, f32) {
    let Rgb { red, green, blue, .. } = color;

    let c_max = red.max(blue.max(green));
    let c_min = red.min(blue.min(green));
    let delta = c_max - c_min;

    let l = (c_max + c_min) * 0.5;

    let s =
        if delta == 0.0 { 0.0 } else { delta / (1.0 - (2.0 * l - 1.0).abs()) };

    let h = 60.0
        * if red > blue && red > green {
            ((green - blue) / delta).rem(6.0)
        }
        else if green > red && green > blue {
            (blue - red) / delta + 2.0
        }
        else if blue > red && blue > green {
            (red - green) / delta + 4.0
        }
        else {
            0.0
        };

    (h, s, l)
}
