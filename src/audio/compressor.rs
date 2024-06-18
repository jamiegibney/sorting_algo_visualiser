use super::*;

pub const DEFAULT_ATTACK_TIME: f32 = 30.0;
pub const DEFAULT_RELEASE_TIME: f32 = 500.0;

#[derive(Debug, Clone)]
pub struct Compressor {
    sample_rate: f32,

    threshold_db: f32,

    ratio: f32,

    knee_width: f32,

    filter: BallisticsFilter,
}

impl Compressor {
    /// Creates a new `Compressor` which can support `num_channels` channels.
    pub fn new(num_channels: usize, sample_rate: f32) -> Self {
        Self {
            sample_rate,
            threshold_db: 0.0,
            ratio: 1.0,
            knee_width: 0.0,
            filter: BallisticsFilter::new(num_channels, sample_rate)
                .with_attack_time(DEFAULT_ATTACK_TIME)
                .with_release_time(DEFAULT_RELEASE_TIME),
        }
    }

    /// Provides a threshold (in decibels) to the `Compressor`.
    ///
    /// # Panics
    ///
    /// Panics if `level_db` is greater than `0.0`.
    pub fn with_threshold_db(mut self, level_db: f32) -> Self {
        self.set_threshold_db(level_db);
        self
    }

    /// Provides a ratio to the `Compressor`. Any values over `100.0` are
    /// clamped to `100.0`.
    ///
    /// # Panics
    ///
    /// Panics if `ratio < 1.0`.
    pub fn with_ratio(mut self, ratio: f32) -> Self {
        self.set_ratio(ratio);
        self
    }

    /// Provides a knee width to the `Compressor`.
    ///
    /// # Panics
    ///
    /// Panics if `width` is negative.
    pub fn with_knee_width(mut self, width: f32) -> Self {
        self.set_knee_width(width);
        self
    }

    /// Sets the compressor's threshold in decibels.
    ///
    /// # Panics
    ///
    /// Panics if `level_db` is greater than `0.0`.
    pub fn set_threshold_db(&mut self, level_db: f32) {
        assert!(level_db <= 0.0);
        self.threshold_db = level_db;
    }

    /// Sets the ratio of the compressor. Any values over `100.0` are clamped to
    /// `100.0`.
    ///
    /// # Panics
    ///
    /// Panics if `ratio` is less than `1.0`.
    pub fn set_ratio(&mut self, ratio: f32) {
        assert!(ratio >= 1.0);
        self.ratio = ratio.clamp(1.0, 100.0);
    }

    /// Sets the compressor's knee width.
    ///
    /// # Panics
    ///
    /// Panics if `width` is negative.
    pub fn set_knee_width(&mut self, width: f32) {
        assert!(width.is_sign_positive());
    }

    /// Sets the compressor's attack time in milliseconds.
    ///
    /// # Panics
    ///
    /// Panics if `time_ms` is negative.
    pub fn set_attack_time_ms(&mut self, time_ms: f32) {
        self.filter.set_attack_time_ms(time_ms);
    }

    /// Sets the compressor's release time in milliseconds.
    ///
    /// # Panics
    ///
    /// Panics if `time_ms` is negative.
    pub fn set_release_time_ms(&mut self, time_ms: f32) {
        self.filter.set_release_time_ms(time_ms);
    }

    fn gain_function(&self, input: f32) -> f32 {
        let Self { threshold_db: thresh, ratio, knee_width: width, .. } = self;
        let half_width = width * 0.5;

        // below the knee
        if input <= (thresh - half_width) {
            0.0
        }
        // within the knee
        else if (thresh - half_width) < input
            && input <= (thresh + half_width)
        {
            (2.0 * width).recip()
                * (ratio.recip() - 1.0)
                * (input - thresh + half_width).powi(2)
        }
        // above the knee
        else {
            (ratio.recip() - 1.0) * (input - thresh)
        }
    }

    #[inline]
    fn gain_function_simd(&self, sample: f32x2) -> f32x2 {
        let l = self.gain_function(sample[CH_L]);
        let r = self.gain_function(sample[CH_R]);

        f32x2::from_array([l, r])
    }
}

impl SimdAudioEffect for Compressor {
    #[inline]
    fn tick(&mut self, sample: f32x2) -> f32x2 {
        let env = self.filter.tick(sample);
        let gain = Audio::db_to_level_simd(
            self.gain_function_simd(Audio::level_to_db_simd(env)),
        );

        gain * sample
    }

    fn sample_rate(&self) -> f32 {
        self.sample_rate
    }
}
