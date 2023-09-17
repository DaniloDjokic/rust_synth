pub struct SampleGenerator {
    sample_rate: f32,
    clock: f32,
    current_hz: f32
}

impl SampleGenerator {
    pub fn new(sample_rate: f32) -> Self {
        Self { sample_rate: sample_rate, clock: 0.0, current_hz: 440.0 }
    }
}

impl Iterator for SampleGenerator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.clock = (self.clock + 1.0) % self.sample_rate;
        let next_sample = (self.clock * self.current_hz * 2.0 * std::f32::consts::PI / self.sample_rate).sin();

        Some(next_sample)
    }
}