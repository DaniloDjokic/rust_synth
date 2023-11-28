pub struct Channel {
    channel_id: usize,
    beat_line: String,
    max_note_lifetime: Option<f32>,
}

impl Channel {
    pub fn new(channel_id: usize, max_note_lifetime: Option<f32>) -> Self {
        Self {
            channel_id,
            beat_line: String::from(""),
            max_note_lifetime,
        }
    }

    pub fn is_beat_active(&self, current_beat: usize) -> bool {
        self.beat_line.chars().nth(current_beat) == Some('X')
    }

    pub fn channel_id(&self) -> usize {
        self.channel_id
    }

    pub fn max_note_lifetime(&self) -> Option<f32> {
        self.max_note_lifetime
    }

    pub fn set_beats(&mut self, beat_line: String) {
        self.beat_line = beat_line
    }
}