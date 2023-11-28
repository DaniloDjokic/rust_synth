pub struct LivePerformanceInfo {
    pub proc_time: f32,
    pub real_time: f32,
}

impl LivePerformanceInfo {
    pub fn new(proc_time: f32, real_time: f32) -> Self {
        Self { 
            proc_time, 
            real_time, 
        }
    }

    pub fn latency(&self) -> f32 {
        self.real_time - self.proc_time
    }
}

pub struct LiveNoteInfo {
    pub note_count: u32,
}