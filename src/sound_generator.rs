mod note_listener;
mod note_config;
pub mod oscilator;
mod adsr_envelope;

use std::sync::{mpsc::{self, Receiver}, Arc, Mutex};
use note_listener::NoteListener;
use oscilator::Oscilator;
use adsr_envelope::ADSREnvelope;

pub struct SampleGenerator {
    clock: Arc<Mutex<f32>>,
    time_step: f32,
    amplitude: f32,
    receiver: Receiver<(Vec<f32>, f32)>,
    oscilator: Oscilator,
}

impl SampleGenerator {
    pub fn new(sample_rate: u16, amplitude: f32, octave: usize, oscilator: Oscilator) -> Self {
        let time_step = 1.0 / sample_rate as f32;

        let (tx, rx) = mpsc::sync_channel(2);
        let clock = Arc::new(Mutex::new(0.0));

        let envelope = ADSREnvelope::new();
        let listener = NoteListener::new(tx, envelope);

        listener.start_listen(octave, clock.clone());

        Self { 
            amplitude, 
            time_step,
            clock, 
            receiver: rx,
            oscilator,
        }
    }
}

impl Iterator for SampleGenerator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let hz = self.receiver.recv().unwrap();

        let mut time = self.clock.lock().unwrap();

        let next_sample = self.oscilator.calc_next_sample(self.amplitude, *time, hz.0);

        *time += self.time_step;

        Some(next_sample)
    }
}