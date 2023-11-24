use crate::sample_generator::note::Note;

pub enum InputEventType {
    Press,
    Release,
    Unknown
}

pub struct InputEventData {
    pub note: Note,
    pub time: f32,
    pub event: InputEventType,
}