pub mod scale_config;

#[derive(PartialEq, Clone, Debug)]
pub struct Note {
    pub scale_id : i32,
    pub time_activated: Option<f32>,
    pub time_deactivated: Option<f32>,
    pub is_active: bool,
    pub channel: usize,
}

impl Note {
    pub fn new(scale_id: i32, time_activated: Option<f32>, time_deactivated: Option<f32>, channel: usize) -> Note {
        Self {
            scale_id,
            time_activated,
            time_deactivated,
            is_active: true,
            channel: channel
        }
    }

    pub fn get_frequency(&self, octave: i32) -> f32 {
        scale_config::get_note_frequency(&self, octave)
    }

    pub fn get_base_frequency(&self) -> f32 {
        scale_config::get_note_frequency(&self, 1)
    }
}