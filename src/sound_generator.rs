mod note_listener;
mod note_config;
pub mod oscilator;
mod adsr_envelope;

use adsr_envelope::ADSREnvelope;
use std::sync::{mpsc::{self, Receiver}, Arc, RwLock};
use note_listener::NoteListener;
use oscilator::Oscilator;

pub struct SampleGenerator {
    clock: Arc<RwLock<f32>>,
    time_step: f32,
    amplitude: f32,
    receiver: Receiver<(Vec<f32>, Option<f32>)>,
    oscilator: Oscilator,
    envelope: ADSREnvelope,
}

impl SampleGenerator {
    pub fn new(sample_rate: u16, amplitude: f32, octave: usize, oscilator: Oscilator) -> Self {
        let clock = Arc::new(RwLock::new(0.0));
        let time_step = 1.0 / sample_rate as f32;

        let (tx, rx) = mpsc::sync_channel(2);

        let envelope = ADSREnvelope::new();

        let listener = NoteListener::new(tx);
        listener.start_listen(octave, Arc::clone(&clock));

        Self { 
            amplitude: amplitude, 
            time_step: time_step,
            clock, 
            receiver: rx,
            oscilator,
            envelope
        }
    }
}

impl Iterator for SampleGenerator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let hz = self.receiver.recv().unwrap();

        if let Some(time) = hz.1 {
            self.envelope.set_envelope(hz.0.len() > 0, time);
        }

        let next_amplitude = self.envelope.get_amplitude(*self.clock.read().unwrap());
        let next_hz = self.oscilator.calc_next_sample(self.amplitude, *self.clock.read().unwrap(), hz.0);

        *self.clock.write().unwrap() += self.time_step;

        let next_sample = next_amplitude * next_hz;
        Some(next_sample)
    }
}