mod input_listener;
mod note_config;
pub mod oscilator;
mod adsr_envelope;

use adsr_envelope::ADSREnvelope;
use std::sync::{mpsc::{self, Receiver}, Arc, RwLock};
use input_listener::InputListener;
use input_listener::InputEventData;
use oscilator::Oscilator;

pub struct SampleGenerator {
    clock: Arc<RwLock<f32>>,
    time_step: f32,
    amplitude: f32,
    receiver: Receiver<InputEventData>,
    oscilator: Oscilator,
    envelope: ADSREnvelope,
    buffered_hz: Vec<f32>,
}

impl SampleGenerator {
    pub fn new(sample_rate: u16, amplitude: f32, octave: usize, oscilator: Oscilator) -> Self {
        let clock = Arc::new(RwLock::new(0.0));
        let time_step = 1.0 / sample_rate as f32;

        let (tx, rx) = mpsc::sync_channel(2);

        let envelope = ADSREnvelope::new();

        let listener = InputListener::new(tx);
        listener.start_listen(octave, Arc::clone(&clock));

        Self { 
            amplitude: amplitude, 
            time_step: time_step,
            clock, 
            receiver: rx,
            oscilator,
            envelope,
            buffered_hz: vec![],
        }
    }
}

impl Iterator for SampleGenerator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let event_data = self.receiver.recv().unwrap();
        if event_data.hz.len() != 0 {
            self.buffered_hz = event_data.hz.clone();
        }

        if let Some(time) = event_data.time {
            if event_data.hz.len() > 0 {
                self.envelope.set_note_on(time);
            }
            else {
                self.envelope.set_note_off(time);
            }
        }

        let next_amplitude = self.envelope.get_amplitude(*self.clock.read().unwrap());
        let next_hz;

        if next_amplitude != 0.0 && event_data.hz.len() == 0 {
            next_hz = self.oscilator.calc_next_sample(self.amplitude, *self.clock.read().unwrap(), self.buffered_hz.clone());
        }
        else {
            next_hz = self.oscilator.calc_next_sample(self.amplitude, *self.clock.read().unwrap(), event_data.hz);
        }

        *self.clock.write().unwrap() += self.time_step;

        let next_sample = next_amplitude * next_hz;
        Some(next_sample)
    }
}