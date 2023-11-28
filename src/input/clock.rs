pub mod real_time_clock;
pub mod proc_clock;
use std::sync::{RwLock, Arc};
use self::{real_time_clock::RealTimeClock, proc_clock::ProcClock};

pub struct Clock {
    proc_clock: Arc<RwLock<ProcClock>>,
    real_time_clock: Arc<RwLock<RealTimeClock>>
}

impl Clock {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            proc_clock: Arc::new(RwLock::new(ProcClock::new(sample_rate))),
            real_time_clock: Arc::new(RwLock::new(RealTimeClock::new()))
        }
    }

    pub fn proc_clock(&self) -> Arc<RwLock<ProcClock>> {
        Arc::clone(&self.proc_clock)
    }

    pub fn real_time_clock(&self) -> Arc<RwLock<RealTimeClock>> {
        Arc::clone(&self.real_time_clock)
    }
}