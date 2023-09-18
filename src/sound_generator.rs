mod note_listener;
use std::sync::mpsc::{self, Receiver};

use note_listener::NoteListener;

pub struct SampleGenerator {
    sample_rate: f32,
    clock: f32,
    receiver: Receiver<f32>,
}

impl SampleGenerator {
    pub fn new(sample_rate: f32) -> Self {
        let (tx, rx) = mpsc::sync_channel(2);

        let listener = NoteListener::new(tx);
        listener.start_listen();

        Self { sample_rate: sample_rate, clock: 0.0, receiver: rx }
    }
}

impl Iterator for SampleGenerator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.clock = (self.clock + 1.0) % self.sample_rate;

        let hz = self.receiver.recv().unwrap();

        let next_sample = (self.clock * hz * 2.0 * std::f32::consts::PI / self.sample_rate).sin();

        Some(next_sample)
    }
}