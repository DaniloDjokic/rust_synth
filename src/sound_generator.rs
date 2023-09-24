mod note_listener;
mod note_config;
pub mod oscilator;

use std::sync::mpsc::{self, Receiver};
use note_listener::NoteListener;
use oscilator::Oscilator;

pub struct SampleGenerator {
    clock: f32,
    time_step: f32,
    amplitude: f32,
    receiver: Receiver<Vec<f32>>,
    oscilator: Oscilator
}

impl SampleGenerator {
    pub fn new(sample_rate: u16, amplitude: f32, octave: usize, oscilator: Oscilator) -> Self {
        let time_step = 1.0 / sample_rate as f32;

        let (tx, rx) = mpsc::sync_channel(2);

        let listener = NoteListener::new(tx);
        listener.start_listen(octave);

        Self { 
            amplitude: amplitude, 
            time_step: time_step,
            clock: 1.0, 
            receiver: rx,
            oscilator
        }
    }
}

impl Iterator for SampleGenerator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let hz = self.receiver.recv().unwrap();

        let next_sample = self.oscilator.calc_next_sample(self.amplitude, self.clock, hz);

        self.clock += self.time_step;

        Some(next_sample)
    }
}