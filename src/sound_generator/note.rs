use self::scale_config::ScaleId;
pub mod scale_config;

#[derive(PartialEq, Clone)]
pub struct Note {
    pub scale_id: ScaleId,
    pub octave: usize,
    pub time_activated: f32,
    pub time_deactivated: f32,
    pub is_active: bool,
    pub channel: usize,
}

impl Note {
    pub fn new(scale_id: ScaleId, time_activated: f32, time_deactivated: f32) -> Note {
        Self {
            scale_id,
            octave: 1,
            time_activated,
            time_deactivated,
            is_active: true,
            channel: 1
        }
    }

    pub fn set_is_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn get_frequency(&self) -> f32 {
        scale_config::get_note_frequency(&self.scale_id, self.octave)
    }
}