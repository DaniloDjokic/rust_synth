use rdev::Key;

use super::Note;

pub fn get_scale_id_for_key(key: &Key) -> Option<u32> {
    match key {
        Key::KeyQ => Some(0), //C
        Key::Num2 => Some(1), //C#
        Key::KeyW => Some(2), //D
        Key::Num3 => Some(3), //D#
        Key::KeyE => Some(4), //E
        Key::KeyR => Some(5), //F
        Key::Num5 => Some(6), //F#
        Key::KeyT => Some(7), //G
        Key::Num6 => Some(8), //G#
        Key::KeyY => Some(9), //A
        Key::Num7 => Some(10), //A#
        Key::KeyU => Some(11), //B
        Key::KeyI => Some(12), //C
        Key::Num9 => Some(13), //C#
        Key::KeyO => Some(14), //D
        Key::Num0 => Some(15), //D#
        Key::KeyP => Some(16), //E

        Key::KeyZ => Some(12),
        Key::KeyS => Some(13),
        Key::KeyX => Some(14),
        Key::KeyD => Some(15),
        Key::KeyC => Some(16),
        Key::KeyV => Some(17),
        Key::KeyG => Some(18),
        Key::KeyB => Some(19),
        Key::KeyH => Some(20),
        Key::KeyN => Some(21),
        Key::KeyJ => Some(22),
        Key::KeyM => Some(23),
        Key::Comma => Some(24),
        Key::KeyL => Some(25),
        Key::Dot => Some(26),
        Key::SemiColon => Some(27),
        Key::Slash => Some(28),
        _ => None,
    }
}

pub fn get_note_frequency(note: &Note, octave : u32) -> f32 {
    let scale_id = note.scale_id + (octave * 12);
    let base: f32 = 1.0594630943592952645618252949463;
    
    8.0 * base.powi(scale_id as i32)
}