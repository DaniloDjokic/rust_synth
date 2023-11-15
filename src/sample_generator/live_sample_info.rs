pub struct LiveSynthInfo {
    pub notes_count: usize,
    pub proc_time: f32,
    pub real_time: f32,
}

impl LiveSynthInfo {
    pub fn new(notes_count: usize, proc_time: f32, real_time: f32) -> Self {
        Self { 
            notes_count, 
            proc_time, 
            real_time, 
        }
    }
}