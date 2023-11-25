use std::sync::Arc;

use crate::sample_generator::instrument::Instrument;

pub struct Channel {
    instrument: Arc<dyn Instrument>,
    channel_id: usize,
    beat_line: String,
}

impl Channel {
    pub fn new(instrument: Arc<dyn Instrument>, channel_id: usize) -> Self {
        Self {
            instrument,
            channel_id,
            beat_line: String::from("")
        }
    }

    pub fn is_beat_active(&self) -> bool {
        self.beat_line.eq_ignore_ascii_case("X")
    }

    pub fn channel_id(&self) -> usize {
        self.channel_id
    }

    pub fn set_beats(&mut self, beat_line: String) {
        self.beat_line = beat_line
    }
}