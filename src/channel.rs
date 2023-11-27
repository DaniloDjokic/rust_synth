pub struct Channel {
    channel_id: usize,
    beat_line: String,
}

impl Channel {
    pub fn new(channel_id: usize) -> Self {
        Self {
            channel_id,
            beat_line: String::from("")
        }
    }

    pub fn is_beat_active(&self, current_beat: usize) -> bool {
        self.beat_line.chars().nth(current_beat) == Some('X')
    }

    pub fn channel_id(&self) -> usize {
        self.channel_id
    }

    pub fn set_beats(&mut self, beat_line: String) {
        self.beat_line = beat_line
    }
}