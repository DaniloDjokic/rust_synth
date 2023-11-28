use std::time::SystemTime;

pub struct RealTimeClock {
    start: SystemTime,
    last_timestamp: SystemTime,
}

impl RealTimeClock {
    pub fn new () -> Self {
        Self { start: SystemTime::now(), last_timestamp: SystemTime::now() }
    }

    pub fn real_time_elapsed(&mut self) -> f32 {
        let elapsed = self.time_elapsed();
        
        self.last_timestamp = SystemTime::now();

        elapsed
    }

    fn time_elapsed(&self) -> f32 {
        SystemTime::now()
            .duration_since(self.last_timestamp)
            .unwrap()
            .as_secs_f32()
    }

    pub fn total_real_time_elapsed(&self) -> f32 {
        self.time_elapsed()
    }
}