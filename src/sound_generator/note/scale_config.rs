use rdev::Key;

use super::Note;

pub fn get_note_for_key(key: &Key) -> Option<i32> {
    match key {
        Key::KeyZ => Some(0),
        Key::KeyS => Some(1),
        Key::KeyX => Some(2),
        Key::KeyD => Some(3),
        Key::KeyC => Some(4),
        Key::KeyV => Some(5),
        Key::KeyG => Some(6),
        Key::KeyB => Some(7),
        Key::KeyH => Some(8),
        Key::KeyN => Some(9),
        Key::KeyJ => Some(10),
        Key::KeyM => Some(11),
        Key::Comma => Some(12),
        Key::KeyL => Some(13),
        Key::Dot => Some(14),
        Key::SemiColon => Some(15),
        Key::Slash => Some(16),
        _ => None,
    }
}

pub fn get_note_frequency(note: &Note) -> f32 {
    let base: f32 = 1.0594630943592952645618252949463;
    256.0 * base.powi(note.scale_id)
}