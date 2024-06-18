use super::*;

#[derive(Debug, Clone)]
struct CoefsSimd {
    a0: f32x2,
    a1: f32x2,
    b1: f32x2,
}

impl CoefsSimd {
    const fn identity() -> Self {
        let simd_0 = f32x2::from_array([0.0, 0.0]);
        Self { a0: f32x2::from_array([1.0, 1.0]), a1: simd_0, b1: simd_0 }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum FilterType {
    #[default]
    Lowpass,
    Highpass,
}

/// A first-order filter using a two-lane SIMD type for stereo processing.
#[derive(Debug, Clone)]
pub struct FilterSimd {
    filter_type: FilterType,

    coefs: CoefsSimd,
    z1: f32x2,

    freq: f32x2,

    sample_rate: f32x2,
}

impl FilterSimd {
    /// Creates a new filter.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            filter_type: FilterType::default(),
            coefs: CoefsSimd::identity(),
            z1: f32x2::splat(0.0),
            freq: f32x2::splat(0.0),
            sample_rate: f32x2::splat(sample_rate),
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
        self.coefs = CoefsSimd::identity();

        self.z1[CH_L] = 0.0;
        self.z1[CH_R] = 0.0;
        self.freq[CH_L] = 0.0;
        self.freq[CH_R] = 0.0;
    }

    /// Sets the frequency of the filter.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if `freq > Self::sample_rate() * 0.5` or if `freq`
    /// is negative.
    pub fn set_freq(&mut self, freq: f32) {
        debug_assert!(
            freq.is_sign_positive() && freq <= self.sample_rate[CH_L] * 0.5
        );

        self.freq[CH_L] = freq;
        self.freq[CH_R] = freq;
        self.set_coefs();
    }

    /// Sets the type of the filter.
    pub fn set_type(&mut self, filter_type: FilterType) {
        self.filter_type = filter_type;
    }

    fn set_coefs(&mut self) {
        let Self { freq: w, sample_rate: sr, .. } = *self;
        let is_lowpass = matches!(self.filter_type, FilterType::Lowpass);

        let phi = (SIMD_TAU * w) / sr;
        let phi_sin = phi.sin();
        let phi_cos = phi.cos();

        let b1 = (-phi_cos) / (SIMD_ONE + phi_sin);
        let a0 = (SIMD_ONE + if is_lowpass { b1 } else { -b1 }) * SIMD_HALF;

        self.coefs.b1 = b1;
        self.coefs.a0 = a0;
        self.coefs.a1 = if is_lowpass { a0 } else { -a0 };
    }
}

impl SimdAudioEffect for FilterSimd {
    #[inline]
    fn tick(&mut self, sample: f32x2) -> f32x2 {
        let CoefsSimd { a0, a1, b1 } = self.coefs;

        let out = a0.mul_add(sample, self.z1);
        self.z1 = a1.mul_add(sample, -b1 * out);

        out
    }

    fn sample_rate(&self) -> f32 {
        self.sample_rate[CH_L]
    }
}
