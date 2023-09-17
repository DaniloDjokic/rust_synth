mod note_listener;
use std::sync::{Arc, Mutex};

use note_listener::NoteListener;

pub struct SampleGenerator {
    sample_rate: f32,
    clock: f32,
    current_note: Arc<Mutex<f32>>,
}

impl SampleGenerator {
    pub fn new(sample_rate: f32) -> Self {
        let current_note = Arc::new(Mutex::new(220.0));

        NoteListener::start_listen(Arc::clone(&current_note));

        Self { sample_rate: sample_rate, clock: 0.0, current_note: current_note}
    }
}

impl Iterator for SampleGenerator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.clock = (self.clock + 1.0) % self.sample_rate;
        let hz  = *self.current_note.lock().unwrap();

        let next_sample = (self.clock * hz * 2.0 * std::f32::consts::PI / self.sample_rate).sin();

        Some(next_sample)
    }
}