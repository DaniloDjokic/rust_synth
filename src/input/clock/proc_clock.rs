pub struct ProcClock {
    time_step: f32,
    current_time: f32,
}

impl ProcClock {
    pub fn new (sample_rate: u32) -> Self {
        Self { time_step: 1.0 / sample_rate as f32, current_time: 0.0 }
    }

    pub fn get_time(&self) -> f32 {
        self.current_time
    }

    pub fn tick(&mut self) {
        self.current_time += self.time_step;
    }
}