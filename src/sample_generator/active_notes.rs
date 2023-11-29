use std::sync::{Arc, RwLock};

use crate::input::clock::proc_clock::ProcClock;

use super::{note::Note, instrument::Instrument};

pub struct ActiveNotes {
    notes: Vec<Note>,
    current_count: usize,
    clock: Arc<RwLock<ProcClock>>,
}

impl ActiveNotes {
    pub fn new(clock: Arc<RwLock<ProcClock>>) -> Self {
        Self { 
            notes: vec![],
            current_count: 0, 
            clock 
        }
    }

    pub fn has_len_change(&mut self) -> Option<usize> {
        let len = self.notes.len();

        if len != self.current_count {
            self.current_count = len;
            return Some(len);
        }

        None
    }

    pub fn note_pressed(&mut self, note: Note, sequence_time: f32) {
        let existing_note = self.notes.iter_mut()
        .find(|n| n.scale_id == note.scale_id && n.channel == note.channel);

        match existing_note {
            Some(existing_note) => ActiveNotes::refresh_same_note(existing_note, sequence_time),
            None => self.add_new_note(note, sequence_time)
        }
    }

    fn refresh_same_note(note: &mut Note, sequence_time: f32) {
        if note.time_deactivated > note.time_activated {
            note.time_activated = Some(sequence_time);
            note.is_active = true;
        }
    }

    fn add_new_note(&mut self, mut note: Note, sequence_time: f32) {
        note.time_activated = Some(sequence_time);
        self.notes.push(note);
    }

    pub fn note_released(&mut self, note: Note, sequence_time: f32) {
        let existing_note = self.notes.iter_mut()
        .find(|n| n.scale_id == note.scale_id);

        if let Some(existing_note) = existing_note {
            if existing_note.time_deactivated < existing_note.time_activated {
                existing_note.time_deactivated = Some(sequence_time);
            }
        }
    }

    pub fn sum_note_samples(&mut self, instruments: &Vec<Arc<dyn Instrument + Send + Sync>>) -> f32 {
        let mut next_sample: f32 = 0.0;

        for note in self.notes.iter_mut() {
            let filtered_instruments = instruments.iter()
            .filter(|i| i.get_channel() == note.channel)
            .collect::<Vec<&Arc<dyn Instrument + Send + Sync>>>();

            filtered_instruments.iter().for_each(|e| {
                let time = self.clock.read().unwrap().get_time();
                let sample = e.get_next_sample(time, note);

                match sample {
                    Some(sample) => next_sample += sample,
                    None => {
                        if note.time_deactivated > note.time_activated { 
                            note.is_active = false
                        }

                        if let Some(lifetime) = note.max_lifetime {
                            if let Some(time_activated) = note.time_activated {
                                if time - time_activated >= lifetime {
                                    note.is_active = false;
                                }
                            }
                        }
                    } 
                }
            });
        }

        self.notes.retain(|n| n.is_active);

        next_sample
    }
}