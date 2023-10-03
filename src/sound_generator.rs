mod oscilator;
mod input_listener;
mod note;
mod instrument;
mod adsr_envelope;

use adsr_envelope::ADSREnvelope;
use std::sync::{mpsc::{self, Receiver}, Arc, RwLock};
use input_listener::InputListener;
use input_listener::InputEventData;

use self::{instrument::{Instrument, InstrumentTrait}, note::Note};

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

        let (tx, receiver) = mpsc::sync_channel(2);

        let listener = InputListener::new(tx);
        listener.start_listen(Arc::clone(&clock));

        let instrument = Instrument::new(ADSREnvelope::new());

        Self { time_step, clock, receiver, instrument }
    }

    fn sum_note_samples(&self, note: &mut Note, next_sample: &mut f32){
        let sample = self.instrument.get_next_sample(*self.clock.read().unwrap(), &note);
        match sample {
            Some(sample) => *next_sample += sample,
            None => if note.time_deactivated > note.time_deactivated { note.is_active = false }
        }
    }
}

impl Iterator for SampleGenerator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut event_data = self.receiver.recv().unwrap();

        let mut next_sample = 0.0;
        for note in event_data.notes.iter_mut() {
            self.sum_note_samples(note, &mut next_sample);
        }

        event_data.notes.retain(|e| e.is_active);
        *self.clock.write().unwrap() += self.time_step;

        Some(next_sample)
    }
}