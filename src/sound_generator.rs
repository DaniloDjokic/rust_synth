mod input_listener;
mod note;
mod instrument;
pub mod oscilator;
mod adsr_envelope;

use adsr_envelope::ADSREnvelope;
use std::sync::{mpsc::{self, Receiver}, Arc, RwLock};
use input_listener::InputListener;
use input_listener::InputEventData;

use self::instrument::{Instrument, InstrumentTrait};

pub struct SampleGenerator {
    clock: Arc<RwLock<f32>>,
    time_step: f32,
    receiver: Receiver<InputEventData>,
    instrument: Instrument,
}

impl SampleGenerator {
    pub fn new(sample_rate: u16) -> Self {
        let clock = Arc::new(RwLock::new(0.0));
        let time_step = 1.0 / sample_rate as f32;

        let (tx, rx) = mpsc::sync_channel(2);

        let envelope = ADSREnvelope::new();

        let listener = InputListener::new(tx);
        listener.start_listen(Arc::clone(&clock));

        let instrument = Instrument {
            envelope: envelope,
            attack_time: 1.0,
            decay_time: 1.0,
            sustain_amplitude: 0.8,
            release_time: 0.5,
            volume: 1.0,
        };

        Self { 
            time_step: time_step,
            clock, 
            receiver: rx,
            instrument,
        }
    }
}

impl Iterator for SampleGenerator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut event_data = self.receiver.recv().unwrap();

        let mut next_sample = 0.0;
        for note in event_data.notes.iter_mut() {
            let mut note_finished = false;
            let sample = self.instrument.get_next_sample(*self.clock.read().unwrap(), &note, &mut note_finished);
            next_sample += sample;

            if note_finished && note.time_deactivated > note.time_deactivated { 
                note.is_active = false; 
            } 
        }

        event_data.notes.retain(|e| e.is_active);
        *self.clock.write().unwrap() += self.time_step;

        Some(next_sample * 0.5)
    }
}