use std::sync::{Arc, RwLock};
use crate::sample_generator::note::Note;
use super::{channel::Channel, clock::{real_time_clock::RealTimeClock, Clock}};

pub struct Sequencer {
    proc_clock: Arc<RwLock<f32>>,
    real_time_clock: Arc<RwLock<RealTimeClock>>,
    beat_time: f32,
    current_beat: usize,
    total_beats: usize,
    accumulator: f32,
    notes: Vec<Note>,
    channels: Vec<Channel>,
}

impl Sequencer {
    pub fn new(clock: &Clock, tempo: f32, beats: usize, sub_beats: usize) -> Self {
        Self {
            proc_clock: clock.proc_clock(),
            real_time_clock: clock.real_time_clock(),
            beat_time: (60.0 / tempo) / sub_beats as f32,
            current_beat: 0,
            total_beats: beats * sub_beats,
            accumulator: 0.0,
            notes: vec![],
            channels: vec![],
        }
    }

    pub fn add_instrument(&mut self, channel_id: usize, instrument_sequence: String, max_note_lifetime: Option<f32>) {
        let mut channel = Channel::new(channel_id, max_note_lifetime);
        channel.set_beats(instrument_sequence);
        self.channels.push(channel);
    }

    pub fn get_next_notes(&mut self) -> Vec<Note> {
        self.notes.clear();

        self.accumulator += self.real_time_clock.write().unwrap().real_time_elapsed();

        while self.accumulator >= self.beat_time {
            self.accumulator -= self.beat_time;
            self.current_beat += 1;

            if self.current_beat >= self.total_beats {
                self.current_beat = 0;
            }

            for channel in self.channels.iter() {
                if channel.is_beat_active(self.current_beat) {
                    self.notes.push(
                        Note { 
                            scale_id: 23, 
                            time_activated: Some(*self.proc_clock.read().unwrap()), 
                            time_deactivated: None, 
                            is_active: true, 
                            channel: channel.channel_id(),
                            max_lifetime: channel.max_note_lifetime()
                        }
                    )
                }
            }
        }

        self.notes.clone()
    }
}