use super::*;

/// A stereo ballistics filter used for envelope following, which uses a
/// two-lane SIMD type for stereo processing.
#[derive(Debug, Clone)]
pub struct BallisticsFilter {
    /// A buffer for storing the last set of output samples.
    z1: f32x2,

    /// The "constant time envelope" attack level.
    cte_attack: f32,
    /// The "constant time envelope" release level.
    cte_release: f32,

    /// The internal sample rate.
    sample_rate: f32x2,
}

impl BallisticsFilter {
    /// Creates a new `BallisticsFilter` which can store `num_channels` samples.
    pub fn new(num_channels: usize, sample_rate: f32) -> Self {
        Self {
            z1: SIMD_ZERO,

            cte_attack: 0.0,
            cte_release: 0.0,

            sample_rate: f32x2::splat(sample_rate),
        }
    }

    /// Provides an attack time (in milliseconds) for the filter.
    ///
    /// # Panics
    ///
    /// Panics if `time_ms` is negative.
    pub fn with_attack_time(mut self, time_ms: f32) -> Self {
        self.set_attack_time_ms(time_ms);
        self
    }

    /// Provides a release time (in milliseconds) for the filter.
    ///
    /// # Panics
    ///
    /// Panics if `time_ms` is negative.
    pub fn with_release_time(mut self, time_ms: f32) -> Self {
        self.set_release_time_ms(time_ms);
        self
    }

    /// Resets the internal buffer to `0.0`.
    pub fn reset(&mut self) {
        self.z1[CH_L] = 0.0;
        self.z1[CH_R] = 0.0;
    }

    /// Sets the attack time of the filter in milliseconds.
    ///
    /// Values less than `0.001` ms (`1.0` µs) are automatically snapped to
    /// `0.0`.
    ///
    /// # Panics
    ///
    /// Panics if `time_ms` is negative.
    pub fn set_attack_time_ms(&mut self, time_ms: f32) {
        assert!(time_ms.is_sign_positive());
        self.cte_attack = self.calculate_cte(time_ms);
    }

    /// Sets the release time of the filter in milliseconds.
    ///
    /// Values less than `0.001` ms (`1.0` µs) are automatically snapped to
    /// `0.0`.
    ///
    /// # Panics
    ///
    /// Panics if `time_ms` is negative.
    pub fn set_release_time_ms(&mut self, time_ms: f32) {
        assert!(time_ms.is_sign_positive());
        self.cte_release = self.calculate_cte(time_ms);
    }

    /// Calculates the constant time envelope ("CTE") value for the given
    /// period.
    ///
    /// Values less than `0.001` ms (`1.0` µs) are automatically snapped to
    /// `0.0`.
    fn calculate_cte(&self, time_ms: f32) -> f32 {
        if time_ms < 0.001 {
            0.0
        }
        else {
            ((-TAU * 1000.0 / self.sample_rate[CH_L]) / time_ms).exp()
        }
    }

    #[inline]
    fn simd_cte(&self, sample: f32x2) -> f32x2 {
        let mask = sample.simd_gt(self.z1);

        let cte_l = if unsafe { mask.test_unchecked(CH_L) } {
            self.cte_attack
        }
        else {
            self.cte_release
        };
        let cte_r = if unsafe { mask.test_unchecked(CH_R) } {
            self.cte_attack
        }
        else {
            self.cte_release
        };

        f32x2::from_array([cte_l, cte_r])
    }
}

impl SimdAudioEffect for BallisticsFilter {
    #[inline]
    fn tick(&mut self, mut sample: f32x2) -> f32x2 {
        sample = sample.abs();

        // obtain the correct CTE values
        let cte = self.simd_cte(sample);

        // process the samples
        let out = sample + cte * (self.z1 - sample);

        // store them for the next call
        self.z1 = out;

        out
    }

    fn sample_rate(&self) -> f32 {
        self.sample_rate[CH_L]
    }
}
