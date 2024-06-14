use super::*;

#[derive(Debug)]
pub struct BallisticsFilter {
    /// A buffer for storing the last set of output samples.
    y_old: Vec<f32>,

    /// The "constant time envelope" attack level.
    cte_attack: f32,
    /// The "constant time envelope" release level.
    cte_release: f32,

    /// The internal sample rate.
    sample_rate: f32,
}

impl BallisticsFilter {
    /// Creates a new `BallisticsFilter` which can store `num_channels` samples.
    pub fn new(num_channels: usize, sample_rate: f32) -> Self {
        Self {
            y_old: vec![0.0; num_channels],

            cte_attack: 0.0,
            cte_release: 0.0,

            sample_rate,
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
        self.y_old.fill(0.0);
    }

    /// Sets the attack time of the filter in milliseconds.
    ///
    /// Values less than `0.001` ms (`1.0` µs) are automatically snapped to `0.0`.
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
    /// Values less than `0.001` ms (`1.0` µs) are automatically snapped to `0.0`.
    ///
    /// # Panics
    ///
    /// Panics if `time_ms` is negative.
    pub fn set_release_time_ms(&mut self, time_ms: f32) {
        assert!(time_ms.is_sign_positive());
        self.cte_release = self.calculate_cte(time_ms);
    }

    /// Calculates the constant time envelope ("CTE") value for the given period.
    ///
    /// Values less than `0.001` ms (`1.0` µs) are automatically snapped to `0.0`.
    fn calculate_cte(&self, time_ms: f32) -> f32 {
        if time_ms < 0.001 {
            0.0
        } else {
            ((-TAU * 1000.0 / self.sample_rate) / time_ms).exp()
        }
    }
}

impl AudioEffect for BallisticsFilter {
    fn tick(&mut self, channel: usize, mut sample: f32) -> f32 {
        const CH_L: usize = 0;
        const CH_R: usize = 1;

        sample = sample.abs();

        // obtain the correct CTE values
        let cte = if sample > self.y_old[channel] {
            self.cte_attack
        } else {
            self.cte_release
        };

        // process the samples
        let out = sample + cte * (self.y_old[CH_L] - sample);

        // store them for the next call
        self.y_old[channel] = out;

        out
    }

    fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    fn num_channels(&self) -> usize {
        self.y_old.len()
    }
}
