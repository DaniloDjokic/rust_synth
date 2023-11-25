use std::sync::Arc;

use crate::{sample_generator::{note::Note, instrument::Instrument}, input::clock::Clock, channel::Channel};

pub struct Sequencer {
    clock: Arc<Clock>,
    tempo: f32,
    beats: usize,
    sub_beats: usize,
    beat_time: f32,
    current_beat: usize,
    total_beats: usize,
    accumulator: f32,
    notes: Vec<Note>,
    channels: Vec<Channel>,
}

impl Sequencer {
    pub fn new(clock: Arc<Clock>, tempo: f32, beats: usize, sub_beats: usize) -> Self {
        Self {
            clock,
            tempo,
            beats,
            sub_beats,
            beat_time: (60.0 / tempo) / sub_beats as f32,
            current_beat: 0,
            total_beats: beats * sub_beats,
            accumulator: 0.0,
            notes: vec![],
            channels: vec![],
        }
    }

    pub fn add_instrument(&mut self, instrument: Arc<dyn Instrument>, instrument_sequence: String) {
        let channel_id = instrument.get_channel();
        let mut channel = Channel::new(instrument, channel_id);
        
        channel.set_beats(instrument_sequence);
        self.channels.push(channel);
    }

    pub fn get_next_notes(&mut self) -> Vec<Note> {
        self.notes.clear();

        self.accumulator += self.clock.real_time_passed();

        while self.accumulator >= self.beat_time {
            self.accumulator -= self.beat_time;
            self.current_beat += 1;

            if self.current_beat >= self.total_beats {
                self.current_beat = 0;
            }

            for channel in self.channels.iter() {
                if channel.is_beat_active() {
                    self.notes.push(
                        Note { 
                            scale_id: 64, 
                            time_activated: Some(*self.clock.proc_clock().read().unwrap()), 
                            time_deactivated: None, 
                            is_active: true, 
                            channel: channel.channel_id() 
                        }
                    )
                }
            }
        }

        self.notes.clone()
    }
}