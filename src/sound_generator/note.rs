use self::scale_config::ScaleId;
pub mod scale_config;

#[derive(PartialEq, Clone, Debug)]
pub struct Note {
    pub scale_id: ScaleId,
    pub time_activated: f32,
    pub time_deactivated: f32,
    pub is_active: bool,
    pub channel: usize,
}

impl Note {
    pub fn new(scale_id: ScaleId, time_activated: f32, time_deactivated: f32) -> Note {
        Self {
            scale_id,
            time_activated,
            time_deactivated,
            is_active: true,
            channel: 1
        }
    }

    pub fn get_frequency(&self, octave: usize) -> f32 {
        scale_config::get_note_frequency(&self.scale_id, octave)
    }
}