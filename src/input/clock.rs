use std::{time::SystemTime, sync::{RwLock, Arc}};

pub struct Clock {
    clock: Arc<RwLock<f32>>,
    real_time_timestamp: SystemTime,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            clock: Arc::new(RwLock::new(0.0)),
            real_time_timestamp: SystemTime::now()
        }
    }

    pub fn proc_clock(&self) -> Arc<RwLock<f32>> {
        Arc::clone(&self.clock)
    }

    pub fn real_time_passed(&self) -> f32 {
        SystemTime::now()
            .duration_since(self.real_time_timestamp)
            .unwrap()
            .as_secs_f32()
    }
}