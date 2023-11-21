use std::sync::{Arc, RwLock};

use super::{note::Note, instrument::Instrument};

pub struct NoteCollection {
    pub notes: Vec<Note>,
    clock: Arc<RwLock<f32>>,
}

impl NoteCollection {
    pub fn new(clock: Arc<RwLock<f32>>) -> Self {
        Self { notes: vec![], clock }
    }

    pub fn count(&self) -> usize {
        self.notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    pub fn note_pressed(&mut self, note: Note, sequence_time: f32) {
        let existing_note = self.notes.iter_mut()
        .find(|n| n.scale_id == note.scale_id);

        match existing_note {
            Some(existing_note) => NoteCollection::refresh_same_note(existing_note, sequence_time),
            None => self.add_new_note(note, sequence_time)
        }
    }

    fn refresh_same_note(note: &mut Note, sequence_time: f32) {
        if note.time_deactivated > note.time_activated {
            note.time_activated = sequence_time;
            note.is_active = true;
        }
    }

    fn add_new_note(&mut self, mut note: Note, sequence_time: f32) {
        note.time_activated = sequence_time;
        self.notes.push(note);
    }

    pub fn note_released(&mut self, note: Note, sequence_time: f32) {
        let existing_note = self.notes.iter_mut()
        .find(|n| n.scale_id == note.scale_id);

        if let Some(existing_note) = existing_note {
            if existing_note.time_deactivated < existing_note.time_activated {
                existing_note.time_deactivated = sequence_time;
            }
        }
    }

    pub fn sum_note_samples(&mut self, instruments: &Vec<Box<dyn Instrument + Send>>) -> f32 {
        let mut next_sample: f32 = 0.0;

        for note in self.notes.iter_mut() {
            let filtered_instruments = instruments.iter()
            .filter(|i| i.get_channel() == note.channel)
            .collect::<Vec<&Box<dyn Instrument + Send>>>();

            filtered_instruments.iter().for_each(|e| {
                let sample = e.get_next_sample(*self.clock.read().unwrap(), &note);

                match sample {
                    Some(sample) => next_sample += sample,
                    None => if note.time_deactivated > note.time_activated { note.is_active = false }
                }
            });
        }

        next_sample
    }
}