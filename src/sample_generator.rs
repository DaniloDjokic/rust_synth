mod oscilator;
mod input_listener;
mod note;
mod adsr_envelope;
pub mod instrument;

use std::sync::{mpsc::{self, Receiver, Sender}, Arc, RwLock};
use input_listener::InputListener;
use input_listener::InputEventData;

use self::{
    note::Note, 
    instrument::Instrument
};

pub struct SampleGenerator {
    clock: Arc<RwLock<f32>>,
    time_step: f32,
    master_volume: f32,
    receiver: Receiver<InputEventData>,
    instruments: Vec<Box<dyn Instrument + Send>>,
    sender: Sender<f32>,
}

impl SampleGenerator {
    pub fn new(
        sample_rate: u16, 
        current_sample_sender: Sender<f32>, 
        instruments: Vec<Box<(dyn Instrument + Send)>>
    ) -> Self {
        
        let clock = Arc::new(RwLock::new(0.0));
        let time_step = 1.0 / sample_rate as f32;

        let (tx, receiver) = mpsc::sync_channel(2);

        let listener = InputListener::new(tx);
        listener.start_listen(Arc::clone(&clock));

        Self { 
            master_volume: 0.2,
            time_step, 
            clock, 
            receiver, 
            instruments,
            sender: current_sample_sender
        }
    }

    fn sum_note_samples(&self, note: &mut Note, next_sample: &mut f32) {
        let filtered_instruments: Vec<&Box<dyn Instrument + Send>> = self.instruments.iter()
            .filter(|i| i.get_channel() == note.channel)
            .collect();
        
        filtered_instruments.iter().for_each(|e| {
            let sample = e.get_next_sample(*self.clock.read().unwrap(), &note);

            match sample {
                Some(sample) => *next_sample += sample,
                None => if note.time_deactivated > note.time_activated { note.is_active = false }
            }
        });
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

        next_sample *= self.master_volume;

        event_data.notes.retain(|e| e.is_active);
        *self.clock.write().unwrap() += self.time_step;

        self.sender.send(next_sample).unwrap();

        Some(next_sample)
    }
}