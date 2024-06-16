use super::*;

#[derive(Debug, Clone)]
struct Coefs {
    a0: f32,
    a1: f32,
    b1: f32,
}

impl Coefs {
    const fn identity() -> Self {
        Self { a0: 1.0, a1: 0.0, b1: 0.0 }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum FilterType {
    #[default]
    Lowpass,
    Highpass,
}

/// A first-order filter.
#[derive(Debug, Clone)]
pub struct Filter {
    filter_type: FilterType,

    coefs: Coefs,
    z1: f32,

    freq: f32,

    sample_rate: f32,
}

impl Filter {
    /// Creates a new filter.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            filter_type: FilterType::default(),
            coefs: Coefs::identity(),
            z1: 0.0,
            freq: 0.0,
            sample_rate,
        }
    }

    /// Provides a frequency to the filter.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if `freq > Self::sample_rate() * 0.5` or if `freq`
    /// is negative.
    pub fn with_freq(mut self, freq: f32) -> Self {
        self.set_freq(freq);
        self
    }

    /// Provides a `FilterType` to the filter.
    pub fn with_type(mut self, filter_type: FilterType) -> Self {
        self.set_type(filter_type);
        self
    }

    /// Resets the filter, including its frequency.
    pub fn reset(&mut self) {
        self.coefs = Coefs::identity();
        self.z1 = 0.0;
        self.freq = 0.0;
    }

    /// Sets the frequency of the filter.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if `freq > Self::sample_rate() * 0.5` or if `freq`
    /// is negative.
    pub fn set_freq(&mut self, freq: f32) {
        debug_assert!(
            freq.is_sign_positive() && freq <= self.sample_rate * 0.5
        );

        self.freq = freq;
        self.set_coefs();
    }

    /// Sets the type of the filter.
    pub fn set_type(&mut self, filter_type: FilterType) {
        self.filter_type = filter_type;
    }

    fn set_coefs(&mut self) {
        let Self { freq: w, sample_rate: sr, .. } = *self;
        let is_lowpass = matches!(self.filter_type, FilterType::Lowpass);
        let (phi_sin, phi_cos) = ((TAU * w) / sr).sin_cos();

        let b1 = (-phi_cos) / (1.0 + phi_sin);
        let a0 = (1.0 + if is_lowpass { b1 } else { -b1 }) * 0.5;

        self.coefs.b1 = b1;
        self.coefs.a0 = a0;
        self.coefs.a1 = if is_lowpass { a0 } else { -a0 };
    }
}

impl AudioEffect for Filter {
    fn tick(&mut self, channel: usize, sample: f32) -> f32 {
        let Coefs { a0, a1, b1 } = self.coefs;

        let out = a0.mul_add(sample, self.z1);
        self.z1 = a1.mul_add(sample, -b1 * out);

        out
    }

    fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    fn num_channels(&self) -> usize {
        1
    }
}
