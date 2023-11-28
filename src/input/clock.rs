pub mod real_time_clock;
use std::sync::{RwLock, Arc};
use self::real_time_clock::RealTimeClock;

pub struct Clock {
    proc_clock: Arc<RwLock<f32>>,
    real_time_clock: Arc<RwLock<RealTimeClock>>
}

impl Clock {
    pub fn new() -> Self {
        Self {
            proc_clock: Arc::new(RwLock::new(0.0)),
            real_time_clock: Arc::new(RwLock::new(RealTimeClock::new()))
        }
    }

    pub fn proc_clock(&self) -> Arc<RwLock<f32>> {
        Arc::clone(&self.proc_clock)
    }

    pub fn real_time_clock(&self) -> Arc<RwLock<RealTimeClock>> {
        Arc::clone(&self.real_time_clock)
    }
}